use core::ptr::NonNull;

use crate::KmallocUnlocked;
use linked_list_allocator::Heap;

pub type LockedLinkedListHeap = crate::LockedHeap<Heap>;

impl LockedLinkedListHeap {
    pub const unsafe fn uninit() -> LockedLinkedListHeap {
        crate::LockedHeap::new(Heap::empty())
    }
}

unsafe impl KmallocUnlocked for Heap {
    unsafe fn init(&mut self, start: *mut u8, size: usize) {
        Heap::init(self, start as usize, size);
    }

    fn print_stats(&mut self, writer: &mut impl core::fmt::Write) -> core::fmt::Result {
        write!(writer, "Used: {}\nFree: {}\n", self.used(), self.free())
    }

    unsafe fn alloc(&mut self, layout: core::alloc::Layout) -> *mut u8 {
        self.allocate_first_fit(layout)
            .map(|ptr| ptr.as_ptr())
            .unwrap_or(core::ptr::null_mut())
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: core::alloc::Layout) {
        if let Some(ptr) = NonNull::new(ptr) {
            self.deallocate(ptr, layout);
        }
    }
}
