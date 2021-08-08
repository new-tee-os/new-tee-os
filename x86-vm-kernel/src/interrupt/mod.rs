pub mod devices;
pub mod gdt;
pub mod idt;
pub mod pic;
pub mod tss;

pub fn init() {
    gdt::GDT.gdt.load();
    unsafe {
        x86_64::instructions::segmentation::set_cs(gdt::GDT.code_selector);
        x86_64::instructions::segmentation::load_ds(gdt::GDT.data_selector);
        x86_64::instructions::segmentation::load_es(gdt::GDT.data_selector);
        x86_64::instructions::segmentation::load_ss(gdt::GDT.data_selector);
        x86_64::instructions::tables::load_tss(gdt::GDT.tss_selector);
    }

    idt::IDT.load();
    pic::init();
}
