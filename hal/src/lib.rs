#![cfg_attr(not(test), no_std)]
// `asm!` is used in the `sbi` module only
#![cfg_attr(feature = "keystone-rt", feature(asm))]

pub mod arch;
pub mod edge;
#[cfg(feature = "kernel")]
pub mod mem;
mod sys;

pub use sys::cfg;
#[cfg(feature = "kernel")]
pub use sys::{edge_call, exit_enclave};
