#[repr(C)]
pub struct EdgeMemory {
    pub req: u32,
    pub len: u32,
    pub buffer: [u8; 3 << 10],
}

static_assertions::const_assert!(core::mem::size_of::<EdgeMemory>() <= 0x1000);

impl EdgeMemory {
    pub fn write_buffer(&mut self, data: &[u8]) {
        use core::convert::TryInto;

        assert!(data.len() <= self.buffer.len());
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
}
