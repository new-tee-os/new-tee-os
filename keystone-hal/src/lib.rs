#![cfg_attr(not(test), no_std)]
// `asm` is used in the `sbi` module only
#![cfg_attr(feature = "rt", feature(asm))]

pub mod edge;
pub mod edge_syscall;
#[cfg(feature = "rt")]
pub mod mem;
#[cfg(feature = "rt")]
pub mod rt;
pub mod vm;
#[cfg(feature = "rt")]
pub use rt::{edge_con, sbi, EDGE_MEM_BASE};

pub use keystone_cfg as cfg;

#[cfg(feature = "rt")]
pub unsafe fn edge_call() {
    sbi::stop_enclave(sbi::STOP_EDGE_CALL_HOST);
}
