extern crate alloc;
use alloc::vec;

use super::SyscallHandler;
use crate::{hal, syscall_try};
use hal::cfg::EDGE_BUFFER_SIZE;

pub const SYSCALL_WRITE: SyscallHandler = SyscallHandler::Syscall3(syscall_write);

#[link(name = "libtest.a", kind = "static")]
#[no_mangle]
extern "C" {
    fn ocall_syscall_write(fd: usize, ptr: *const u8, len: usize) -> isize;
}

unsafe fn syscall_write(fd: usize, ptr: usize, mut len: usize) -> isize {
    let edge_mem = &hal::EDGE_MEM_BASE;

    let mut bytes_written = 0;

    while len > EDGE_BUFFER_SIZE {
        edge_mem.write_buffer(core::slice::from_raw_slice_mut(ptr as *const u8, len));
        bytes_written += syscall_try!(ocall_syscall_write(fd, ptr as _, EDGE_BUFFER_SIZE));
        ptr = ptr.add(EDGE_BUFFER_SIZE);
        len -= EDGE_BUFFER_SIZE;
    }

    hal::mem::copy_from_user(&mut buf[0..len], ptr as *const u8);
    bytes_written += syscall_try!(ocall_syscall_write(fd, ptr as _, len));

    bytes_written
}
