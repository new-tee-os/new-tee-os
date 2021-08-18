pub mod devices;
pub mod idt;
pub mod pic;

use hal::arch::x86_vm::gdt;

pub fn init() {
    gdt::GDT.load();
    unsafe {
        gdt::apply_selectors();
    }
    hal::arch::x86_vm::security::enforce();

    idt::IDT.load();
    pic::init();
}
