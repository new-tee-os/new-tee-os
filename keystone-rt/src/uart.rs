use core::fmt::Write;

use hal::arch::keystone::sbi;

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
macro_rules! uart_print {
    ($($args:tt)+) => ({
        use core::fmt::Write;
        write!($crate::uart::UnsafeUart, $($args)+).unwrap()
    });
}

#[macro_export]
macro_rules! uart_println {
    () => ({
        $crate::uart_print!("\n")
    });
    ($fmt:expr) => ({
        $crate::uart_print!(concat!($fmt, "\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        $crate::uart_print!(concat!($fmt, "\n"), $($args)+)
    });
}
