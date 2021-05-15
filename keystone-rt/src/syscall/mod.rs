use core::convert::TryInto;

use crate::frame::TrapFrame;
use phf::{phf_map, Map};
use riscv::register::sepc;

mod fs;

#[derive(Clone, Copy)]
pub enum SyscallHandler {
    Syscall3(unsafe fn(usize, usize, usize) -> usize),
}

static SYSCALL_MAP: Map<u32, SyscallHandler> = phf_map! {
    4u32 => fs::SYSCALL_WRITE,
};

pub unsafe fn handle_syscall(frame: *mut TrapFrame) {
    let (nr, arg0, arg1, arg2) = {
        let frame = &*frame;
        (frame.a7, frame.a0, frame.a1, frame.a2)
    };
    let nr = nr.try_into().unwrap();
    let result;
    match SYSCALL_MAP.get(&nr).map(|&f| f) {
        Some(SyscallHandler::Syscall3(f)) => {
            result = f(arg0, arg1, arg2);
        }
        _ => panic!("unknown syscall number {}", nr),
    }
    (*frame).a0 = result;
    // move to the next instruction of `ecall`
    sepc::write(sepc::read() + 4);
}
