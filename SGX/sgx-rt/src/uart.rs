use core::fmt::Write;

pub struct UnsafeUart;

impl Write for UnsafeUart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let nr=1;
        match crate::syscall::SYSCALL_MAP.get(&nr).map(|&f| f) {
            Some(crate::syscall::SyscallHandler::Syscall3(f))=>unsafe{
                f(0,s.as_ptr() as usize,s.len() as usize);
            },
            _=>panic!("Not a write syscall!"),
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