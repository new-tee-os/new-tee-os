pub mod devices;
pub mod gdt;
pub mod idt;
pub mod pic;
pub mod tss;

pub fn init() {
    gdt::GDT.gdt.load();
    unsafe {
        use x86_64::instructions::segmentation::*;
        CS::set_reg(gdt::GDT.code_selector);
        DS::set_reg(gdt::GDT.data_selector);
        ES::set_reg(gdt::GDT.data_selector);
        SS::set_reg(gdt::GDT.data_selector);
        x86_64::instructions::tables::load_tss(gdt::GDT.tss_selector);
    }

    idt::IDT.load();
    pic::init();
}
