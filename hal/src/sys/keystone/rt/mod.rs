mod edge;
pub mod mem;

use crate::arch::keystone::sbi;

pub fn edge_call(retval: usize) {
    sbi::exit_enclave(retval);
}

pub fn exit_enclave(retval: usize) {
    sbi::exit_enclave(retval);
}
