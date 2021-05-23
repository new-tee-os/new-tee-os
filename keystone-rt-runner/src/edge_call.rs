use keystone_hal::edge::EdgeCallReq::{self, *};
use keystone_hal::edge::EdgeMemory;
use keystone_hal::edge_syscall::SyscallReq;
use std::convert::{TryFrom, TryInto};

pub unsafe fn handle_edge_call(edge_mem: *mut EdgeMemory) {
    let edge_mem = &mut *edge_mem;
    match EdgeCallReq::try_from(edge_mem.req).unwrap_or(EdgeCallInvalid) {
        EdgeCallPrint => {
            print!(
                "{}",
                std::str::from_utf8(edge_mem.read_buffer())
                    .expect("the enclave tries to print an invalid UTF-8 string")
            );
            // return 42
            edge_mem.req = 42;
        }
        EdgeCallSyscall => handle_syscall(edge_mem),
        _ => {
            println!("Warning: invalid edge call number, ignoring");
        }
    }
}

unsafe fn handle_syscall(edge_mem: &mut EdgeMemory) {
    let req = edge_mem.read_syscall_request();
    let result = match req {
        SyscallReq::Write { fd, len } => {
            nix::unistd::write(fd as i32, &edge_mem.buffer[0..len as usize])
        }
    };
    let result: i64 = match result {
        Ok(retval) => retval.try_into().expect("integer overflow?!"),
        Err(err) => err.as_errno().expect("not an errno?!") as i32 as i64,
    };
    edge_mem.result = result;
}
