//! The COM serial port for x86 architecture.

use x86_64::structures::port::{PortRead, PortWrite};

pub const COM1_PORT_BASE: u16 = 0x3F8;
pub const COM2_PORT_BASE: u16 = 0x2F8;

pub struct IsaSerialPort {
    port_base: u16,
}

impl IsaSerialPort {
    pub const fn new_uninit(port_base: u16) -> IsaSerialPort {
        IsaSerialPort { port_base }
    }

    pub unsafe fn init(&self) -> Result<(), &'static str> {
        u8::write_to_port(self.port_base + 1, 0x00); // Disable all interrupts
        u8::write_to_port(self.port_base + 3, 0x80); // Enable DLAB (set baud rate divisor)
        u8::write_to_port(self.port_base + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
        u8::write_to_port(self.port_base + 1, 0x00); //                  (hi byte)
        u8::write_to_port(self.port_base + 3, 0x03); // 8 bits, no parity, one stop bit
        u8::write_to_port(self.port_base + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
        u8::write_to_port(self.port_base + 4, 0x0B); // IRQs enabled, RTS/DSR set
        u8::write_to_port(self.port_base + 4, 0x1E); // Set in loopback mode, test the serial chip
        u8::write_to_port(self.port_base + 0, 0xAE); // Test serial chip (send byte 0xAE and check if serial returns same byte)

        // Check if serial is faulty (i.e: not same byte as sent)
        if u8::read_from_port(self.port_base + 0) != 0xAE {
            return Err("serial port is absent or faulty");
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        u8::write_to_port(self.port_base + 4, 0x0F);
        Ok(())
    }

    pub unsafe fn write(&self, ch: u8) {
        // wait for idle
        while u8::read_from_port(self.port_base + 5) & 0x20 == 0 {
            x86_64::instructions::hlt(); // does not halt actually
        }
        u8::write_to_port(self.port_base, ch);
    }
}
