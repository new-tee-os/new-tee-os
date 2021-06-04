use alloc::vec;
use hal::edge::{EdgeCallInfo, EdgeCallReq, EDGE_BUFFER_SIZE};

use super::SyscallHandler;
use crate::syscall_try;

pub const SYSCALL_WRITE: SyscallHandler = SyscallHandler::Syscall3(syscall_write);

unsafe fn edge_write(fd: usize, buf: &[u8]) -> isize {
    hal::edge::with_edge_caller(|caller| {
        caller
            .edge_mem()
            .write_request(EdgeCallReq::EdgeCallSyscall)
            .write_info(EdgeCallInfo::SyscallWrite {
                fd: fd as u64,
                len: buf.len() as u64,
            })
            .write_buffer(buf);
        unsafe { caller.edge_call() };

        caller.edge_mem().read_syscall_result()
    })
}

unsafe fn syscall_write(fd: usize, ptr: usize, mut len: usize) -> isize {
    let mut bytes_written = 0;

    let mut ptr = ptr as *const u8;
    let mut buf = vec![0; EDGE_BUFFER_SIZE];
    while len > EDGE_BUFFER_SIZE {
        hal::mem::copy_from_user(&mut buf, ptr as *const u8);
        bytes_written += syscall_try!(edge_write(fd, &buf));
        ptr = ptr.add(EDGE_BUFFER_SIZE);
        len -= EDGE_BUFFER_SIZE;
    }

    hal::mem::copy_from_user(&mut buf[0..len], ptr as *const u8);
    bytes_written += syscall_try!(edge_write(fd, &buf[0..len]));

    bytes_written
}
