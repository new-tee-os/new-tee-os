#![no_std]


pub mod edge;

pub use sgx_cfg as cfg;
use edge::*;

pub static EDGE_MEM_BASE:EdgeMemory=EdgeMemory{
    buffer: core::slice::from_raw_parts_mut(0 as *mut u8, 0x1),
    len:0,
};

