#[cfg(feature = "async-edge")]
use alloc::boxed::Box;
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

pub trait EdgeStream {
    fn read(&mut self) -> u8;
    fn write(&mut self, ch: u8);

    fn read_bulk(&mut self, buf: &mut [u8]) {
        for byte in buf {
            *byte = self.read();
        }
    }

    fn write_bulk(&mut self, buf: &[u8]) {
        for byte in buf {
            self.write(*byte);
        }
    }
}

#[cfg(feature = "async-edge")]
#[async_trait::async_trait]
pub trait AsyncEdgeStream<E> {
    async fn read_bulk_async(&mut self, buf: &mut [u8]) -> Result<(), E>;
    async fn write_bulk_async(&mut self, buf: &[u8]) -> Result<(), E>;
}

impl EdgeMemory {
    pub const fn new() -> EdgeMemory {
        EdgeMemory {
            req: 0,
            buf_len: 0,
            result: 0,
            info: EdgeCallInfo::U64Info(0),
            buffer: [0; EDGE_BUFFER_SIZE],
        }
    }

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

    #[inline]
    pub fn serialize(&self, stream: &mut impl EdgeStream) {
        // write the header part
        stream.write_bulk(unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                memoffset::offset_of!(EdgeMemory, buffer),
            )
        });
        // write the data
        stream.write_bulk(self.read_buffer());
    }

    #[inline]
    pub fn deserialize(&mut self, stream: &mut impl EdgeStream) {
        // read the header part
        stream.read_bulk(unsafe {
            core::slice::from_raw_parts_mut(
                self as *mut Self as *mut u8,
                memoffset::offset_of!(EdgeMemory, buffer),
            )
        });
        // read the data
        stream.read_bulk(&mut self.buffer[0..(self.buf_len as usize)]);
    }
}

#[cfg(feature = "async-edge")]
impl EdgeMemory {
    #[inline]
    pub async fn serialize_async<E>(&self, stream: &mut impl AsyncEdgeStream<E>) -> Result<(), E> {
        // write the header part
        stream
            .write_bulk_async(unsafe {
                core::slice::from_raw_parts(
                    self as *const Self as *const u8,
                    memoffset::offset_of!(EdgeMemory, buffer),
                )
            })
            .await?;
        // write the data
        stream.write_bulk_async(self.read_buffer()).await?;
        Ok(())
    }

    #[inline]
    pub async fn deserialize_async<E>(
        &mut self,
        stream: &mut impl AsyncEdgeStream<E>,
    ) -> Result<(), E> {
        // read the header part
        stream
            .read_bulk_async(unsafe {
                core::slice::from_raw_parts_mut(
                    self as *mut Self as *mut u8,
                    memoffset::offset_of!(EdgeMemory, buffer),
                )
            })
            .await?;
        // read the data
        stream
            .read_bulk_async(&mut self.buffer[0..(self.buf_len as usize)])
            .await?;
        Ok(())
    }
}
