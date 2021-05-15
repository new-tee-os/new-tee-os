#![no_std]
#![no_main]
#![feature(asm, global_asm, panic_info_message)]

mod entry;
mod frame;
mod panic;
mod sbi;
mod syscall;
mod trap;
mod uart;

#[no_mangle]
extern "C" fn rt_init() {
    println!("Hello, RISC-V Keystone!");

    // execute U-mode program
    unsafe {
        riscv::register::sepc::write(0x400000);
        riscv::register::sstatus::set_spp(riscv::register::sstatus::SPP::User);
        asm!(
            r#"
            csrw sscratch, sp;
            li sp, 0x402000;
            sret
            "#
        );
    }

    sbi::exit_enclave(0);
}
