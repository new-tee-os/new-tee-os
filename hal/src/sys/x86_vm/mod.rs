// stub

pub mod cfg;
#[cfg(feature = "kernel")]
pub mod edge;

#[cfg(feature = "kernel")]
pub fn exit_enclave(retval: usize) {
    // send a "stream close" signal
    crate::edge::with_edge_caller(|caller| {
        caller
            .edge_mem()
            .write_request(crate::edge::EdgeCallReq::EdgeCallStreamClose)
            .write_buffer(&[]);
        unsafe {
            caller.edge_call();
        }
    });
    crate::arch::x86_vm::qemu::exit_qemu(retval as u32);
}
