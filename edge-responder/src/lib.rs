use std::convert::{TryFrom, TryInto};

use hal::edge::{EdgeCallInfo, EdgeCallReq, EdgeMemory};

mod edge_file;

pub unsafe fn handle_edge_call(edge_mem: *mut EdgeMemory) {
    use EdgeCallReq::*;

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
        EdgeCallFileApi => edge_file::dispatch_api_call(edge_mem),
        _ => {
            println!("Warning: invalid edge call number, ignoring");
        }
    }
}

unsafe fn handle_syscall(edge_mem: &mut EdgeMemory) {
    let result = match edge_mem.read_info() {
        EdgeCallInfo::SyscallWrite { fd, len } => {
            nix::unistd::write(fd as i32, &edge_mem.buffer[0..len as usize])
        }
        _ => panic!("unknown syscall type"),
    };
    // wrap the result in an i64
    let result: i64 = match result {
        Ok(retval) => retval.try_into().expect("integer overflow?!"),
        Err(err) => err as i32 as i64,
    };
    edge_mem.result = result;
}
