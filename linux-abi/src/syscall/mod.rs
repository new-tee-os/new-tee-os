use phf::{phf_map, Map};
mod fs;
mod process;

#[derive(Clone, Copy)]
pub enum SyscallHandler {
    Syscall1(unsafe fn(usize) -> isize),
    Syscall3(unsafe fn(usize, usize, usize) -> isize),
}

// https://elixir.bootlin.com/linux/latest/source/include/uapi/asm-generic/unistd.h
pub static SYSCALL_MAP: Map<u32, SyscallHandler> = phf_map! {
    64u32 => fs::SYSCALL_WRITE,
    93u32 => process::SYSCALL_EXIT,
};

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
