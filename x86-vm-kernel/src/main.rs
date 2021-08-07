#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod panic;
mod qemu;

use bootloader::{entry_point, BootInfo};
use x86_64::instructions::hlt;

entry_point!(start_kernel);

fn clear_screen(boot_info: &'static mut BootInfo) {
    let vga_buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    vga_buffer.fill(0);
}

fn start_kernel(boot_info: &'static mut BootInfo) -> ! {
    clear_screen(boot_info);

    loop {
        hlt();
    }
}
