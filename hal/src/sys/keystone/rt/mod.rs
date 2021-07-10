/// Implementations of edge caller.
mod edge;

/// Implementations of `copy_from_user` and `copy_to_user`.
pub mod mem;

// expose the global edge caller from `edge` module
pub use edge::GLOBAL_EDGE_CALLER;

use crate::arch::keystone::sbi;

pub fn edge_call(retval: usize) {
    sbi::exit_enclave(retval);
}

pub fn exit_enclave(retval: usize) {
    sbi::exit_enclave(retval);
}
