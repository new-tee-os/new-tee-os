cfg_if::cfg_if! {
    if #[cfg(feature = "x86-vm-kernel")] {
        pub mod gdt;
        pub mod qemu;
        pub mod security;
        pub mod tss;
    }
}
