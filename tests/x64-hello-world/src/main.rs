#![no_std]
#![no_main]
#![feature(asm, global_asm)]

global_asm!(
    r#"
    .section .text.entry
    .global _start

_start:
    jmp     main
    "#
);

const WRITE: usize = 1;
const EXIT: usize = 60;
static MSG: &str = "Hello, world!\n";

unsafe fn syscall(nr: usize, arg0: usize, arg1: usize, arg2: usize) {
    asm!(
        "syscall",
        inout("rax") nr => _,
        in("rdi") arg0,
        in("rsi") arg1,
        in("rdx") arg2,
        out("rcx") _,
        out("r11") _,
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
    unsafe {
        asm!("ud2", options(noreturn));
    }
}
