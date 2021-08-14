use core::alloc::Layout;

use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

pub struct HeapFrameAlloc;

pub const LAYOUT_4K_PAGE: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x1000) };
pub const MIRROR_BASE_VIRT: VirtAddr =
    VirtAddr::new_truncate(crate::cfg::KERNEL_MIRROR_BASE as u64);

unsafe impl FrameAllocator<Size4KiB> for HeapFrameAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let virt_addr = VirtAddr::from_ptr(unsafe { alloc::alloc::alloc(LAYOUT_4K_PAGE) });
        if virt_addr.is_null() {
            return None;
        }
        let phys_addr = PhysAddr::new(virt_addr - MIRROR_BASE_VIRT);
        Some(PhysFrame::from_start_address(phys_addr).unwrap())
    }
}
