#![no_std]
#![no_main]

mod com;
mod panic;

use bootloader::{entry_point, BootInfo};
use x86_64::instructions::hlt;

entry_point!(start_kernel);

fn clear_screen(boot_info: &'static mut BootInfo) {
    let vga_buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    let com1 = com::IsaSerialPort::new_uninit(com::COM1_PORT_BASE);
    // display a grid pattern if the serial is properly initialized
    let pattern = if let Ok(()) = unsafe { com1.init() } {
        b"\x00\xDD\x00"
    } else {
        b"\x00\x00\x00"
    };

    for &ch in b"Hello, world!\r\n".iter() {
        unsafe {
            com1.write(ch);
        }
    }

    for (i, byte) in vga_buffer.iter_mut().enumerate() {
        *byte = pattern[i % 3];
    }
}

fn start_kernel(boot_info: &'static mut BootInfo) -> ! {
    clear_screen(boot_info);

    loop {
        hlt();
    }
}
