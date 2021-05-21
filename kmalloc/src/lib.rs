#![no_std]
#![feature(alloc_error_handler, const_fn_trait_bound)]

extern crate alloc;

use core::alloc::GlobalAlloc;

mod linked_list;
mod non_reentrant;

pub use linked_list::LockedLinkedListHeap;
pub use non_reentrant::LockedHeap;

pub trait Kmalloc: GlobalAlloc + Sync {
    // const unsafe fn uninit() -> Self;
    unsafe fn init(&self, start: *mut u8, size: usize);
    fn print_stats(&self, writer: &mut impl core::fmt::Write) -> core::fmt::Result;
}

pub unsafe trait KmallocUnlocked {
    // const unsafe fn uninit() -> Self;
    unsafe fn init(&mut self, start: *mut u8, size: usize);
    fn print_stats(&mut self, writer: &mut impl core::fmt::Write) -> core::fmt::Result;
    unsafe fn alloc(&mut self, layout: core::alloc::Layout) -> *mut u8;
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: core::alloc::Layout);
}

#[alloc_error_handler]
fn alloc_error_handler(_layout: alloc::alloc::Layout) -> ! {
    panic!("out of memory")
}
