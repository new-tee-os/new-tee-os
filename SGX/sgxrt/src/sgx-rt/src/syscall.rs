use core::convert::TryInto;
use phf::{phf_map, Map};
use linux_abi::syscall::{SyscallHandler, SYSCALL_MAP};

use sgx_types::{sgx_cpu_context_t,int32_t,int64_t};

#[no_mangle]
pub extern "C" fn handle_syscall(frame: *mut sgx_cpu_context_t)->int32_t{
    // get arguments from the frame
    let (nr, arg0, arg1, arg2) = {
        let frame = unsafe {&*frame;}
        (frame.cpu_context.rax, frame.cpu_context.rdi, frame.cpu_context.rsi, frame.cpu_context.rdx)
    };
    let nr = nr.try_into().unwrap();
    let result;

    // dispatch syscall by number
    match SYSCALL_MAP.get(&nr).map(|&f| f) {
        Some(SyscallHandler::Syscall1(f)) => {
            result = f(arg0);
        }
        Some(SyscallHandler::Syscall3(f)) => {s
            result = f(arg0, arg1, arg2);
        }
        None => panic!("unknown syscall number {}", nr),
    }

    // write return value back to the frame
    (*frame).cpu_context.rax = result as int64_t;
    // move to the next instruction of `int x80` ,which is required by the SGX Guide Ref
    //@?+4/+8?
    (*frame).cpu_context.rip = (*frame).cpu_context.rip + 4;
    // unsafe{
        //there are no iret instructions,and the sgx exception mechanism
        //ensures that the cpu context will be restored to the SSA 
        //rsp--sp rbp--frame rip--pc
        // asm!(
        //     "mov r12, return":"m{return}"(ret):::"intel",
        //     "mov r13, return":"m{return}"((*frame).cpu_context.r13):::"intel",
        //     "mov r14, return":"m{return}"((*frame).cpu_context.r14):::"intel",
        //     "mov r15, return":"m{return}"((*frame).cpu_context.r15):::"intel",
        //     "mov rbx, return":"m{return}"((*frame).cpu_context.rbx):::"intel",
        //     "mov rsi, return":"m{return}"((*frame).cpu_context.rsi):::"intel",
        //     "mov rdi, return":"m{return}"((*frame).cpu_context.rdi):::"intel",
        //     "mov r8, return":"r{return}"(ret):::"intel",
        //     "mov rbp, return":"m{return}"((*frame).cpu_context.rbp):::"intel",
        //     "mov rsp, return":"m{return}"((*frame).cpu_context.rsp):::"intel",
        //     "mov rip, r8",

        // );
    // }
}
