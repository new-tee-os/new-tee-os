use x86_64::instructions::hlt;

use crate::qemu::QemuExitCode;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // try to exit QEMU
    crate::qemu::exit_qemu(QemuExitCode::Failed);

    // if QEMU didn't exit, trap inside an infinite loop
    loop {
        hlt();
    }
}
