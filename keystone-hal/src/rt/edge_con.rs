use core::fmt::Write;

use super::{sbi, EDGE_MEM_BASE};
use crate::edge::EdgeCallReq;

pub struct EdgeConsole;

unsafe fn print_buffer_once(msg: &[u8]) {
    let edge_memory = &mut *EDGE_MEM_BASE;
    edge_memory.req = EdgeCallReq::EdgeCallPrint.into();
    edge_memory.write_buffer(msg);
    sbi::stop_enclave(sbi::STOP_EDGE_CALL_HOST);
}

pub unsafe fn print_str(msg: &str) {
    for chunk in msg.as_bytes().chunks(crate::cfg::EDGE_BUFFER_SIZE) {
        print_buffer_once(chunk);
    }
}

impl Write for EdgeConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            print_str(s);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        write!($crate::edge_con::EdgeConsole, $($args)+).unwrap()
    });
}

#[macro_export]
macro_rules! println {
    () => ({
        $crate::print!("\n")
    });
    ($fmt:expr) => ({
        $crate::print!(concat!($fmt, "\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        $crate::print!(concat!($fmt, "\n"), $($args)+)
    });
}
