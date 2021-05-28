pub mod elfloader;

#[test]
fn test1() {
    elfloader::ElfFile::load(&std::fs::read("./riscv-hello-world").unwrap());
}
