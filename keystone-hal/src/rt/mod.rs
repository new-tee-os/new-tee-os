mod edge;
pub mod mem;
pub mod sbi;

pub fn exit_enclave(retval: usize) {
    sbi::exit_enclave(retval);
}
