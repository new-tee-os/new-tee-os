use core::fmt::Write;

use crate::edge::{with_edge_caller, EdgeCallReq};

pub struct EdgeConsole;

fn print_buffer_once(msg: &[u8]) {
    with_edge_caller(|caller| {
        caller
            .edge_mem()
            .write_request(EdgeCallReq::EdgeCallPrint)
            .write_buffer(msg);
        unsafe { caller.edge_call() };
    })
}

pub fn print_str(msg: &str) {
    for chunk in msg.as_bytes().chunks(crate::edge::EDGE_BUFFER_SIZE) {
        print_buffer_once(chunk);
    }
}

impl Write for EdgeConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        write!($crate::edge::EdgeConsole, $($args)+).unwrap()
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
