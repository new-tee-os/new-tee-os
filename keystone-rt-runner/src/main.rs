use std::{collections::HashMap, fs::File, io::Read};

mod edge_call;
mod edge_file;
mod keystone;

use keystone::{EnclaveStatus, KeystoneDev};
use keystone_cfg::*;
use keystone_hal::{
    vm::{PageManager, PageTableEntry, PhysAddr, RootPageTable, VirtAddr},
    EdgeMemory,
};

/// The enclave page manager, which supports linear page allocation for the page table.
struct EnclaveMemoryManager<'a> {
    enclave: &'a KeystoneDev,
    phys_base: PhysAddr,
    alloc_ptr: PhysAddr,
    alloc_end: PhysAddr,
    memory_map: HashMap<PhysAddr, *mut ()>,
}

impl<'a> EnclaveMemoryManager<'a> {
    pub fn new(
        enclave: &'a KeystoneDev,
        phys_base: PhysAddr,
        alloc_end: PhysAddr,
    ) -> EnclaveMemoryManager<'a> {
        EnclaveMemoryManager {
            enclave,
            phys_base,
            alloc_ptr: phys_base,
            alloc_end,
            memory_map: HashMap::new(),
        }
    }
}

impl PageManager for EnclaveMemoryManager<'_> {
    fn alloc_physical_page(&mut self) -> PhysAddr {
        let result = self.alloc_ptr;
        self.alloc_ptr.0 += PAGE_SIZE;
        assert!(
            self.alloc_ptr.0 <= self.alloc_end.0,
            "bootstrap page table overflow"
        );
        result
    }

    unsafe fn map_physical_page(&mut self, phys: PhysAddr) -> *mut () {
        assert_eq!(phys.page_offset(), 0);
        if let Some(&entry) = self.memory_map.get(&phys) {
            entry
        } else {
            let mapped = self
                .enclave
                .map_mem(phys.0 - self.phys_base.0, PAGE_SIZE)
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
                    .unmap_mem(ptr, PAGE_SIZE)
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
            .map_mem(dest_offset, PAGE_SIZE)
            .expect("failed to map enclave memory");
        //println!("Map +{:#X} -> {:?}", dest_offset, mem);
        let dest = std::slice::from_raw_parts_mut(mem as _, PAGE_SIZE);
        dest.copy_from_slice(src);
        enclave
            .unmap_mem(mem, PAGE_SIZE)
            .expect("failed to unmap enclave memory");
    }
}

fn main() {
    let mut kernel_file = File::open("keystone-rt.bin").expect("failed to open keystone-rt.bin");
    // keystone-rt.bin contains everything until _end
    let kernel_mem_size = kernel_file
        .metadata()
        .expect("failed to stat keystone-rt.bin")
        .len() as usize;

    let mut enclave = KeystoneDev::open().expect("failed to open Keystone device");
    enclave
        .create(EPM_SIZE >> 12)
        .expect("failed to create enclave");
    let epm_phys_base = enclave.phys_addr();
    let utm_phys_base = enclave
        .init_utm(UTM_SIZE)
        .expect("failed to create untrusted memory (UTM)");
    let kernel_phys_base = epm_phys_base + KERNEL_EPM_OFFSET;

    // load kernel to the EPM
    let mut dest_offset = kernel_phys_base - epm_phys_base;
    loop {
        let mut buf = [0; PAGE_SIZE];
        let bytes_read = kernel_file
            .read(&mut buf)
            .expect("failed to read keystone-rt.bin");
        if bytes_read == 0 {
            break;
        }
        copy_to_enclave(&enclave, &buf, dest_offset);
        dest_offset += PAGE_SIZE;
    }

    // create page tables
    unsafe {
        let total_pages = kernel_mem_size >> 12;
        let mem_mgr = EnclaveMemoryManager::new(
            &enclave,
            PhysAddr(epm_phys_base),
            PhysAddr(epm_phys_base + KERNEL_EPM_OFFSET),
        );
        let mut root_page_table = RootPageTable::allocate_from(mem_mgr);
        for i in 0..total_pages {
            let phys = PhysAddr(kernel_phys_base + (i << 12));
            let virt = VirtAddr(KERNEL_BASE + (i << 12));
            root_page_table.map_4k(virt, PageTableEntry::for_phys(phys).make_rwx());
        }
    }

    let fake_user_phys_base = kernel_phys_base + kernel_mem_size;
    let phys_free = fake_user_phys_base;
    println!("Base: {:#X}", epm_phys_base);
    println!("Krnl: {:#X}", kernel_phys_base);
    println!("User: {:#X}", fake_user_phys_base);
    println!("Free: {:#X}", phys_free);
    println!("End:  {:#X}", epm_phys_base + EPM_SIZE);
    println!("UTM:  {:#X}", utm_phys_base);
    println!("-------------------------");

    enclave
        .finalize(
            kernel_phys_base,
            fake_user_phys_base,
            phys_free,
            keystone::RuntimeParams {
                runtime_entry: KERNEL_BASE,
                user_entry: 0,
                untrusted_ptr: utm_phys_base,
                untrusted_size: UTM_SIZE,
            },
        )
        .expect("failed to finalize enclave");

    let edge_mem = unsafe { enclave.map_mem(0, PAGE_SIZE) }.expect("failed to map untrusted memory")
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
                    edge_call::handle_edge_call(edge_mem);
                }
            }
            _ => panic!("Unexpected enclave status: {:?}", status),
        }
        status = enclave.resume().expect("failed to resume enclave");
    }

    enclave.destroy().expect("failed to destroy enclave");
}
