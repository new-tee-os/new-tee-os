use alloc::vec;
use log::info;

use super::SyscallHandler;
use crate::hal;
use hal::cfg::EDGE_BUFFER_SIZE;

pub const SYSCALL_WRITE: SyscallHandler = SyscallHandler::Syscall3(syscall_write);

unsafe fn syscall_write(fd: usize, ptr: usize, mut len: usize) -> usize {
    assert_eq!(fd, 1);
    let bytes_written = len;

    let mut ptr = ptr as *const u8;
    let mut buf = vec![0; EDGE_BUFFER_SIZE];
    while len > EDGE_BUFFER_SIZE {
        hal::mem::copy_from_user(&mut buf, ptr as *const u8);
        info!("{}", core::str::from_utf8(&buf).unwrap());
        ptr = ptr.add(EDGE_BUFFER_SIZE);
        len -= EDGE_BUFFER_SIZE;
    }

    hal::mem::copy_from_user(&mut buf[0..len], ptr as *const u8);
    info!("{}", core::str::from_utf8(&buf[0..len]).unwrap());
    bytes_written
}
