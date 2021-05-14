use std::{collections::HashMap, fs::File, io::Read};

mod keystone;
mod riscv;

use keystone::{EnclaveStatus, KeystoneDev};
use keystone_hal::edge::EdgeMemory;
use riscv::{PageManager, PageTableEntry, PhysAddr, RootPageTable, VirtAddr};

const KERNEL_BASE: usize = 0xffffffffc0000000;

/// The enclave page manager, which supports linear page allocation for the page table.
struct EnclaveMemoryManager<'a> {
    enclave: &'a KeystoneDev,
    phys_base: PhysAddr,
    alloc_ptr: PhysAddr,
    memory_map: HashMap<PhysAddr, *mut ()>,
}

impl<'a> EnclaveMemoryManager<'a> {
    pub fn new(enclave: &'a KeystoneDev, phys_base: PhysAddr) -> EnclaveMemoryManager<'a> {
        EnclaveMemoryManager {
            enclave,
            phys_base,
            alloc_ptr: phys_base,
            memory_map: HashMap::new(),
        }
    }
}

impl PageManager for EnclaveMemoryManager<'_> {
    fn alloc_physical_page(&mut self) -> PhysAddr {
        let result = self.alloc_ptr;
        self.alloc_ptr.0 += 0x1000;
        result
    }

    unsafe fn map_physical_page(&mut self, phys: PhysAddr) -> *mut () {
        assert_eq!(phys.page_offset(), 0);
        if let Some(&entry) = self.memory_map.get(&phys) {
            entry
        } else {
            let mapped = self
                .enclave
                .map_mem(phys.0 - self.phys_base.0, 0x1000)
                .expect("failed to map enclave memory");
            self.memory_map.insert(phys, mapped);
            //println!("Map +{:#X} -> {:?}", phys.0 - self.phys_base.0, mapped);
            mapped
        }
    }
}

impl Drop for EnclaveMemoryManager<'_> {
    fn drop(&mut self) {
        // release all mappings
        for (_, ptr) in self.memory_map.drain() {
            unsafe {
                self.enclave
                    .unmap_mem(ptr, 0x1000)
                    .expect("failed to unmap enclave memory");
            }
        }
    }
}

/// Copy one page of the host OS's memory to the enclave's EPM.
///
/// `dest_offset` must be aligned to a 4 kB page boundary.
fn copy_to_enclave(enclave: &KeystoneDev, src: &[u8], dest_offset: usize) {
    assert_eq!(dest_offset & 0xFFF, 0);
    unsafe {
        let mem = enclave
            .map_mem(dest_offset, 0x1000)
            .expect("failed to map enclave memory");
        //println!("Map +{:#X} -> {:?}", dest_offset, mem);
        let dest = std::slice::from_raw_parts_mut(mem as _, 0x1000);
        dest.copy_from_slice(src);
        enclave
            .unmap_mem(mem, 0x1000)
            .expect("failed to unmap enclave memory");
    }
}

unsafe fn handle_edge_call(edge_mem: *mut EdgeMemory) {
    use keystone_hal::edge::EdgeCallReq::{self, *};
    use std::convert::TryFrom;

    let edge_mem = &mut *edge_mem;
    match EdgeCallReq::try_from(edge_mem.req).unwrap_or(EdgeCallInvalid) {
        EdgeCallPrint => {
            print!(
                "{}",
                std::str::from_utf8(edge_mem.read_buffer())
                    .expect("the enclave tries to print an invalid UTF-8 string")
            );
            // return 42
            edge_mem.req = 42;
        }
        _ => {
            println!("Warning: invalid edge call number, ignoring");
        }
    }
}

