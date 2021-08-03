//! The COM serial port for x86 architecture.

use x86_64::structures::port::{PortRead, PortWrite};

const PORT_BASE: u16 = 0x3f8;

pub unsafe fn serial_init() -> bool {
    u8::write_to_port(PORT_BASE + 1, 0x00u8); // Disable all interrupts
    u8::write_to_port(PORT_BASE + 3, 0x80u8); // Enable DLAB (set baud rate divisor)
    u8::write_to_port(PORT_BASE + 0, 0x03u8); // Set divisor to 3 (lo byte) 38400 baud
    u8::write_to_port(PORT_BASE + 1, 0x00u8); //                  (hi byte)
    u8::write_to_port(PORT_BASE + 3, 0x03u8); // 8 bits, no parity, one stop bit
    u8::write_to_port(PORT_BASE + 2, 0xC7u8); // Enable FIFO, clear them, with 14-byte threshold
    u8::write_to_port(PORT_BASE + 4, 0x0Bu8); // IRQs enabled, RTS/DSR set
    u8::write_to_port(PORT_BASE + 4, 0x1Eu8); // Set in loopback mode, test the serial chip
    u8::write_to_port(PORT_BASE + 0, 0xAEu8); // Test serial chip (send byte 0xAE and check if serial returns same byte)

    // Check if serial is faulty (i.e: not same byte as sent)
    if u8::read_from_port(PORT_BASE + 0) != 0xAE {
        return false;
    }

    // If serial is not faulty set it in normal operation mode
    // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
    u8::write_to_port(PORT_BASE + 4, 0x0F);
    true
}

pub unsafe fn serial_write(ch: u8) {
    while u8::read_from_port(PORT_BASE + 5) & 0x20 == 0 {
        x86_64::instructions::hlt(); // does not halt actually
    }
    u8::write_to_port(PORT_BASE, ch);
}
