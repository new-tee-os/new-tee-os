#![cfg_attr(not(test), no_std)]
// `asm` is used in the `sbi` module only
#![cfg_attr(feature = "rt", feature(asm))]

pub mod edge;
pub mod mem;
pub mod riscv;
#[cfg(feature = "rt")]
pub mod rt;
#[cfg(feature = "rt")]
pub use rt::{edge_con, sbi};

pub use keystone_cfg as cfg;
