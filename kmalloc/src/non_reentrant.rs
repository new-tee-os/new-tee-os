use core::{
    alloc::GlobalAlloc,
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::{Kmalloc, KmallocUnlocked};

pub struct LockedHeap<H: KmallocUnlocked> {
    inner: UnsafeCell<H>,
    locked: AtomicBool,
}

// SAFETY: LockedHeap is not reentrant, and thus safe to be shared between threads
unsafe impl<H: KmallocUnlocked> Sync for LockedHeap<H> {}

impl<H: KmallocUnlocked> LockedHeap<H> {
    pub const fn new(inner: H) -> LockedHeap<H> {
        LockedHeap {
            inner: UnsafeCell::new(inner),
            locked: AtomicBool::new(false),
        }
    }

    #[inline]
    fn do_locked<R>(&self, f: impl FnOnce(&mut H) -> R) -> R {
        let prev_locked = self.locked.swap(true, Ordering::Relaxed);
        if prev_locked {
            panic!("the allocator is non-reentrant");
        }
        let result = unsafe { f(&mut *self.inner.get()) };
        self.locked.store(false, Ordering::Relaxed);
        result
    }
}

unsafe impl<H: KmallocUnlocked> GlobalAlloc for LockedHeap<H> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.do_locked(|h| h.alloc(layout))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.do_locked(|h| h.dealloc(ptr, layout))
    }
}

impl<H: KmallocUnlocked> Kmalloc for LockedHeap<H> {
    unsafe fn init(&self, start: *mut u8, size: usize) {
        self.do_locked(|h| h.init(start, size))
    }

    fn print_stats(&self, writer: &mut impl core::fmt::Write) -> core::fmt::Result {
        self.do_locked(|h| h.print_stats(writer))
    }
}
