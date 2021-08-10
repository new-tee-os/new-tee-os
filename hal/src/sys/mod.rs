// dispatch to architecture's implementation
cfg_if::cfg_if! {
    // use `target_arch` to work around an issue of rust-analyzer
    if #[cfg(all(target_arch = "riscv64", feature = "keystone"))] {
        mod keystone;
        pub use keystone::*;
    } else if #[cfg(feature = "x86-vm")] {
        mod x86_vm;
        pub use x86_vm::*;
    } else {
        compile_error!("unsupported platform configuration");
    }
}
