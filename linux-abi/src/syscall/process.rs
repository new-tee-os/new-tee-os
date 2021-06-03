use super::SyscallHandler;
use crate::hal;
use log::*;

pub const SYSCALL_EXIT: SyscallHandler = SyscallHandler::Syscall1(syscall_exit);

unsafe fn syscall_exit(retval: usize) -> isize {
    debug!("U-mode program exited with status {}", retval);
    hal::exit_enclave(retval);
    unreachable!()
}
