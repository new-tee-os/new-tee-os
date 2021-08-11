/// Memory APIs (e.g. `copy_from_user`).
// TODO: port to x86 VM
#[cfg(not(feature = "x86-vm"))]
pub mod mem;

/// Process APIs.
// TODO: port to x86 VM
#[cfg(not(feature = "x86-vm"))]
pub mod task;

// expose the `exit_enclave` API
pub use crate::sys::exit_enclave;
