use phf::{phf_map, Map};
// mod fs;
pub mod syscall{
    
    pub mod process;

    #[derive(Clone, Copy)]
    pub enum SyscallHandler {
        Syscall1(unsafe fn(usize) -> isize),
        Syscall3(unsafe fn(usize, usize, usize) -> isize),
    }
    
    // https://elixir.bootlin.com/linux/latest/source/include/uapi/asm-generic/unistd.h
    pub static SYSCALL_MAP: Map<u64, SyscallHandler> = phf_map! {
        //different archs has different syscall numbers, here is x86_64's
        // 1u64 => fs::SYSCALL_WRITE,
        60u64 => process::SYSCALL_EXIT,
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
}