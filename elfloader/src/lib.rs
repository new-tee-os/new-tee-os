//#[cfg(feature="elf64")]
// #[derive(Debug)]
pub mod elfloader;


#[test]
fn test1() {//riscv-hello-world
    use elfloader::load_elf64_riscv;
    load_elf64_riscv("./as");
//     let a:[i32;3]=[1,2,3];
//     assert_eq!(a[..2],[1,2]);
}

