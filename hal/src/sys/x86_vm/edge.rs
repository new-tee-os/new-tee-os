use spin::{Mutex, MutexGuard};

use crate::edge::{EdgeCallerHolder, EdgeMemory, GlobalEdgeCaller};

pub struct QemuGlobalEdgeCaller(Mutex<EdgeMemory>);
pub struct QemuEdgeCallerHolder<'l>(MutexGuard<'l, EdgeMemory>);

pub static GLOBAL_EDGE_CALLER: QemuGlobalEdgeCaller =
    QemuGlobalEdgeCaller(Mutex::new(EdgeMemory::new()));

impl<'h> GlobalEdgeCaller<'h> for QemuGlobalEdgeCaller {
    type Holder = QemuEdgeCallerHolder<'h>;

    fn acquire(&self) -> QemuEdgeCallerHolder {
        QemuEdgeCallerHolder(self.0.try_lock().unwrap())
    }
}

impl EdgeCallerHolder for QemuEdgeCallerHolder<'_> {
    fn edge_mem(&mut self) -> &mut EdgeMemory {
        &mut self.0
    }

    unsafe fn edge_call(&mut self) {
        todo!()
    }
}
