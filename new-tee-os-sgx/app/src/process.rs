use super::SyscallHandler;
use sgx_urts::sgx_enclave_id_t;
use sgx_urts;
use log::*;


pub const SYSCALL_EXIT: SyscallHandler = SyscallHandler::Syscall1(syscall_exit);

unsafe fn syscall_exit(eid: sgx_enclave_id_t) -> isize {
    sgx_urts::rsgx_destroy_enclave(eid);
    unreacable!()
}