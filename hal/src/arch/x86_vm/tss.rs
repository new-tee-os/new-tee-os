use lazy_static::lazy_static;
use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

pub const EMERGENCY_IST_INDEX: u16 = 0;
const EMERGENCY_STACK_SIZE: usize = 4096 * 4;

lazy_static! {
    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[EMERGENCY_IST_INDEX as usize] = {
            // an emergency stack used at double faults
            static mut EMERGENCY_STACK: [u8; EMERGENCY_STACK_SIZE] = [0; EMERGENCY_STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &EMERGENCY_STACK });
            let stack_end = stack_start + EMERGENCY_STACK_SIZE;
            stack_end
        };
        tss.privilege_stack_table[0] = {
            // no cascaded interrupt here, so one stack per CPU is sufficient
            static mut KERNEL_INTR_STACK: [u8; EMERGENCY_STACK_SIZE] = [0; EMERGENCY_STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &KERNEL_INTR_STACK });
            let stack_end = stack_start + EMERGENCY_STACK_SIZE;
            stack_end
        };
        tss
    };
}
