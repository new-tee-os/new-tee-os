pub mod devices;
pub mod gdt;
pub mod idt;
pub mod pic;
pub mod tss;

pub fn init() {
    gdt::GDT.load();
    unsafe {
        use x86_64::instructions::segmentation::*;
        CS::set_reg(gdt::KERNEL_CODE_SEL);
        DS::set_reg(gdt::KERNEL_DATA_SEL);
        ES::set_reg(gdt::KERNEL_DATA_SEL);
        SS::set_reg(gdt::KERNEL_DATA_SEL);
        x86_64::instructions::tables::load_tss(gdt::TSS_SEL);
    }

    idt::IDT.load();
    pic::init();
}
