use keystone_hal as hal;

use super::SyscallHandler;
use log::*;

pub const SYSCALL_EXIT: SyscallHandler = SyscallHandler::Syscall1(syscall_exit);

unsafe fn syscall_exit(retval: usize) -> usize {
    debug!("U-mode program exited with status {}", retval);
    hal::sbi::exit_enclave(retval);
    unreachable!()
}
