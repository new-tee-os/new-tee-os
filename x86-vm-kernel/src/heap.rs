use bootloader::{
    boot_info::{MemoryRegion, MemoryRegionKind},
    BootInfo,
};
use hal::dbg_println;
use kmalloc::{Kmalloc, LockedLinkedListHeap};

#[global_allocator]
static HEAP: LockedLinkedListHeap = unsafe { LockedLinkedListHeap::uninit() };

pub fn init(boot_info: &BootInfo) {
    let mut heap_region: Option<&MemoryRegion> = None;
    for memory_region in boot_info.memory_regions.iter() {
        if memory_region.kind == MemoryRegionKind::Usable {
            if let Some(_) = heap_region {
                dbg_println!("Warning: ignoring extra memory region {:?}", memory_region);
            } else {
                heap_region = Some(memory_region);
            }
        }
    }

    let heap_region = heap_region.unwrap();
    dbg_println!("Creating heap at {:?}", heap_region);
    unsafe {
        HEAP.init(
            (crate::cfg::KERNEL_MIRROR_BASE + heap_region.start as usize) as *mut _,
            (heap_region.end - heap_region.start) as usize,
        );
    }
}
