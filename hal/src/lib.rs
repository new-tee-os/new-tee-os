#![cfg_attr(not(test), no_std)]
#![feature(asm, const_btree_new, global_asm, map_first_last)]

extern crate alloc;

/// Architecture-specific APIs.
pub mod arch;

/// Edge call APIs.
pub mod edge;

/// Architecture-specific data structures and implementations.
/// Private to this crate.
mod sys;

#[cfg(feature = "kernel")]
/// Kernel mode specific items (e.g. `copy_from_user`).
mod kernel;
#[cfg(feature = "kernel")]
pub use kernel::*;

// expose platform-specific configuration items
pub use sys::cfg;
