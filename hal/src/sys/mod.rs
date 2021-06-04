cfg_if::cfg_if! {
    if #[cfg(feature = "keystone")] {
        mod keystone;
        pub use keystone::*;
    } else {
        compile_error!("unsupported platform configuration");
    }
}
