use riscv::register::sstatus;

use super::SyscallHandler;
use keystone_hal::print;

pub const SYSCALL_WRITE: SyscallHandler = SyscallHandler::Syscall3(syscall_write);

unsafe fn syscall_write(fd: usize, ptr: usize, len: usize) -> usize {
    assert_eq!(fd, 1);
    sstatus::set_sum();
    {
        let ptr = ptr as *const u8;
        let content = core::slice::from_raw_parts(ptr, len);
        print!(
            "U-mode program writes: {}",
            core::str::from_utf8(content).expect("U-mode program tried to print invalid UTF-8")
        );
    }
    sstatus::clear_sum();
    len
}
