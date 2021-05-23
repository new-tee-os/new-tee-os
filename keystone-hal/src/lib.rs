#![cfg_attr(not(test), no_std)]
// `asm` is used in the `sbi` module only
#![cfg_attr(feature = "rt", feature(asm))]

pub mod edge;
#[cfg(feature = "rt")]
pub mod mem;
#[cfg(feature = "rt")]
pub mod rt;
pub mod vm;
#[cfg(feature = "rt")]
pub use rt::{edge_con, sbi};

pub use keystone_cfg as cfg;
