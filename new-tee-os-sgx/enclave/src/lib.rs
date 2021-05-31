#![crate_name = "newteeosenclave"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
// #[cfg(not(target_env = "sgx"))]
// #[macro_use]
// extern crate sgx_tstd as std;

use sgx_types::*;
// use std::string::String;
// use std::vec::Vec;
// use std::io::{self, Write};
// use std::slice;

extern crate alloc;
mod elfloader;

mod linux_abi;
mod sgx_rt;

#[feature(asm)]
#[no_mangle]
pub extern "C" rt_main(){
    // load U-mode program
    let entry;
    unsafe {

        // let edge_mem = &mut *keystone_hal::EDGE_MEM_BASE;
        // // read ELF file
        // let mut elf_file = EdgeReader::new(edge_mem, "keystone-init");
        // let mut elf_data = alloc::vec![0; elf_file.size(edge_mem)];
        // elf_file.read(edge_mem, &mut elf_data);
        // elf_file.close(edge_mem);


        let elf = elf_loader::ElfFile::load(&elf_data);
        let entry = elf.entry() as usize;
        let sp=elf.prepare_libc_args();

        unsafe{
            asm!(
                "mov rsp, stackp":"r{stackp}"(sp):::"intel",
                "mov rbp, framep":"r{framep}"(sp):::"intel",
                "call usr_main":"r{usr_main}"(entry):::"intel",
            )
        };

    }
    debug!("user bin returned?")
    linux_abi::syscall::process::SYSCALL_EXIT(0);
}