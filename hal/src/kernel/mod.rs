/// Memory APIs (e.g. `copy_from_user`).
pub mod mem;

/// Process APIs.
pub mod task;

// expose the `exit_enclave` API
pub use crate::sys::exit_enclave;