fn main() {
    let mut kernel_file = File::open("keystone-rt.bin").expect("failed to open keystone-rt.bin");
    // keystone-rt.bin contains everything until _end
    let kernel_mem_size = kernel_file
        .metadata()
        .expect("failed to stat keystone-rt.bin")
        .len() as usize;
    let epm_size = 0x30_000;

    let mut enclave = KeystoneDev::open().expect("failed to open Keystone device");
    enclave
        .create(epm_size >> 12)
        .expect("failed to create enclave");
    let epm_phys_base = enclave.phys_addr();
    let utm_phys_base = enclave
        .init_utm(0x1000)
        .expect("failed to create untrusted memory (UTM)");
    let kernel_phys_base = epm_phys_base + 0x10_000;
    let user_base = kernel_phys_base + kernel_mem_size;

    // load kernel to the EPM
    let mut dest_offset = kernel_phys_base - epm_phys_base;
    loop {
        let mut buf = [0; 0x1000];
        let bytes_read = kernel_file
            .read(&mut buf)
            .expect("failed to read keystone-rt.bin");
        if bytes_read == 0 {
            break;
        }
        copy_to_enclave(&enclave, &buf, dest_offset);
        dest_offset += 0x1000;
    }

    // load user program
    {
        let mut user_program = File::open("user.bin").expect("failed to open user.bin");
        let mut buf = [0; 0x1000];
        let bytes_read = user_program
            .read(&mut buf)
            .expect("failed to read user.bin");
        copy_to_enclave(&enclave, &buf, user_base - epm_phys_base);
    }

    // create page tables
    unsafe {
        let total_pages = kernel_mem_size >> 12;
        let mem_mgr = EnclaveMemoryManager::new(&enclave, PhysAddr(epm_phys_base));
        let mut root_page_table = RootPageTable::allocate_from(mem_mgr);
        for i in 0..total_pages {
            let phys = PhysAddr(kernel_phys_base + (i << 12));
            let virt = VirtAddr(KERNEL_BASE + (i << 12));
            root_page_table.map_4k(virt, PageTableEntry::for_phys(phys).make_rwx());
        }
        // map user memory
        root_page_table.map_4k(
            VirtAddr(0x40_0000),
            PageTableEntry::for_phys(PhysAddr(user_base))
                .make_user()
                .make_rwx(),
        );
        root_page_table.map_4k(
            VirtAddr(0x40_1000),
            PageTableEntry::for_phys(PhysAddr(user_base + 0x1000))
                .make_user()
                .make_rwx(),
        );
        // map untrusted memory
        root_page_table.map_4k(
            VirtAddr(KERNEL_BASE + (total_pages << 12)),
            PageTableEntry::for_phys(PhysAddr(utm_phys_base)).make_rwx(),
        );
    }

    let phys_free = kernel_phys_base + kernel_mem_size + 0x2000;
    println!("Base: {:#X}", epm_phys_base);
    println!("Krnl: {:#X}", kernel_phys_base);
    println!("User: {:#X}", user_base);
    println!("End:  {:#X}", epm_phys_base + epm_size);
    println!("UTM:  {:#X}", utm_phys_base);

    enclave
        .finalize(
            kernel_phys_base,
            user_base,
            phys_free,
            keystone::RuntimeParams {
                runtime_entry: KERNEL_BASE,
                user_entry: 0,
                untrusted_ptr: utm_phys_base,
                untrusted_size: 0x1000,
            },
        )
        .expect("failed to finalize enclave");

    let edge_mem = unsafe { enclave.map_mem(0, 0x1000) }.expect("failed to map untrusted memory")
        as *mut EdgeMemory;

    let mut status = enclave.run().expect("failed to run enclave");
    loop {
        match status {
            EnclaveStatus::Done(code) => {
                println!("Enclave exited with status {}", code);
                break;
            }
            EnclaveStatus::Interrupted => (),
            EnclaveStatus::EdgeCallHost => {
                //println!("Edge call requested");
                unsafe {
                    handle_edge_call(edge_mem);
                }
            }
            _ => panic!("Unexpected enclave status: {:?}", status),
        }
        status = enclave.resume().expect("failed to resume enclave");
    }

    enclave.destroy().expect("failed to destroy enclave");
}
