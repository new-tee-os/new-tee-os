use crate::edge_syscall::SyscallReq;

#[repr(C)]
pub struct EdgeMemory {
    pub req: u32,
    pub len: u32,
    pub result: i64,
    pub syscall_req: [u8; 256],
    pub buffer: [u8; crate::cfg::EDGE_BUFFER_SIZE],
}

static_assertions::const_assert!(core::mem::size_of::<EdgeMemory>() <= 0x1000);

impl EdgeMemory {
    pub fn write_syscall_request(&mut self, req: SyscallReq) {
        self.req = EdgeCallReq::EdgeCallSyscall.into();
        req.write_to(&mut self.syscall_req);
    }

    pub unsafe fn read_syscall_request(&self) -> SyscallReq {
        SyscallReq::read_from(&self.syscall_req)
    }

    pub fn read_syscall_result(&self) -> isize {
        use core::convert::TryInto;

        self.result.try_into().expect("integer overflow?!")
    }

    pub fn write_buffer(&mut self, data: &[u8]) {
        use core::convert::TryInto;

        assert!(data.len() <= crate::cfg::EDGE_BUFFER_SIZE);
        self.buffer[0..data.len()].copy_from_slice(data);
        self.len = data.len().try_into().unwrap();
    }

    pub fn read_buffer(&self) -> &[u8] {
        &self.buffer[0..(self.len as usize)]
    }
}

#[repr(u32)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
pub enum EdgeCallReq {
    EdgeCallInvalid,
    EdgeCallPrint,
    EdgeCallSyscall,
}
