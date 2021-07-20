#![no_std]
#![no_main]

mod panic;

use bootloader::{entry_point, BootInfo};
use x86_64::instructions::hlt;

entry_point!(start_kernel);

fn clear_screen(boot_info: &'static mut BootInfo) {
    let vga_buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    let pattern = b"\x00\xDD\x00";

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
