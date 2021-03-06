#![no_std]
#![no_main]
#![feature(abi_x86_interrupt, asm, global_asm, panic_info_message)]

extern crate alloc;

pub use hal::cfg;

mod elf;
mod heap;
mod interrupt;
mod klog;
mod memory;
mod panic;
mod syscall;
mod user;

use bootloader::{entry_point, BootInfo};
use x86_64::instructions::hlt;

entry_point!(start_kernel);

fn clear_screen(boot_info: &mut BootInfo) {
    let vga_buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    vga_buffer.fill(0);
}

fn start_kernel(boot_info: &'static mut BootInfo) -> ! {
    klog::klog_init().unwrap();
    // assert that a mirror mapping is created at `KERNEL_MIRROR_BASE`
    assert_eq!(
        Option::from(boot_info.physical_memory_offset),
        Some(cfg::KERNEL_MIRROR_BASE as u64)
    );
    heap::init(boot_info);

    hal::arch::x86_vm::arch_init();
    syscall::init();
    interrupt::init();
    x86_64::instructions::interrupts::enable();

    clear_screen(boot_info);
    log::info!("It didn't crash!");

    user::enter_user_mode();

    loop {
        hlt();
    }
}
