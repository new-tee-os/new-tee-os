pub mod devices;
pub mod gdt;
pub mod idt;
pub mod pic;
pub mod tss;

pub fn init() {
    gdt::GDT.load();
    unsafe {
        gdt::apply_selectors();
    }
    hal::arch::x86_vm::security::enforce();

    idt::IDT.load();
    pic::init();
}
