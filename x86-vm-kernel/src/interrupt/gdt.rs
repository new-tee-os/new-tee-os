use lazy_static::lazy_static;
use x86_64::{
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    PrivilegeLevel,
};

pub const KERNEL_CODE_SEL: SegmentSelector = SegmentSelector::new(1, PrivilegeLevel::Ring0);
pub const KERNEL_DATA_SEL: SegmentSelector = SegmentSelector::new(2, PrivilegeLevel::Ring0);
pub const TSS_SEL: SegmentSelector = SegmentSelector::new(3, PrivilegeLevel::Ring0);

lazy_static! {
    pub static ref GDT: GlobalDescriptorTable = {
        let mut gdt = GlobalDescriptorTable::new();
        assert_eq!(
            gdt.add_entry(Descriptor::kernel_code_segment()),
            KERNEL_CODE_SEL,
        );
        assert_eq!(
            gdt.add_entry(Descriptor::kernel_data_segment()),
            KERNEL_DATA_SEL,
        );
        assert_eq!(
            gdt.add_entry(Descriptor::tss_segment(&super::tss::TSS)),
            TSS_SEL,
        );
        gdt
    };
}
