use keystone_cfg::*;
use keystone_hal::vm::{PageManager, PageTableEntry, PhysAddr, RootPageTable, VirtAddr};
use riscv::register::satp;

struct LinearMemoryManager {
    phys2virt_offset: usize,
    alloc_ptr: PhysAddr,
    alloc_end: PhysAddr,
}

impl PageManager for LinearMemoryManager {
    fn alloc_physical_page(&mut self) -> PhysAddr {
        let result = self.alloc_ptr;
        self.alloc_ptr.0 += PAGE_SIZE;
        assert!(self.alloc_ptr.0 <= self.alloc_end.0, "page table overflow");
        result
    }

    unsafe fn map_physical_page(&mut self, phys: PhysAddr) -> *mut () {
        (phys.0 + self.phys2virt_offset) as _
    }
}

extern "C" {
    fn rt_page_table();
}

// according to RISC-V calling convention, the two numbers will be passed in
// registers a0 an a1
#[repr(C)]
struct FreeMem(usize, usize);

#[no_mangle]
unsafe extern "C" fn vm_init(
    _sbiret: usize, // return value from the SBI, usually 0
    epm_base: usize,
    epm_size: usize,
    runtime_phys: usize,
    user_phys: usize,
    free_phys: usize,
    utm_phys: usize,
    utm_size: usize,
) -> FreeMem {
    // create root page table
    let phys2virt_offset = KERNEL_BASE - runtime_phys;
    let rpt_virt = rt_page_table as *const () as usize;
    let rpt_phys = rpt_virt - phys2virt_offset;
    let mem_mgr = LinearMemoryManager {
        phys2virt_offset,
        alloc_ptr: PhysAddr(rpt_phys),
        alloc_end: PhysAddr(rpt_phys + KERNEL_PAGE_TABLE_PREALLOC),
    };
    let mut root_page_table = RootPageTable::allocate_from(mem_mgr);

    // map kernel code and data
    let kernel_pages = (user_phys - runtime_phys) >> 12;
    for i in 0..kernel_pages {
        root_page_table.map_4k(
            VirtAddr(KERNEL_BASE + (i << 12)),
            PageTableEntry::for_phys(PhysAddr(runtime_phys + (i << 12))).make_rwx(),
        );
    }
    // map user code & stack
    for i in 0..((free_phys - user_phys) >> 12) {
        root_page_table.map_4k(
            VirtAddr(USER_BASE + (i << 12)),
            PageTableEntry::for_phys(PhysAddr(user_phys + (i << 12)))
                .make_user()
                .make_rwx(),
        );
    }
    // map EPM mirror
    // sadly, `epm_base` is not always aligned to 2 MB boundary, so we have to
    // use map_4k again
    for i in 0..(epm_size >> 12) {
        root_page_table.map_4k(
            VirtAddr(KERNEL_MIRROR_BASE + (i << 12)),
            PageTableEntry::for_phys(PhysAddr(epm_base + (i << 12))).make_rwx(),
        );
    }
    // map untrusted memory
    for i in 0..(utm_size >> 12) {
        root_page_table.map_4k(
            VirtAddr(KERNEL_UTM_BASE + (i << 12)),
            PageTableEntry::for_phys(PhysAddr(utm_phys + (i << 12))).make_rwx(),
        );
    }

    // write to satp
    satp::set(satp::Mode::Sv39, 0, rpt_phys >> 12);

    FreeMem(
        free_phys - epm_base + KERNEL_MIRROR_BASE, // virtual base address of free memory
        epm_base + epm_size - free_phys,           // size of free memory
    )
}
