use spin::{Mutex, MutexGuard};
use uart_16550::SerialPort;

use crate::edge::{EdgeCallerHolder, EdgeMemory, EdgeStream, GlobalEdgeCaller};

pub struct QemuGlobalEdgeCaller(Mutex<EdgeMemory>);
pub struct QemuEdgeCallerHolder<'l>(MutexGuard<'l, EdgeMemory>);
struct QemuEdgeStream<'l>(MutexGuard<'l, SerialPort>);

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
        let edge_mem = self.edge_mem();
        let mut edge_stream = QemuEdgeStream(crate::arch::x86_vm::qemu::SERIAL_EDGE.lock());
        // write request
        edge_mem.serialize(&mut edge_stream);
        // read result
        edge_mem.deserialize(&mut edge_stream);
    }
}

impl EdgeStream for QemuEdgeStream<'_> {
    fn read(&mut self) -> u8 {
        self.0.receive()
    }

    fn write(&mut self, ch: u8) {
        self.0.send_binary(ch);
    }
}
