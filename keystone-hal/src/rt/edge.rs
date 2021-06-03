use core::sync::atomic::{AtomicBool, Ordering};

use edge_lib::{EdgeCaller, EdgeMemory};

use crate::sbi;

struct KeystoneEdgeCaller(AtomicBool);

static EDGE_CALLER: KeystoneEdgeCaller = KeystoneEdgeCaller(AtomicBool::new(false));

#[no_mangle]
static GLOBAL_EDGE_CALLER: &'static dyn EdgeCaller = &EDGE_CALLER;

const EDGE_MEM_BASE: *mut EdgeMemory = keystone_cfg::KERNEL_UTM_BASE as _;

impl EdgeCaller for KeystoneEdgeCaller {
    fn acquire(&self) {
        let prev = self.0.swap(true, Ordering::Relaxed);
        assert_eq!(prev, false, "the edge caller is not reentrant");
    }

    fn edge_mem(&self) -> &mut EdgeMemory {
        unsafe { &mut *EDGE_MEM_BASE }
    }

    unsafe fn edge_call(&self) {
        sbi::stop_enclave(sbi::STOP_EDGE_CALL_HOST);
    }

    fn release(&self) {
        self.0.store(false, Ordering::Relaxed);
    }
}
