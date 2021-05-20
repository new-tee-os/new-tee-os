pub mod edge_con;
pub mod sbi;

use crate::edge::EdgeMemory;

pub const EDGE_MEM_BASE: *mut EdgeMemory = keystone_cfg::KERNEL_UTM_BASE as _;
