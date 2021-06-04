mod edge;
pub mod mem;

pub use edge::GLOBAL_EDGE_CALLER;

use crate::arch::keystone::sbi;

pub fn edge_call(retval: usize) {
    sbi::exit_enclave(retval);
}

pub fn exit_enclave(retval: usize) {
    sbi::exit_enclave(retval);
}
