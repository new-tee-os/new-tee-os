use x86_64::VirtAddr;

global_asm!(include_str!("asm/syscall.asm"));

extern "C" {
    fn syscall_entry();
}

#[no_mangle]
extern "C" fn handle_syscall(arg0: usize, arg1: usize, arg2: usize, nr: usize) {
    panic!("syscall number {}", nr);
}

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
    msr::LStar::write(VirtAddr::from_ptr(syscall_entry as *const ()));

    // set IA32_EFER.SCE = 1
    unsafe {
        msr::Efer::update(|flags| {
            flags.set(msr::EferFlags::SYSTEM_CALL_EXTENSIONS, true);
        });
    }
}
