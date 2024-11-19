# Macros to Display Data on the Host OS

The code from the previous phase can send data from the kernel to the host's console using QEMU's debugging console feature. However, it would be more ergonomic to wrap this feature in __print!__ and __println!__ macros that work similarly to those in Rust's standard library. That is the objective of this phase.

This phase is closely based on 

Philipp Oppermann's [__Writing an OS in Rust__ blog](https://os.phil-opp.com/), specifically the section about in his [Testing blog about implementing a serial port](https://os.phil-opp.com/testing/#serial-port).

## `print!` macro

Rust's standard `print!` macro is defined as:

```rust
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::io::_print($crate::format_args!($($arg)*));
    }};
}
```

Add the slightly altered version below to the kernel.
```rust
// In the src/main.rs file
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::_print(format_args!($($arg)*));
    }};
}
```

A call to the macro will be replaced with a call to an internal `_print` function that will be tackled soon, passing all arguments passed to the macro to a single call to the `core::format_args!` macro. The latter handles all the conversions and substitutions required to replace "{}" in the format string with the other arguments supplied. This includes variations such as "{:?}", and "{variable_name}". The output of `format_args!` has a type of `fmt::Arguments`.

Rather than attempting to understand the `fmt::Arguments` type, a simple way of using it is to implement the `Write` trait, which provides a `write_fmt` method that takes this type. The only method required to implement the `Write` trait has the signature:
```rust
    fn write_str(&mut self, s: &str) -> Result;
```

The `host_write` method is close to what's required, except that it now needs to return a `Result`. It also needs to be a method in a new `struct` which implements `Write`. Performing all these changes converts the existing `host_write` method to:

```rust
// In src/main.rs to replace the host_write function with the following
struct HostWriter {}

impl Write for HostWriter {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for b in s.bytes() {
            unsafe {
                QEMU_CONSOLE_PORT.lock().write(b);
            }
        }

        Ok(())
    }
}
```

All that remains is to implement the `_print` function. This needs to pass the `fmt::Arguments` output from `format_args!` to the new `write_str` method. Add the following new function to the kernel:

```rust
// In src/main.rs 
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut hw = HostWriter {};
    hw.write_fmt(args).unwrap();
}
```

The _doc_ attribute suppresses the function from the documentation as the function is intended to only be used internally. The output from `write_fmt` will always return `Ok`, so it is safe to use `unwrap`.

## `println!` macro

A macro that appends a newline to the arguments passed, or outputs just a newline if no arguments are passed, can be based on Rust's standard `println!` macro. Add the slightly altered version below to the kernel.

```rust
#[macro_export]
// In src/main.rs 
macro_rules! println {
    () => (crate::print!("\n"));
    ($($arg:tt)*) => {{
        crate::print!("{}\n", format_args!($($arg)*));
    }};
}
```

This has two matching rules that handle the case of no arguments and some arguments respectively. Both call the `print!` macro created earlier. The _macro_export_ attribute defines the macro at the crate's root and makes the macro public.

## Testing the New Macros

The two new macros can now be tested by adding the following code to the kernel:

```rust
// In the simpleos_main function of src/main.rs
    let n = 1234;
    let arr = [2.6, f64::NAN, -10.3];
    print!("Printing integer '{n}' and array of floats {:?} with no newline. ", arr);

    const S: &str = &"a slice";
    println!("Test printing slice '{S}' with a newline.");
    println!("Test printing slice '{}' with a newline.", S);
    println!();
    println!();
    println!("{}", "Two blank lines should be printed above this line");
```

## Moving the New Code to a Separate Module

To stop the src/main.rs code getting cluttered with the console printing code, move the latter into a new file in the same directory with the following abbreviated contents:

```rust
// In new file src/qemu_console.rs

use core::fmt::{self, Write};
use spin::Mutex;
use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

pub static QEMU_CONSOLE_PORT: Mutex<PortGeneric<u8, ReadWriteAccess>> = Mutex::new(Port::new(0xE9));

struct HostWriter {}

impl Write for HostWriter {
    // contents removed for brevity
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut hw = HostWriter {};
    hw.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::qemu_console::_print(format_args!($($arg)*)); // Changed code
    }};
}

#[macro_export]
macro_rules! println {
    () => (crate::print!("\n"));
    ($($arg:tt)*) => {{
        crate::print!("{}\n", format_args!($($arg)*));
    }};
}
```

Note the change to the `print!` macro which now requires the name of the module added to the path to call `_print()`.

In addition to removing items moved to the new file, remove the following line from src/main.rs:
```rust
// In src/main.rs
use core::fmt::{self, Write}; // Delete this line
```

and add the following to the same file to pull in the new _qemu_console_ module:
```rust
// In src/main.rs
mod qemu_console;
```

## Display Panic Messages

The `panic()` function which currently just loops forever can become more useful by outputting the data that is passed to it using the new `println!` macro. Replace the `panic()`  as follows:

```rust
// Replace panic() in src/main.rs with
fn panic(panic_info: &PanicInfo) -> ! {
    println!("\nKERNEL PANIC");
    println!("{panic_info:#?}");

    loop {}
}
```

This change can be tested by temporarily adding code that causes a panic before the loop at the end of the `simpleos_main()`. For example:
```rust
    // In src/main.rs before the "loop {}" statement
    assert!(false, "panic message text");
```

## Summary

`print!` and `println!` macros that behave in the same way as their standard library counterparts were added, except that output is sent to QEMU's debugging console port. The `panic()` function was improved to make use of this to output debug information before looping forever.
