use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use x86_64::instructions::port::PortWriteOnly;

#[derive(Clone, Copy)]
#[repr(u32)]
#[allow(unused)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {
        let mut port = PortWriteOnly::new(0xF4);
        port.write(exit_code as u32);
    }
}

fn new_mutex_serial(port_base: u16) -> Mutex<SerialPort> {
    let mut serial_port = unsafe { SerialPort::new(port_base) };
    serial_port.init();
    Mutex::new(serial_port)
}

lazy_static! {
    pub static ref SERIAL_DBG: Mutex<SerialPort> = new_mutex_serial(0x3F8);
    pub static ref SERIAL_EDGE: Mutex<SerialPort> = new_mutex_serial(0x2F8);
}

#[macro_export]
macro_rules! dbg_print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        write!($crate::qemu::SERIAL_DBG.lock(), $($args)+).unwrap()
    });
}

#[macro_export]
macro_rules! dbg_println {
    () => ({
        $crate::dbg_print!("\n")
    });
    ($fmt:expr) => ({
        $crate::dbg_print!(concat!($fmt, "\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        $crate::dbg_print!(concat!($fmt, "\n"), $($args)+)
    });
}
