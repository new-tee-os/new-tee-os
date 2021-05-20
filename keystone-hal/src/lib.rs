#![cfg_attr(not(test), no_std)]
// `asm` is used in the `sbi` module only
#![cfg_attr(feature = "rt", feature(asm))]

pub mod edge;
pub mod riscv;
#[cfg(feature = "rt")]
pub mod rt;
#[cfg(feature = "rt")]
pub mod sbi;
