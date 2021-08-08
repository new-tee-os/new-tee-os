use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use super::pic::{InterruptIndex, PICS};
use crate::dbg_print;

extern "x86-interrupt" fn timer_int(_stack_frame: InterruptStackFrame) {
    dbg_print!(".");
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub fn load_idt_entries(idt: &mut InterruptDescriptorTable) {
    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_int);
}
