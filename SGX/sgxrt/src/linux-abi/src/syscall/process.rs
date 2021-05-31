use super::SyscallHandler;
use sgx_urts::{sgx_enclave_id_t,rsgx_destroy_enclave};
use log::*;


pub const SYSCALL_EXIT: SyscallHandler = SyscallHandler::Syscall1(syscall_exit);

unsafe fn syscall_exit(eid: sgx_enclave_id_t) -> isize {
    sgx_urts::rsgx_destroy_enclave(eid);
    unreachable!()
}