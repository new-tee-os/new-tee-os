use alloc::vec;

use super::SyscallHandler;
use crate::{hal, syscall_try};
use hal::{cfg::EDGE_BUFFER_SIZE, edge_syscall::SyscallReq};

pub const SYSCALL_WRITE: SyscallHandler = SyscallHandler::Syscall3(syscall_write);

unsafe fn edge_write(fd: usize, buf: &[u8]) -> isize {
    let edge_mem = &mut *hal::EDGE_MEM_BASE;
    edge_mem.write_syscall_request(SyscallReq::Write {
        fd: fd as u64,
        len: buf.len() as u64,
    });
    edge_mem.write_buffer(buf);
    hal::edge_call();
    edge_mem.read_syscall_result()
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
