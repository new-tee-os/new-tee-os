//! Lists all syscalls in a single module, allowing each architecture to easily
//! define their own syscall tables.

use super::*;

pub use fs::SYSCALL_WRITE;
pub use process::SYSCALL_EXIT;
