use super::SyscallHandler;
use crate::println;

pub const SYSCALL_EXIT: SyscallHandler = SyscallHandler::Syscall1(syscall_exit);

unsafe fn syscall_exit(retval: usize) -> usize {
    println!("U-mode program exited with status {}", retval);
    crate::sbi::exit_enclave(retval);
    unreachable!()
}
