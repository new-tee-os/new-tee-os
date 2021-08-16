use linux_abi::syscall::tables::TABLE_X86_64 as SYSCALL_TABLE;
use linux_abi::syscall::SyscallHandler;
use x86_64::VirtAddr;

global_asm!(include_str!("asm/syscall.asm"));

extern "C" {
    fn syscall_entry();
}

#[no_mangle]
unsafe extern "C" fn handle_syscall(arg0: usize, arg1: usize, arg2: usize, nr: usize) -> isize {
    crate::interrupt::gdt::enter_kernel();
    let result;

    // dispatch syscall by number
    let nr = nr as u32;
    match SYSCALL_TABLE.get(&nr).map(|&f| f) {
        Some(SyscallHandler::Syscall1(f)) => {
            result = f(arg0);
        }
        Some(SyscallHandler::Syscall3(f)) => {
            result = f(arg0, arg1, arg2);
        }
        None => panic!("unknown syscall number {}", nr),
    }

    hal::task::yield_to_sched();

    crate::interrupt::gdt::enter_user();
    result
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
