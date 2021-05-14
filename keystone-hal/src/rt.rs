use crate::edge::EdgeMemory;

extern "C" {
    fn _end();
}

pub const EDGE_MEM_BASE: *mut EdgeMemory = _end as _;
