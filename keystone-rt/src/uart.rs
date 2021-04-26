use core::fmt::Write;

use crate::sbi;

pub struct UnsafeUart;

impl Write for UnsafeUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            sbi::putchar(b);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        write!(crate::uart::UnsafeUart, $($args)+).unwrap()
    });
}

#[macro_export]
macro_rules! println {
    () => ({
        crate::print!("\n")
    });
    ($fmt:expr) => ({
        crate::print!(concat!($fmt, "\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        crate::print!(concat!($fmt, "\n"), $($args)+)
    });
}
