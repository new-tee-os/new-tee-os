#![no_std]
#![no_main]
#![feature(asm, global_asm, panic_info_message)]

use keystone_hal::{edge::EdgeCallReq, rt::EDGE_MEM_BASE};

mod entry;
mod panic;
mod sbi;
mod trap;
mod uart;

#[no_mangle]
extern "C" fn rt_init() {
    println!("Hello, RISC-V Keystone!");
    println!("Edge memory address: {:?}", EDGE_MEM_BASE);

    unsafe {
        let edge_mem = &mut *EDGE_MEM_BASE;
        edge_mem.req = EdgeCallReq::EdgeCallPrint.into();
        edge_mem.write_buffer("Hello world from enclave!\n".as_bytes());
    }

    sbi::stop_enclave(sbi::STOP_EDGE_CALL_HOST);

    unsafe {
        let edge_mem = &*EDGE_MEM_BASE;
        println!("Edge call result: {}", edge_mem.req);
    }

    sbi::exit_enclave(0);
}
