use spin::{Mutex, MutexGuard};

use super::sbi;
use crate::cfg::KERNEL_UTM_BASE;
use crate::edge::{EdgeCaller, EdgeMemory, GlobalEdgeCaller};

/// The Keystone global edge caller, which provides access to `KsEdgeCallerHolder`
/// and prevents concurrent edge calls by using a mutex.
///
/// In Rust, it is represented by a `Mutex` holding some phantom data.
///
/// This struct must be marked public (in order to compile successfully).
pub struct KsGlobalEdgeCaller(Mutex<()>);

/// The Keystone edge caller holder, which provides unrestricted access to
/// edge memory and allows the user to issue edge calls.
///
/// It is returned by `KsGlobalEdgeCaller` and holds the mutex's lock.
/// Therefore, the object's holder will be the only one able to issue
/// edge calls when this object is alive.
///
/// When this object gets dropped, the mutex will be unlocked, thus
/// allowing others to acquire the edge caller again.
///
/// This struct must be marked public (in order to compile successfully).
pub struct KsEdgeCallerHolder<'l>(MutexGuard<'l, ()>);

/// Export the global edge caller to `crate::edge`. This must be marked
/// public (in order to compile successfully).
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
