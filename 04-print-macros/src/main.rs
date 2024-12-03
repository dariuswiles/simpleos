#![no_main] // Prevents the compiler from "emitting the main symbol for an executable binary".
#![no_std] // Prevents the linking of Rust's standard library.

//! A freestanding kernel based on example code in the `bootloader` and `bootloader_api` crates, and
//! Philipp Oppermann's blog on writing a kernel in Rust at <https://os.phil-opp.com/>.
//!
//! It sends a message to the QEMU's debugging console port via `print` and `println` macros which
//! are designed to work in the same way as their namesakes in Rust's standard library. QEMU can be
//! configured via command line options to send data received over its debugging console port to
//! various destinations. For this project, the intention is to direct data to the terminal from
//! which QEMU is invoked. After sending data, the kernel loops forever.

use core::panic::PanicInfo;
use spin::Mutex;
use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

mod qemu_console;

// A single instance of a QEMU debugging console `Port`, protected against multiple accesses by a
// spinlock-based `Mutex`.
pub static QEMU_CONSOLE_PORT: Mutex<PortGeneric<u8, ReadWriteAccess>> = Mutex::new(Port::new(0xE9));

// Specifies the name of the function that should be invoked by the bootloader when it hands
// control to this code. The function name is arbitrary.
bootloader_api::entry_point!(simpleos_main);

/// The bootloader invokes this function at the end of its boot process when it is ready to hand
/// control to the kernel. This implementation simply loops forever.
fn simpleos_main(_bootinfo: &'static mut bootloader_api::BootInfo) -> ! {
    let n = 1234;
    let arr = [2.6, f64::NAN, -10.3];
    print!("Printing integer '{n}' and array of floats {:?} with no newline. ", arr);

    const S: &str = "a slice";
    println!("Test printing slice '{S}' with a newline.");
    println!("Test printing slice '{}' with a newline.", S);
    println!();
    println!();
    println!("{}", "Two blank lines should be printed above this line");

    #[allow(clippy::empty_loop)]
    loop {}
}

/// Rust requires a function with the "panic_handler" attribute [1] to be defined. This is usually
/// called if a panic occurs, except that this is overridden by the `panic = "abort"` lines in
/// Cargo.toml in this project to keep things simple. The function name is arbirary as only the
/// attribute is used to identify which function should be called.
///
/// This function prints a message indicating that the kernel has panicked and the debug output
/// of the `PanicInfo` object passed, which includes the panic message and the line of code where
/// the panic occurred.
///
/// [1]: https://doc.rust-lang.org/reference/runtime.html#the-panic_handler-attribute
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    println!("\nKERNEL PANIC");
    println!("{panic_info:#?}");

    #[allow(clippy::empty_loop)]
    loop {}
}
