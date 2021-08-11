// stub

pub mod cfg;
#[cfg(feature = "kernel")]
pub mod edge;

#[cfg(feature = "kernel")]
pub fn exit_enclave(retval: usize) {
    crate::arch::x86_vm::qemu::exit_qemu(retval as u32);
}
