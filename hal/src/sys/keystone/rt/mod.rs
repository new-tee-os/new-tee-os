/// Implementations of edge caller.
pub mod edge;

/// Implementations of `copy_from_user` and `copy_to_user`.
pub mod mem;

/// Implementations of process data structures and operations.
pub mod task;

use crate::arch::keystone::sbi;

pub fn exit_enclave(retval: usize) {
    sbi::exit_enclave(retval);
}
