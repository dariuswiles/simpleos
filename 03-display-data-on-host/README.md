# Display Data on the Host OS

The objective is to send data from the kernel to the host's console. This uses QEMU's debugging console feature. This will be used in later phases to display data until transitioning to using the framebuffer.

## QEMU Debugging Console

QEMU has a simple debugging feature that, when enabled, connects to the I/O port at address 0xE9. I/O ports live in a 16-bit address space that is separate from physical memory, and which can only be written to using the x86 assembler **out** instruction. The port is a byte in length, and each byte written is passed by QEMU to the host OS and can be output to stdio, a graphical display, or a file. This project outputs to stdio, which displays the data in the console session in which qemu was invoked.

## Output One Byte

Rust allows assembler code to be included inline in Rust programs by using the **asm!** macro. This is an unsafe operation as there are many ways of destabilizing a system. The first goal is to output the character "H" and a newline to check everything works. Add the following code to the kernel:

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

This sets the variable _h_ to the ASCII code for 'H', then passes it to the assembler code in a register. **reg_byte** is used rather than **reg** to indicate the value is a single byte. This is required as QEMU's I/O port is byte-sized. This code will not generate any output yet because the debugging console also needs to be enable in QEMU. QEMU is invoked by the code in **add_uefi_boot**, so add the following code:

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

Add the following new function to the kernel source code:

```rust
// In the src/main.rs file

fn host_write(s: &str) {
    for b in s.bytes() {
        unsafe {
            core::arch::asm! {
                "out 0xE9, {}",
                in(reg_byte) b
            }
        }
    }
}
```

Replace the code in the **simpleos_main()** function that outputs a single 'H' with the following code that outputs a string:
```rust
    // In the src/main.rs file, within the simpleos_main() function

    host_write("Hello world!\n");
```

Run the kernel with:
```bash
cargo run -p add_uefi_boot
```

and it should now output "Hello world!" and a newline.

## Alternative Implementation

QEMU also offers a virtual serial port device which is more complex to setup and use, and which has lower bandwidth that QEMU's debugging console. On the plus side, it is bi-directionaly, i.e., data can be send from the host OS to the guest kernel. See [Philipp Oppermann's blog](https://os.phil-opp.com/testing/#serial-port) for more details.

## Summary

Strings can now be output to the host's console via QEMU's debugging console feature. The next phase modifies and improves this to allow formatted strings to be output.
