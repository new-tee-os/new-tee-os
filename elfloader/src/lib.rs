pub mod elfloader;

#[test]
fn test1() {
    use elfloader::load_elf64_riscv;
    load_elf64_riscv("./riscv-hello-world");
}
