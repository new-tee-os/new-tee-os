use crate::riscv::{PageTableEntry, RootPageTable};

use super::{PageManager, PhysAddr, VirtAddr};

#[repr(C, align(4096))]
#[derive(Clone)]
struct PageTable([u64; 512]);

struct MockManager(Vec<PageTable>, usize);

const KERNEL_BASE: usize = 0xFFFF_FFFF_C000_0000;

impl PageManager for MockManager {
    fn alloc_physical_page(&mut self) -> PhysAddr {
        let result = unsafe { self.0.as_mut_ptr().offset(self.1 as isize) };
        self.1 += 1;
        assert!(self.1 < self.0.len());
        PhysAddr(result as usize)
    }

    unsafe fn map_physical_page(&mut self, phys: PhysAddr) -> *mut () {
        phys.0 as *mut _
    }
}

#[test]
fn it_works() {
    let mut manager = MockManager(vec![PageTable([0; 512]); 4], 0);
    assert_eq!((manager.0.as_ptr() as usize) & 0xFFF, 0);
    let rpt_phys = manager.alloc_physical_page();
    let mut rpt = RootPageTable::new(unsafe { manager.map_physical_page(rpt_phys) }, manager);
    unsafe {
        rpt.map_2m(
            VirtAddr(KERNEL_BASE),
            PageTableEntry::for_ppn(42).make_rwx(),
        );
    }

    let manager = rpt.into_manager();
    assert_eq!(manager.1, 2);
    // internal node
    let addr = unsafe { manager.0.as_ptr().offset(1) } as usize;
    assert_eq!(manager.0[0].0[0x1FF], (addr >> 12 << 10) as u64 | 1);
    // leaf node
    assert_eq!(manager.0[1].0[0], (42 << 10) | 15);
}
