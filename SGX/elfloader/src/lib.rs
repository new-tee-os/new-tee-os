#![no_std]
#![feature(alloc_error_handler)]
//#![feature(const_fn)]
pub mod elfloader;
pub mod rsrvmalloc;

pub use rsrvmalloc::MYALLOCATOR as MYALLOCATOR;
