#![no_std]
#![no_main]
#![feature(asm, global_asm, panic_info_message)]

extern crate alloc;

use kmalloc::{Kmalloc, LockedLinkedListHeap};

mod entry;
mod frame;
mod klog;
mod panic;
mod syscall;
mod trap;
mod uart;
mod vm;

#[global_allocator]
static ALLOC: LockedLinkedListHeap = unsafe { LockedLinkedListHeap::uninit() };

#[no_mangle]
extern "C" fn rt_main(vm_info: &vm::VmInfo) {
    // initialize modules
    klog::klog_init().expect("failed to initialize klog module");
    unsafe {
        ALLOC.init(vm_info.free_virt as *mut u8, vm_info.free_size);
    }
    log::debug!("It did not crash!");

    // execute U-mode program
    unsafe {
        riscv::register::sepc::write(0x400000);
        riscv::register::sstatus::set_spp(riscv::register::sstatus::SPP::User);
        #[rustfmt::skip]
        asm!(
            "csrw sscratch, sp",
            "li sp, 0x402000",
            "sret",
        );
    }
}
