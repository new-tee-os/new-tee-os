use x86_64::VirtAddr;

pub fn init() {
    use crate::interrupt::gdt;
    use x86_64::registers::model_specific as msr;

    // configure segment selectors
    msr::Star::write(
        gdt::USER_CODE_SEL,
        gdt::USER_DATA_SEL,
        gdt::KERNEL_CODE_SEL,
        gdt::KERNEL_DATA_SEL,
    )
    .unwrap();

    // configure syscall handler address
    msr::LStar::write(VirtAddr::new(0xdeadbeef));

    // set IA32_EFER.SCE = 1
    unsafe {
        msr::Efer::update(|flags| {
            flags.set(msr::EferFlags::SYSTEM_CALL_EXTENSIONS, true);
        });
    }
}
