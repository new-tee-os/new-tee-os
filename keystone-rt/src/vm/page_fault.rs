use alloc::alloc::{alloc, Layout};

use hal::arch::keystone::vm::*;
use hal::cfg::*;
use log::debug;

// SAFETY: already checked
static ALLOC_LAYOUT_PAGE: Layout =
    unsafe { Layout::from_size_align_unchecked(PAGE_SIZE, PAGE_SIZE) };

#[derive(Clone, Copy)]
pub struct HeapPageManager {
    epm_base: usize,
}

impl HeapPageManager {
    pub fn new() -> HeapPageManager {
        HeapPageManager {
            epm_base: *crate::EPM_PHYS.get().unwrap(),
        }
    }

    pub fn phys2virt(&self, phys: PhysAddr) -> VirtAddr {
        VirtAddr(phys.0 - self.epm_base + KERNEL_MIRROR_BASE)
    }

    pub fn virt2phys(&self, virt: VirtAddr) -> PhysAddr {
        PhysAddr(virt.0 - KERNEL_MIRROR_BASE + self.epm_base)
    }
}

impl PageManager for HeapPageManager {
    fn alloc_physical_page(&mut self) -> PhysAddr {
        let addr = alloc_page();
        debug!("Allocated {:#X} for page table", addr.0);
        self.virt2phys(addr)
    }

    unsafe fn map_physical_page(&mut self, phys: PhysAddr) -> *mut () {
        self.phys2virt(phys).as_mut_ptr()
    }
}

pub fn alloc_page() -> VirtAddr {
    let addr = unsafe { alloc(ALLOC_LAYOUT_PAGE) };
    VirtAddr::from_ptr(addr)
}

pub fn current_root_page_table() -> RootPageTable<HeapPageManager> {
    let rpt_phys = PhysAddr(riscv::register::satp::read().ppn() << 12);
    let mem_mgr = HeapPageManager::new();
    let rpt_virt = mem_mgr.phys2virt(rpt_phys);
    RootPageTable::new(rpt_virt.as_mut_ptr(), mem_mgr)
}

pub unsafe fn handle_page_fault_at(addr: usize) {
    let mut root_page_table = current_root_page_table();
    let new_page = alloc_page();
    root_page_table.map_4k(
        VirtAddr(addr & !0xFFF),
        PageTableEntry::for_phys(HeapPageManager::new().virt2phys(new_page))
            .make_user()
            .make_rwx(),
    );
}
