use core::convert::TryFrom;

use super::EdgeCallInfo;

pub const EDGE_BUFFER_SIZE: usize = 3 << 10;
pub const EDGE_CALL_INFO_SIZE: usize = 256;

#[repr(C)]
pub struct EdgeMemory {
    pub req: u32,
    pub buf_len: u32,
    pub result: i64,
    pub info: EdgeCallInfo,
    pub buffer: [u8; EDGE_BUFFER_SIZE],
}

#[repr(u32)]
#[derive(num_enum::TryFromPrimitive, num_enum::IntoPrimitive)]
pub enum EdgeCallReq {
    EdgeCallInvalid,
    EdgeCallPrint,
    EdgeCallSyscall,
    EdgeCallFileApi,
}

impl EdgeMemory {
    #[inline]
    pub fn read_request(&self) -> EdgeCallReq {
        EdgeCallReq::try_from(self.req).expect("invalid edge call request")
    }

    #[inline]
    pub fn write_request(&mut self, req: EdgeCallReq) -> &mut Self {
        self.req = req.into();
        self
    }

    #[inline]
    pub fn read_buffer(&self) -> &[u8] {
        &self.buffer[0..(self.buf_len as usize)]
    }

    #[inline]
    pub fn write_buffer(&mut self, data: &[u8]) -> &mut Self {
        use core::convert::TryInto;

        assert!(data.len() <= EDGE_BUFFER_SIZE);
        self.buffer[0..data.len()].copy_from_slice(data);
        self.buf_len = data.len().try_into().unwrap();

        self
    }

    #[inline]
    pub fn read_info(&self) -> EdgeCallInfo {
        self.info
    }

    #[inline]
    pub fn write_info(&mut self, info: EdgeCallInfo) -> &mut Self {
        self.info = info;

        self
    }

    #[inline]
    pub fn read_syscall_result(&self) -> isize {
        use core::convert::TryInto;

        self.result.try_into().expect("integer overflow?!")
    }
}
