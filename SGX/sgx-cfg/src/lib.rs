#![no_std]

pub const EDGE_MEM_BASE:usize=0x1000;

pub const EDGE_BUFFER_SIZE:usize=0x1000;

pub static mut HEAP_START:usize=0;
pub static mut HEAP_SIZE:usize=0;