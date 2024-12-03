//! Defines `print!` and `println!` macros to send data to QEMU's debugging console.

use core::fmt::{self, Write};
use spin::Mutex;
use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

// A single instance of a QEMU debugging console `Port`, protected against multiple accesses by a
// spinlock-based `Mutex`.
pub static QEMU_CONSOLE_PORT: Mutex<PortGeneric<u8, ReadWriteAccess>> = Mutex::new(Port::new(0xE9));

struct HostWriter {}

impl Write for HostWriter {
    /// Outputs the given string to QEMU's debug console on the host. To see the output, the
    /// "-debugcon" argument must be passed to QEMU when it is invoked. This function is always
    /// successful so never returns an error.
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for b in s.bytes() {
            unsafe {
                QEMU_CONSOLE_PORT.lock().write(b);
            }
        }

        Ok(())
    }
}

/// Writes data to QEMU's debugging console. The passed data is of type `core::fmt::Arguments`
/// because this is the type: returned from the `format_args!` macro; and required by the `Write`
/// traits `write_fmt()` method.
///
/// This function is intended only for internal use, but is declared `pub` to allow its use from
/// macros.
//
// The implementation is closely based on <https://os.phil-opp.com/testing/#serial-port>.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut hw = HostWriter {};
    hw.write_fmt(args).unwrap();
}

/// An alternate implementation of the standard `print!` macro, except that output is sent to QEMU's
/// debugging console.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::qemu_console::_print(format_args!($($arg)*));
    }};
}

/// An alternate implementation of the standard `println!` macro, except that output is sent to
/// QEMU's debugging console.
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {{
        $crate::print!("{}\n", format_args!($($arg)*));
    }};
}
