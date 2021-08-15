use crate::edge::{with_edge_caller, EdgeCallInfo, EdgeCallReq, EDGE_BUFFER_SIZE};

pub struct EdgeFile {
    file_obj: u64,
}

impl EdgeFile {
    pub fn open(path: &str) -> EdgeFile {
        let file_obj = with_edge_caller(|caller| {
            caller
                .edge_mem()
                .write_request(EdgeCallReq::EdgeCallFileApi)
                .write_info(EdgeCallInfo::FileOpen)
                .write_buffer(path.as_bytes());
            unsafe { caller.edge_call() };

            let edge_mem = caller.edge_mem();
            assert_eq!(edge_mem.req, 0, "failed to open edge file");
            let result = edge_mem.read_info();
            result.into()
        });
        EdgeFile { file_obj }
    }

    pub fn size(&self) -> usize {
        with_edge_caller(|caller| {
            caller
                .edge_mem()
                .write_request(EdgeCallReq::EdgeCallFileApi)
                .write_info(EdgeCallInfo::FileGetSize {
                    file_obj: self.file_obj,
                });
            unsafe { caller.edge_call() };

            let edge_mem = caller.edge_mem();
            assert_eq!(edge_mem.req, 0, "failed to stat edge file");
            u64::from(edge_mem.read_info()) as usize
        })
    }

    fn read_once(&mut self, dest: &mut [u8]) -> usize {
        assert!(dest.len() <= EDGE_BUFFER_SIZE);
        with_edge_caller(|caller| {
            let edge_mem = caller.edge_mem();
            edge_mem
                .write_request(EdgeCallReq::EdgeCallFileApi)
                .write_info(EdgeCallInfo::FileRead {
                    file_obj: self.file_obj,
                    len: dest.len() as u32,
                });
            unsafe { caller.edge_call() };

            let edge_mem = caller.edge_mem();
            assert_eq!(edge_mem.req, 0, "failed to read edge file");
            dest[0..edge_mem.buf_len as usize].copy_from_slice(edge_mem.read_buffer());
            edge_mem.buf_len as usize
        })
    }

    pub fn read(&mut self, mut dest: &mut [u8]) -> usize {
        let mut bytes_read = 0;
        while dest.len() > EDGE_BUFFER_SIZE {
            let bytes_read_cur = self.read_once(&mut dest[0..EDGE_BUFFER_SIZE]);
            bytes_read += bytes_read_cur;
            if bytes_read_cur < EDGE_BUFFER_SIZE {
                return bytes_read_cur;
            }
            dest = &mut dest[EDGE_BUFFER_SIZE..];
        }
        bytes_read += self.read_once(dest);
        bytes_read
    }

    pub fn seek(&mut self, pos: u64) {
        with_edge_caller(|caller| {
            let edge_mem = caller.edge_mem();
            edge_mem
                .write_request(EdgeCallReq::EdgeCallFileApi)
                .write_info(EdgeCallInfo::FileSeek {
                    file_obj: self.file_obj,
                    pos,
                });
            unsafe { caller.edge_call() };

            let edge_mem = caller.edge_mem();
            assert_eq!(edge_mem.req, 0, "failed to seek edge file");
        });
    }

    fn close_remote_file(&self) {
        with_edge_caller(|caller| {
            let edge_mem = caller.edge_mem();
            edge_mem
                .write_request(EdgeCallReq::EdgeCallFileApi)
                .write_info(EdgeCallInfo::FileClose {
                    file_obj: self.file_obj,
                });
            unsafe { caller.edge_call() };
        });
    }

    pub fn close(self) {
        // prevent drop handler from being called
        let guard = core::mem::ManuallyDrop::new(self);
        guard.close_remote_file();
    }
}

impl Drop for EdgeFile {
    fn drop(&mut self) {
        self.close_remote_file();
    }
}
