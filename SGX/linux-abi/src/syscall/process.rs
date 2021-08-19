use super::SyscallHandler;
use sgx_types::sgx_enclave_id_t;//u64


pub const SYSCALL_EXIT: SyscallHandler = SyscallHandler::Syscall1(syscall_exit);

//TODO
unsafe fn syscall_exit(eid: usize) -> isize {
    // sgx_urts::rsgx_destroy_enclave(eid as sgx_enclave_id_t);
    // unreachable!()
        let info="hello world";
        let mut ptr=info.as_ptr() as usize;
        match crate::syscall::fs::SYSCALL_WRITE {
            crate::syscall::SyscallHandler::Syscall3(f)=>{f(0,ptr,11);},
            _=>panic!("Not a write syscall!"),
        }
    return 1;
}