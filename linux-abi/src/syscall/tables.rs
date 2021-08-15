//! System call tables for various architectures.
//!
//! If this does not contain what you need, feel free to define your own one.

use phf::{phf_map, Map};

use super::{listing::*, SyscallHandler};

// https://elixir.bootlin.com/linux/latest/source/include/uapi/asm-generic/unistd.h
pub static TABLE_GENERIC: Map<u32, SyscallHandler> = phf_map! {
    64u32 => SYSCALL_WRITE,
    93u32 => SYSCALL_EXIT,
};

// https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/
pub static TABLE_X86_64: Map<u32, SyscallHandler> = phf_map! {
    1u32 => SYSCALL_WRITE,
    60u32 => SYSCALL_EXIT,
};
