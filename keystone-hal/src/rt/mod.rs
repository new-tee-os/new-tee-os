pub mod edge_con;
pub mod edge_reader;
pub mod sbi;

use crate::edge::EdgeMemory;

pub const EDGE_MEM_BASE: *mut EdgeMemory = keystone_cfg::KERNEL_UTM_BASE as _;

#[cfg(feature = "rt")]
pub unsafe fn edge_call() {
    sbi::stop_enclave(sbi::STOP_EDGE_CALL_HOST);
}
