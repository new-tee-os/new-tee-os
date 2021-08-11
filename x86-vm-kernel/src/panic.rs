use x86_64::instructions::hlt;

use hal::{dbg_print, dbg_println};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    dbg_print!("Kernel panic - aborting: ");
    if let Some(p) = info.location() {
        dbg_println!("at {}:{}: {}", p.file(), p.line(), info.message().unwrap());
    } else {
        dbg_println!("no information available.");
    }

    // try to exit QEMU
    hal::arch::x86_vm::qemu::exit_qemu(1);

    // if QEMU didn't exit, trap inside an infinite loop
    loop {
        hlt();
    }
}
