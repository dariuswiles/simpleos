# Minimal Running Kernel

The purpose of this project is to create a tiny kernel that meets all the compiler requirements and which simply loops forever when invoked.

The code is separated into two Rust packages. The _kernel_ package compiles a freestanding executable that contains the code that loops forever. The _add_uefi_boot_ package creates an executable that, when run, adds the data required by the UEFI boot standard to the freestanding kernel executable.

A "nightly" version of the Rust toolchain is currently required for both as the "per-package-target" Cargo feature is an unstable feature. The "rust-toolchain" file in the top-level of each project directory forces its use.

# How to Build and Run the Kernel

The kernel code can be run by building it, copying it to the _add_uefi_boot_ project, running _add_uefi_boot_ to create a UEFI-enabled version of it, and then running this using qemu.

1. cd kernel
2. cargo build
3. cp target/x86_64-unknown-none/debug/kernel ../add_uefi_boot/
4. cd ../add_uefi_boot
5. cargo run
6. qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -drive file=kernel_uefi,format=raw,index=0,media=disk

# What's Next?

Executing six steps each time the kernel changes is hassle. The next project reduces this process to a single step.

# References

Philipp Oppermann's [__Writing an OS in Rust__ blog](https://os.phil-opp.com/) is the main inspiration for this project, but code is also based on examples in the [bootloader](https://docs.rs/bootloader/latest/bootloader/) and [bootloader_api](https://docs.rs/bootloader_api/latest/bootloader_api/) documentation.
