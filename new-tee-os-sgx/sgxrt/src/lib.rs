#![crate_name = "newteeossgxrt"]
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
mod sgx_cfg;
mod sgx_hal;

use sgx_hal::EDGE_MEM_BASE;

#[feature(asm)]
#[no_mangle]
pub extern "C" fn rt_main(sharemem: *mut u8, memsz: usize) -> sgx_status_t{
    // load U-mode program
    let entry;
    unsafe {
        let edge_mem=&mut EDGE_MEM_BASE;
        edge_mem.buffer=unsafe { core::slice::from_raw_parts(sharemem, memsz) };
        edge_mem.len=memsz;

        let elf_data= edge_mem.read_buffer();

        let elf = elf_loader::ElfFile::load(&elf_data);
        let entry = elf.entry() as usize;
        let sp=elf.prepare_libc_args();

        unsafe{
            asm!(
                "mov rsp, stackp":"r{stackp}"(sp):::"intel",
                "mov rbp, framep":"r{framep}"(sp):::"intel",
                //@? asm call an the main address
                "call usr_main":"r{usr_main}"(entry):::"intel",
            )
        };

    }
    debug!("user bin returned?")
    linux_abi::syscall::process::SYSCALL_EXIT(0);
    sgx_status_t::SGX_SUCCESS
}