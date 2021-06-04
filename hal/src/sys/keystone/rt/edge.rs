use spin::{Mutex, MutexGuard};

use super::sbi;
use crate::cfg::KERNEL_UTM_BASE;
use crate::edge::{EdgeCaller, EdgeMemory, GlobalEdgeCaller};

pub struct KsEdgeCallerHolder<'l>(MutexGuard<'l, ()>);
pub struct KsGlobalEdgeCaller(Mutex<()>);

pub static GLOBAL_EDGE_CALLER: KsGlobalEdgeCaller = KsGlobalEdgeCaller(Mutex::new(()));

const EDGE_MEM_BASE: *mut EdgeMemory = KERNEL_UTM_BASE as _;

impl EdgeCaller for KsEdgeCallerHolder<'_> {
    fn edge_mem(&mut self) -> &mut EdgeMemory {
        unsafe { &mut *EDGE_MEM_BASE }
    }

    unsafe fn edge_call(&self) {
        sbi::stop_enclave(sbi::STOP_EDGE_CALL_HOST);
    }
}

impl<'l> GlobalEdgeCaller<'l, KsEdgeCallerHolder<'l>> for KsGlobalEdgeCaller {
    fn acquire(&'l self) -> KsEdgeCallerHolder {
        KsEdgeCallerHolder(self.0.try_lock().expect("the edge caller is not reentrant"))
    }
}
