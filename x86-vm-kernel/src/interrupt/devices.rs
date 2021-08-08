use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use super::pic::InterruptIndex;
use crate::dbg_print;

extern "x86-interrupt" fn timer_int(_stack_frame: InterruptStackFrame) {
    dbg_print!(".");
}

pub fn load_idt_entries(idt: &mut InterruptDescriptorTable) {
    idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_int);
}
