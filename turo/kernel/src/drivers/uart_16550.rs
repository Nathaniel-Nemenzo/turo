
use core::{fmt, panic};
use spin::Mutex;
use spin::once::Once;

use crate::arch::io;

static COM_1: Once<Mutex<SerialPort>> = Once::new();

///
/// init()
/// 
/// Initialize the COM_1 serial interface. This will be used for printing to the host terminal.
pub fn init() {
    unsafe {
        let s = SerialPort::new(0x3F8).init();
        COM_1.call_once(move || Mutex::new(s));
    }
}

///
/// SerialPort(u16)
/// 
/// Struct that represents a serial port. Allows for writing to a serial port at a given
/// port address.
#[repr(transparent)]
struct SerialPort(u16);

impl SerialPort {
    ///
    /// new(port: u16)
    /// 
    /// Creates a new, uninialized SerialPort with the given port address.
    pub const fn new(port: u16) -> Self {
        Self(port)
    }

    ///
    /// init()
    /// 
    /// Initializes the COM1 serial interface
    /// 
    /// This function is unsafe because the caller must make sure that the device 
    /// is initialized properly at the same port.
    pub unsafe fn init(self) -> Self {
        io::outb(self.0 + 1, 0x00);    // Disable all interrupts
        io::outb(self.0 + 3, 0x80);    // Enable DLAB (set baud rate divisor)
        io::outb(self.0 + 0, 0x03);    // Set divisor to 3 (lo byte) 38400 baud
        io::outb(self.0 + 1, 0x00);    //                  (hi byte)
        io::outb(self.0 + 3, 0x03);    // 8 bits, no parity, one stop bit
        io::outb(self.0 + 2, 0xC7);    // Enable FIFO, clear them, with 14-byte threshold
        io::outb(self.0 + 4, 0x0B);    // IRQs enabled, RTS/DSR set

        // Enable interrupts
        io::outb(self.0 + 1, 0x01);
        
        self
    }

    unsafe fn is_transmit_empty(&self) -> bool {
        return (io::inb(self.0 + 5) & 0x20) == 0
    }

    unsafe fn wait_for_transmit_empty(&self) {
        while self.is_transmit_empty() {
            core::hint::spin_loop();
        }
    }

    pub fn send_byte(&self, byte: u8) {
        unsafe {
            match byte {
                // Handle the case where backspace / delete
                8 | 127 => {
                    // Send the backspace to move the cursor back
                    self.wait_for_transmit_empty();
                    io::outb(self.0, 8);

                    // Send a ' ' to delete the character
                    self.wait_for_transmit_empty();
                    io::outb(self.0, b' ');

                    // Send a backspace to move the cursor back
                    self.wait_for_transmit_empty();
                    io::outb(self.0, 8);
                },
                _ => {
                    self.wait_for_transmit_empty();
                    io::outb(self.0, byte);
                }
            }
        }
    } 
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        for byte in s.bytes() {
            self.send_byte(byte)
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    if let Some(com) = COM_1.get() {
        unsafe {
            // disable interrupts
            asm!("cli");

            com.lock().write_fmt(args).expect("Printing to serial failed.");

            // enable interrupts
            asm!("sti");
        }
    };
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::drivers::uart_16550::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}