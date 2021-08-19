#![no_std]
#![no_main]
#![feature(asm, global_asm)]

global_asm!(
    r#"
    .section .text.entry
    .global _start

_start:
    j  main
    "#
);

const WRITE: usize = 64;
const EXIT: usize = 93;
static MSG: &str = "Hello, world!\n";

unsafe fn syscall(nr: usize, arg0: usize, arg1: usize, arg2: usize) {
    asm!(
        "ecall",
        in("a7") nr,
        in("a0") arg0,
        in("a1") arg1,
        in("a2") arg2,
    );
}

#[no_mangle]
extern "C" fn main() {
    unsafe {
        syscall(WRITE, 1, MSG.as_bytes().as_ptr() as usize, MSG.len());
        syscall(EXIT, 0, 0, 0);
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
