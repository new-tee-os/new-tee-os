#[repr(C)]
pub struct EdgeMemory<'a> {
    pub buffer: &'a mut [u8],
    pub len: usize,
}

impl<'a> EdgeMemory<'a> {
    pub fn write_buffer(&mut self, data: &[u8]) {
        assert!(data.len() <= crate::cfg::EDGE_BUFFER_SIZE);
        self.buffer[0..data.len()].copy_from_slice(data);
    }

    pub fn read_buffer(&self) -> &[u8] {
        &self.buffer[0..(self.len as usize)]
    }
}
