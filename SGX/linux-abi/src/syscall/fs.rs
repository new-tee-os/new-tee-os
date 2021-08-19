use super::SyscallHandler;
use crate::{hal, syscall_try};
use hal::cfg::EDGE_BUFFER_SIZE;

pub const SYSCALL_WRITE: SyscallHandler = SyscallHandler::Syscall3(syscall_write);

// #[link(name = "libtest.a", kind = "static")]
extern "C" {
    fn ocall_syscall_write(eid: u64,fd: usize, ptr: *const u8, len: usize) -> isize;
}

unsafe fn syscall_write(fd: usize,mut ptr: usize, mut len: usize) -> isize {
    let edge_mem = &mut hal::EDGE_MEM_BASE;

    let mut bytes_written = 0;

    while len > EDGE_BUFFER_SIZE {
        edge_mem.write_buffer(core::slice::from_raw_parts_mut(ptr as *mut u8, len));
        let valid_cnt=syscall_try!(ocall_syscall_write(0,fd, edge_mem.buffer.as_ptr(), EDGE_BUFFER_SIZE));        
        if valid_cnt<0{
            return -1;
        }
        bytes_written += valid_cnt;
        ptr = ptr+EDGE_BUFFER_SIZE;
        len -= EDGE_BUFFER_SIZE;
    }


    edge_mem.write_buffer(core::slice::from_raw_parts_mut(ptr as *mut u8, len));
    let valid_cnt=syscall_try!(ocall_syscall_write(0,fd, edge_mem.buffer.as_ptr(), len));

    if valid_cnt<0{
        valid_cnt
    }else{
        bytes_written+valid_cnt
    }
}
