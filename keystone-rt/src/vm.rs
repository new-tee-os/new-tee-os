use keystone_cfg::*;
use keystone_hal::riscv::{PageManager, PageTableEntry, PhysAddr, RootPageTable, VirtAddr};
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
) {
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
    assert!(epm_size <= (2 << 20)); // 2 MB
    root_page_table.map_2m(
        VirtAddr(KERNEL_MIRROR_BASE),
        PageTableEntry::for_phys(PhysAddr(epm_base)).make_rwx(),
    );
    // map untrusted memory
    for i in 0..(utm_size >> 12) {
        root_page_table.map_4k(
            VirtAddr(KERNEL_UTM_BASE + (i << 12)),
            PageTableEntry::for_phys(PhysAddr(utm_phys + (i << 12))).make_rwx(),
        );
    }

    // write to satp
    satp::set(satp::Mode::Sv39, 0, rpt_phys >> 12);
}