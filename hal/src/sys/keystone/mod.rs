/// Configurations for Keystone platform.
pub mod cfg;

#[cfg(feature = "keystone-rt")]
/// Keystone runtime (i.e. kernel) specific items.
pub mod rt;
#[cfg(feature = "keystone-rt")]
pub use rt::*;
