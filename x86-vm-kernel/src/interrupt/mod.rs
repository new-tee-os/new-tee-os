pub mod gdt;
pub mod tss;
pub mod idt;

pub fn init() {
    gdt::GDT.gdt.load();
    unsafe {
        x86_64::instructions::segmentation::set_cs(gdt::GDT.code_selector);
        x86_64::instructions::tables::load_tss(gdt::GDT.tss_selector);
    }

    idt::IDT.load();
}
