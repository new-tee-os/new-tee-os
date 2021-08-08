use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

use super::tss::TSS;

pub struct MyGdt {
    pub gdt: GlobalDescriptorTable,
    pub code_selector: SegmentSelector,
    pub data_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}

lazy_static! {
    pub static ref GDT: MyGdt = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        MyGdt {
            gdt,
            code_selector,
            data_selector,
            tss_selector,
        }
    };
}
