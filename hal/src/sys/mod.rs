// dispatch to architecture's implementation
cfg_if::cfg_if! {
    if #[cfg(feature = "keystone")] {
        mod keystone;
        pub use keystone::*;
    } else if #[cfg(feature = "x86-vm")] {
        mod x86_vm;
        pub use x86_vm::*;
    } else {
        compile_error!("unsupported platform configuration");
    }
}
