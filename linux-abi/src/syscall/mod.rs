mod fs;
/// Lists all syscalls in a single module, allowing each architecture to easily
/// build their own syscall tables.
pub mod listing;
mod process;

#[derive(Clone, Copy)]
pub enum SyscallHandler {
    Syscall1(unsafe fn(usize) -> isize),
    Syscall3(unsafe fn(usize, usize, usize) -> isize),
}

#[macro_export]
macro_rules! syscall_try {
    ($val:expr) => {{
        let val: isize = { $val };
        if val < 0 {
            return val;
        }
        val
    }};
}
