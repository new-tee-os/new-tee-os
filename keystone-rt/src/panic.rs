use crate::{uart_print, uart_println};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    uart_print!("Kernel panic - aborting: ");
    if let Some(p) = info.location() {
        uart_println!("at {}:{}: {}", p.file(), p.line(), info.message().unwrap());
    } else {
        uart_println!("no information available.");
    }
    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}
