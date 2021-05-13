#![no_std]
#![no_main]
#![feature(asm, global_asm, panic_info_message)]

mod entry;
mod panic;
mod sbi;
mod trap;
mod uart;

#[no_mangle]
extern "C" fn rt_init() {
    println!("Hello, RISC-V Keystone!");
    sbi::exit_enclave(0);
}
