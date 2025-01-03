# Display Data on the Host OS

The objective is to send data from the kernel to the host's console. This uses QEMU's debugging console feature. This will be used in later phases to display data until transitioning to using the framebuffer.

## QEMU Debugging Console

QEMU has a simple debugging feature that, when enabled, connects to the I/O port at address 0xE9. I/O ports live in a 16-bit address space that is separate from physical memory, and which can only be written to using the x86 assembler **out** instruction. The port is a byte in length, and each byte written is passed by QEMU to the host OS and can be output to stdio, a graphical display, or a file. This project outputs to stdio, which displays the data in the console session in which qemu was invoked.

## Output One Byte

Rust allows assembler code to be included inline in Rust programs by using the **asm!** macro. This is an unsafe operation as there are many ways to destabilize a system if a write is made to the wrong memory location. It is possible to use inline assembler in the code to achieve this, e.g., to output the character "H" and a newline the code could look like:

```rust
    // In the src/main.rs file, within the simpleos_main() function before the "loop {}" statement

    let h: u8 = 0x48; // ASCII 'H'

    unsafe {
        core::arch::asm! {
            "out 0xE9, {}",
            in(reg_byte) h,
        };
    }
```

This code sets the variable _h_ to the ASCII code for 'H', then passes it to the assembler code in a register. **reg_byte** is used rather than **reg** to indicate the value is a single byte. This is required as QEMU's I/O port is byte-sized. This approach works, but the **x86-64** crate nicely abstracts this functionality with the additional safety of indicating to the compiler that it should not discard the write to the I/O port. This might happen if the compiler sees that data is written to an I/O port address that is never read. To use this crate, first add it as a dependency:
```toml
x86_64 = "0.15"
```

Then add the following to the kernel code that will output an 'H'.

```rust
    // In the src/main.rs file, within the simpleos_main() function before the "loop {}" statement
    let mut qemu_console_port = Port::new(0xE9);

    unsafe { qemu_console_port.write(0x48u8); } // ASCII 'H'
```

This creates a new `Port` at the address of QEMU's debugging console, then writes a `u8` value for ASCII 'H'. The code will not generate any output yet because the debugging console also needs to be enabled in QEMU. QEMU is invoked by the code in **add_uefi_boot**, so add the following code:

```rust
    // In the add_uefi_boot/main.rs file, within the main() function after the other "cmd.arg" lines

    cmd.arg("-debugcon").arg("stdio");
```

Run the kernel with:
```bash
cargo run -p add_uefi_boot
```

and it should output a single 'H'.

## Output A String


Add the following new function to the kernel source code, based on the earlier code that outputs a single character, to output a string:

```rust
// In the src/main.rs file

fn host_write(s: &str) {
    let mut qemu_console_port = Port::new(0xE9);
    
    for b in s.bytes() {
        unsafe { 
            qemu_console_port.write(b); 
        }
    }
}
```

Replace the code in the **simpleos_main()** function that outputs a single 'H' with the following code that uses the new function to output a string:
```rust
    // In the src/main.rs file, within the simpleos_main() function

    host_write("Hello world!\n");
```

Run the kernel with:
```bash
cargo run -p add_uefi_boot
```

and it should now output "Hello world!" and a newline.

## Create the Port Only Once

The current code recreates the same `Port` for each string written, which is wasteful and which could cause issues if the kernel is improved to be multi-threaded and multiple threads output simultaneously. The first problem could be solved by creating a single `Port` as a mutable static, e.g.:
```rust
    // In the src/main.rs file

    pub static mut QEMU_CONSOLE_PORT: PortGeneric<u8, ReadWriteAccess> = Port::new(0xE9);
```

However, this results in a compiler warning that "creating a mutable reference to mutable static is discouraged" with additional information stating that "mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives". This is the second problem identified above.

A `Mutex` is a typical way of implementing locking that would fix the problem, but it is part of the standard library, so unavailable. One solution is to use a much simpler locking mechanism called a _spinlock_, which enters a tight loop waiting for a resource to become available rather than implementing the more sophisticated system used by `Mutex`. This technique is described in this section on [Philipp Oppermann's blog](https://os.phil-opp.com/vga-text-mode/#spinlocks). To create a single `Port` for the debugging console that is protected by a spinlock-based Mutex and which lasts the lifetime of the program, first add the following line to the kernel:


```rust
    // In the src/main.rs file
    pub static QEMU_CONSOLE_PORT: Mutex<PortGeneric<u8, ReadWriteAccess>> = Mutex::new(Port::new(0xE9));
```

Then change the code that writes to the debugging port to:
```rust
        unsafe {
            QEMU_CONSOLE_PORT.lock().write(b);
        }
```

This attempts to lock the debugging console port, and when the lock succeeds, the string passed to the function is written to the debugging port one byte at a time.

## Alternative Implementation

QEMU offers a virtual serial port device which can be used in a similar way to QEMU's debugging console to output data. However, it is more complex to setup and use, and has a fixed speed, so is typically slower. On the plus side, it is bi-directional, i.e., data can be sent from the host OS to the guest kernel. See [Philipp Oppermann's blog](https://os.phil-opp.com/testing/#serial-port) for more details.

## Summary

Strings can now be output to the host's console via QEMU's debugging console feature. The next phase modifies and improves this to allow formatted strings to be output.
