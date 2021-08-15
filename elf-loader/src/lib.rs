// enable no_std for !test
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod arch;
mod elf_loader;

pub use crate::elf_loader::*;

#[test]
fn test1() {
    elf_loader::ElfFile::load(
        &std::fs::read("./riscv-hello-world").unwrap(),
        arch::RiscV,
        |from, size, to| println!("({:?} + {:#X}) -> {:#X}", from, size, to),
    );
}
