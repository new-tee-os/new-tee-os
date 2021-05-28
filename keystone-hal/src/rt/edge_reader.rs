use bytes::{Buf, BufMut};
use keystone_cfg as cfg;

use crate::{
    edge::{EdgeCallReq, EdgeMemory},
    edge_call,
};

pub struct EdgeReader {
    file_obj: u64,
}

impl EdgeReader {
    pub fn new(edge_mem: &mut EdgeMemory, path: &str) -> EdgeReader {
        edge_mem.req = EdgeCallReq::EdgeCallOpen.into();
        edge_mem.write_buffer(path.as_bytes());
        unsafe { edge_call() };
        assert_eq!(edge_mem.req, 0, "failed to open edge file");
        EdgeReader {
            file_obj: (&edge_mem.buffer[0..8]).get_u64(),
        }
    }

    fn read_once(&mut self, edge_mem: &mut EdgeMemory, dest: &mut [u8]) -> usize {
        edge_mem.req = EdgeCallReq::EdgeCallRead.into();
        (&mut edge_mem.buffer[0..8]).put_u64(self.file_obj);
        edge_mem.len = dest.len() as u32;
        unsafe { edge_call() };
        assert_eq!(edge_mem.req, 0, "failed to read edge file");
        dest.copy_from_slice(edge_mem.read_buffer());
        edge_mem.len as usize
    }

    pub fn size(&self, edge_mem: &mut EdgeMemory) -> usize {
        edge_mem.req = EdgeCallReq::EdgeCallGetSize.into();
        (&mut edge_mem.buffer[0..8]).put_u64(self.file_obj);
        unsafe { edge_call() };
        assert_eq!(edge_mem.req, 0, "failed to stat edge file");
        (&edge_mem.buffer[0..8]).get_u64() as usize
    }

    pub fn read(&mut self, edge_mem: &mut EdgeMemory, mut dest: &mut [u8]) -> usize {
        let mut bytes_read = 0;
        while dest.len() > cfg::EDGE_BUFFER_SIZE {
            let bytes_read_cur = self.read_once(edge_mem, &mut dest[0..cfg::EDGE_BUFFER_SIZE]);
            bytes_read += bytes_read_cur;
            if bytes_read_cur < cfg::EDGE_BUFFER_SIZE {
                return bytes_read_cur;
            }
            dest = &mut dest[cfg::EDGE_BUFFER_SIZE..];
        }
        bytes_read += self.read_once(edge_mem, dest);
        bytes_read
    }

    pub fn close(self, edge_mem: &mut EdgeMemory) {
        edge_mem.req = EdgeCallReq::EdgeCallClose.into();
        (&mut edge_mem.buffer[0..8]).put_u64(self.file_obj);
        unsafe { edge_call() };
    }
}
