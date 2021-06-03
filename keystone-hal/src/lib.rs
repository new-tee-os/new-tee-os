#![cfg_attr(not(test), no_std)]
// `asm!` is used in the `sbi` module only
#![cfg_attr(feature = "rt", feature(asm))]

pub use edge_lib::*;

pub mod cfg;
#[cfg(feature = "rt")]
pub mod rt;
pub mod vm;
#[cfg(feature = "rt")]
pub use rt::*;
