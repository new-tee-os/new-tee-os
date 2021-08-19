#![no_std]

pub mod edge;
pub use sgx_cfg as cfg;
use edge::*;

static mut BUF:[u8;2]=[0,0];

pub static mut EDGE_MEM_BASE:EdgeMemory=unsafe{
    EdgeMemory{
        buffer: &mut BUF,
        len:0,
    }
};
