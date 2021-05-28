// enable no_std for !test
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod elfloader;

#[test]
fn test1() {
    elfloader::ElfFile::load(
        &std::fs::read("./riscv-hello-world").unwrap(),
        |from, to| println!("{:?} -> {:#X}", from, to),
    );
}
