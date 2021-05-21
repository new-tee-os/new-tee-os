use core::{
    alloc::GlobalAlloc,
    cell::UnsafeCell,
    ptr::NonNull,
    sync::atomic::{AtomicBool, Ordering},
};

struct LeakySlabAllocState {
    ptr: NonNull<u8>,
    end: NonNull<u8>,
    total_alloc: usize,
    total_dealloc: usize,
}

pub struct LeakySlabAlloc {
    inner: UnsafeCell<LeakySlabAllocState>,
    locked: AtomicBool,
}

// SAFETY: LeakyAlloc can be called from multiple threads safely
unsafe impl Sync for LeakySlabAlloc {}

impl LeakySlabAllocState {
    pub const fn new() -> LeakySlabAllocState {
        // TODO: initial state for the allocator
        LeakySlabAllocState {
            ptr: NonNull::dangling(),
            end: NonNull::dangling(),
            total_alloc: 0,
            total_dealloc: 0,
        }
    }

    pub unsafe fn alloc_fallible(&mut self, layout: core::alloc::Layout) -> Option<NonNull<u8>> {
        let mut ptr = self.ptr.as_ptr();
        let align_offset = ptr.align_offset(layout.align());
        // UNSAFE!
        ptr = ptr.offset(align_offset as isize);

        let new_ptr = NonNull::new(ptr.offset(layout.size() as isize)).unwrap();
        if new_ptr > self.end {
            // out of memory!
            return None;
        }
        self.ptr = new_ptr;
        self.total_alloc += layout.size();

        NonNull::new(ptr)
    }

    pub unsafe fn alloc(&mut self, layout: core::alloc::Layout) -> *mut u8 {
        self.alloc_fallible(layout)
            .map(NonNull::as_ptr)
            .unwrap_or(core::ptr::null_mut())
    }

    pub unsafe fn dealloc(&mut self, _ptr: *mut u8, layout: core::alloc::Layout) {
        self.total_dealloc += layout.size();
    }
}

impl LeakySlabAlloc {
    #[inline]
    unsafe fn call_sync<F, V>(&self, f: F) -> V
    where
        F: FnOnce(&mut LeakySlabAllocState) -> V,
    {
        let locked = self.locked.swap(true, Ordering::Relaxed);
        if locked {
            panic!("the allocator is not reentrant");
        }
        let mut_self = self.inner.get();
        let result = f(&mut *mut_self);
        self.locked.store(false, Ordering::Relaxed);
        result
    }

    pub const fn new() -> LeakySlabAlloc {
        let state = LeakySlabAllocState::new();
        LeakySlabAlloc {
            inner: UnsafeCell::new(state),
            locked: AtomicBool::new(false),
        }
    }
}

unsafe impl GlobalAlloc for LeakySlabAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.call_sync(|state| state.alloc(layout))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.call_sync(|state| state.dealloc(ptr, layout))
    }
}
