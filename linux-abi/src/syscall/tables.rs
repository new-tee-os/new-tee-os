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
