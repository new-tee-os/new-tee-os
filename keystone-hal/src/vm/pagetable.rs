use super::{PhysAddr, VirtAddr};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct PageTableEntry(pub u64);

#[derive(Clone, Copy)]
pub struct PageTable(pub *mut PageTableEntry);

pub trait PageManager {
    fn alloc_physical_page(&mut self) -> PhysAddr;
    unsafe fn map_physical_page(&mut self, phys: PhysAddr) -> *mut ();
    //unsafe fn unmap_physical_page(&mut self, phys: PhysAddr);
}

#[derive(Clone, Copy)]
pub struct RootPageTable<M: PageManager> {
    inner: PageTable,
    manager: M,
}

pub const PTE_VALID: u64 = 1;
pub const PTE_RWX: u64 = 0xE;
pub const PTE_USER: u64 = 16;
pub const MODE_SV39: u64 = 8;

impl PageTableEntry {
    #[inline]
    pub fn for_ppn(ppn: usize) -> PageTableEntry {
        PageTableEntry(((ppn as u64) << 10) | PTE_VALID)
    }

    #[inline]
    pub fn for_phys(phys: PhysAddr) -> PageTableEntry {
        assert_eq!(phys.page_offset(), 0);
        PageTableEntry::for_ppn(phys.ppn())
    }

    #[inline]
    pub fn is_valid(self) -> bool {
        (self.0 & PTE_VALID) != 0
    }

    #[inline]
    pub fn is_leaf(self) -> bool {
        (self.0 & PTE_RWX) != 0
    }

    #[inline]
    pub fn make_user(mut self) -> Self {
        self.0 |= PTE_USER;
        self
    }

    #[inline]
    pub fn make_rwx(mut self) -> Self {
        self.0 |= PTE_RWX;
        self
    }

    #[inline]
    pub fn ppn(self) -> usize {
        ((self.0 >> 10) & 0xFFFFFFFFFFF) as usize
    }

    #[inline]
    pub fn descending_page(self) -> PhysAddr {
        PhysAddr::from_ppn_offset(self.ppn(), 0)
    }

    #[inline]
    pub unsafe fn descending_page_table<M: PageManager>(self, manager: &mut M) -> PageTable {
        assert!(self.is_valid() && !self.is_leaf());
        let pt_addr_phys: PhysAddr = self.descending_page();
        let pt_ptr = manager.map_physical_page(pt_addr_phys) as *mut PageTableEntry;
        PageTable(pt_ptr)
    }
}

impl PageTable {
    #[inline]
    unsafe fn entry(self, index: usize) -> *mut PageTableEntry {
        assert!(index < 512);
        self.0.offset(index as isize)
    }

    #[inline]
    unsafe fn clear(self) -> Self {
        let slice = core::slice::from_raw_parts_mut(self.0, 512);
        slice.fill(PageTableEntry(0));
        self
    }
}

impl<M: PageManager> RootPageTable<M> {
    pub fn new(ptr: *mut (), manager: M) -> RootPageTable<M> {
        RootPageTable {
            inner: PageTable(ptr as *mut _),
            manager,
        }
    }

    pub unsafe fn new_zeroed(ptr: *mut (), manager: M) -> RootPageTable<M> {
        let inner = PageTable(ptr as *mut _);
        inner.clear();
        RootPageTable { inner, manager }
    }

    pub unsafe fn allocate_from(mut manager: M) -> RootPageTable<M> {
        let rpt_phys_addr = manager.alloc_physical_page();
        let rpt_virt_addr = manager.map_physical_page(rpt_phys_addr);
        Self::new_zeroed(rpt_virt_addr, manager)
    }

    unsafe fn alloc_page_table(&mut self) -> (PageTable, PhysAddr) {
        let page_addr: PhysAddr = self.manager.alloc_physical_page();
        let pt = PageTable(self.manager.map_physical_page(page_addr) as *mut _).clear();
        (pt, page_addr)
    }

    pub unsafe fn lookup_1g(&mut self, addr: VirtAddr) -> PageTable {
        let pte2_ptr = self.inner.entry(addr.vpn2());
        let pte2 = pte2_ptr.read();
        if !pte2.is_valid() {
            let (pt1, pt1_addr) = self.alloc_page_table();
            pte2_ptr.write(PageTableEntry::for_phys(pt1_addr));
            pt1
        } else {
            assert!(!pte2.is_leaf());
            pte2.descending_page_table(&mut self.manager)
        }
    }

    pub unsafe fn lookup_2m(&mut self, addr: VirtAddr) -> PageTable {
        let pt2 = self.lookup_1g(addr);
        let pte1_ptr = pt2.entry(addr.vpn1());
        let pte1 = pte1_ptr.read();
        if !pte1.is_valid() {
            let (pt0, pt0_addr) = self.alloc_page_table();
            pte1_ptr.write(PageTableEntry::for_phys(pt0_addr));
            pt0
        } else {
            assert!(!pte1.is_leaf());
            pte1.descending_page_table(&mut self.manager)
        }
    }

    pub unsafe fn map_2m(&mut self, src: VirtAddr, pte: PageTableEntry) {
        assert!(src.vpn0() == 0 && src.page_offset() == 0);
        let pt2 = self.lookup_1g(src);
        pt2.entry(src.vpn1()).write(pte);
    }

    pub unsafe fn map_4k(&mut self, src: VirtAddr, pte: PageTableEntry) {
        assert!(src.page_offset() == 0);
        let pt1 = self.lookup_2m(src);
        pt1.entry(src.vpn0()).write(pte);
    }

    pub fn into_manager(self) -> M {
        self.manager
    }
}
