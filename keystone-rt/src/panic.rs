use crate::{print, println};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Kernel panic - aborting: ");
    if let Some(p) = info.location() {
        println!("at {}:{}: {}", p.file(), p.line(), info.message().unwrap());
    } else {
        println!("no information available.");
    }
    loop {
        unsafe {
            riscv::asm::wfi();
        }
    }
}
