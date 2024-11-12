#![no_main] // Prevents the compiler from "emitting the main symbol for an executable binary".
#![no_std] // Prevents the linking of Rust's standard library.

//! A freestanding kernel based on example code in the `bootloader` and `bootloader_api` crates, and
//! Philipp Oppermann's blog on writing a kernel in Rust at <https://os.phil-opp.com/>.
//!
//! It sends a message to the host OS's console from which QEMU was invoked, then loops forever.

use bootloader_api;
use core::panic::PanicInfo;
use spin::Mutex;
use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

// A single instance of a QEMU debugging console `Port`, protected against multiple accesses by a
// spinlock-based `Mutex`.
pub static QEMU_CONSOLE_PORT: Mutex<PortGeneric<u8, ReadWriteAccess>> = Mutex::new(Port::new(0xE9));

// Specifies the name of the function that should be invoked by the bootloader when it hands
// control to this code. The function name is arbitrary.
bootloader_api::entry_point!(simpleos_main);

/// The bootloader invokes this function at the end of its boot process when it is ready to hand
/// control to the kernel. This implementation simply loops forever.
fn simpleos_main(_bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    host_write("Hello world!\n");

    loop {}
}

/// Outputs the given string to QEMU's debug console. To see the output, the "-debugcon" argument
/// must be passed when QEMU is invoked. A newline is not appended to the string, so must be
/// included if desired.
fn host_write(s: &str) {
    for b in s.bytes() {
        unsafe {
            QEMU_CONSOLE_PORT.lock().write(b);
        }
    }
}

/// Rust requires a function with the "panic_handler" attribute [1] to be defined. This is usually
/// called if a panic occurs, except that this is overridden by the `panic = "abort"` lines in
/// Cargo.toml in this project to keep things simple. The function name is arbirary as only the
/// attribute is used to identify which function should be called.
///
/// [1]: https://doc.rust-lang.org/reference/runtime.html#the-panic_handler-attribute
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
