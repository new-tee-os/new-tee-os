// use `target_arch` to work around an issue of rust-analyzer
#[cfg(all(target_arch = "riscv64", feature = "keystone"))]
pub mod keystone;

#[cfg(all(target_arch = "x86_64", feature = "x86-vm"))]
pub mod x86_vm;
