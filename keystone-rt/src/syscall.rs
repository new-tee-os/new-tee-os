use core::convert::TryInto;

use crate::frame::TrapFrame;
use linux_abi::syscall::{SyscallHandler, SYSCALL_MAP};
use riscv::register::sepc;

pub unsafe fn handle_syscall(frame: *mut TrapFrame) {
    // get arguments from the frame
    let (nr, arg0, arg1, arg2) = {
        let frame = &*frame;
        (frame.a7, frame.a0, frame.a1, frame.a2)
    };
    let nr = nr.try_into().unwrap();
    let result;

    // dispatch syscall by number
    match SYSCALL_MAP.get(&nr).map(|&f| f) {
        Some(SyscallHandler::Syscall1(f)) => {
            result = f(arg0);
        }
        Some(SyscallHandler::Syscall3(f)) => {
            result = f(arg0, arg1, arg2);
        }
        None => panic!("unknown syscall number {}", nr),
    }

    // write return value back to the frame
    (*frame).a0 = result as usize;
    // move to the next instruction of `ecall`
    sepc::write(sepc::read() + 4);
}
