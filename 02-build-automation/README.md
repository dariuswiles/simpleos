# Automate Building and Running the Kernel

The objective is to reduce the six commands it currently takes to build and run the minimal kernel to a single command.

## Starting Point

The starting point is the previous phase, where there are two separate Rust directories:

* _kernel_ - a kernel consisting of placeholder code that loops forever when run.
* _add_uefi_boot_ - an executable that takes a non-bootable kernel and adds the required bits to allow it to be booted using the UEFI protocol.

It currently requires multiple steps to get the kernel source code running in a Qemu environment.


## Cargo Workspaces

Cargo workspaces can be used to create dependencies between separate cargo projects. As the kernel source is the star of the show, move the entire contents of the the _kernel_ directory to the top-level directory (i.e., its parent), then remove the now-empty directory:

```bash
mv kernel/* .
rmdir kernel/
```

Add the following lines to the _Cargo.toml_ file that is now in the top-level directory to declare the _add_uefi_boot_ project as a workspace member:

```toml
[workspace]
members = [
    "add_uefi_boot",
]
resolver = "2"
```

## Artifact Dependency

Cargo's binary artifact dependency functionality can be used to determine the name of the kernel object and embed it in the _add_uefi_boot_ code. This avoids the need to copy the kernel so _add_uefi_boot_ can find it. To indicate that _add_uefi_boot_ depends on the kernel being built first, add the following to the existing dependency section in _add_uefi_boot/Config.toml_:

```toml
kernel = { path = "..", artifact = "bin", target = "x86_64-unknown-none" }
```

and add the following lines to create a new section for dependencies for the build system, and add the same dependency on the kernel:

```toml
[build-dependencies]
kernel = { path = "..", artifact = "bin", target = "x86_64-unknown-none" }
```

Cargo sets an environment variable named _CARGO_BIN_FILE_KERNEL_kernel_ to the file path and name of the object file built from the kernel source code. This is set at the time Cargo builds the _add_uefi_boot_ executable, and the **env!** macro can be used within _add_uefi_boot_ to obtain the value of this. This is used to read the kernel object file and output output a UEFI-enabled version to the same directory.

Modify the existing _add_uefi_boot_ source code by:

* Removing the line setting the **KERNEL_IMAGE_NAME** constant.
* Adding a constant holding the location of the OVMF firmware provided by the **ovmf** package on the operating system:
```rust
    const UEFI_FIRMWARE_PATH: &str = "/usr/share/ovmf/OVMF.fd";
```
* Adding the following line as the first line of **main()** to obtain the file path and name of the kernel object file:
```rust
    let kernel_path_env: &'static str = env!("CARGO_BIN_FILE_KERNEL_kernel");
```
* Replacing the lines defining **kernel_path** and **uefi_kernel_path** with:
```rust
    let uefi_kernel_path = [kernel_path_env, UEFI_EXTENSION].concat();
    let kernel_path = Path::new(kernel_path_env);
```
* Adding the following lines at the end of **main()** to display the qemu command that runs the UEFI-enabled kernel:  
```rust
    println!("Successfully created a UEFI-enabled version of the kernel image. Run with:");
    print!("qemu-system-x86_64 -bios {UEFI_FIRMWARE_PATH} ");
    println!("-drive file={},format=raw,index=0,media=disk", bootable_kernel_path.display());
```

Attempting to run with:
```bash
cargo run -p add_uefi_boot
```

results in an error, the last lines of which are:
```
Caused by:
 `artifact = …` requires `-Z bindeps` (kernel)
```

This is because Cargo's binary artifact dependency functionality is unstable at the time of writing. Fix the problem by enabling this functionality in Cargo's configuration by running:
```bash
mkdir .cargo
cat >.cargo/config.toml <<EOF
[unstable]
bindeps = true
EOF
```

Attempting to run the command again:
```bash
cargo run -p add_uefi_boot
```

is now successful and the qemu command it outputs can now be entered to boot the kernel image.


## Running qemu Automatically

Rather than _add_uefi_boot_ displaying the qemu command to run, its code can be improved to run qemu automatically. Add the following import near the top of the file:
```rust
use std::process::Command;
```

and replace the _println_ and _print_ statements that display the qemu command with:
```rust
    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-bios").arg(UEFI_FIRMWARE_PATH);
    cmd.arg("-drive").arg(format!("file={},format=raw,index=0,media=disk", bootable_kernel_path.display()));

    let mut child = cmd.spawn().expect("Failed to run 'qemu' on the bootable kernel image");
    child.wait().expect("qemu terminated with an exit status indicating a failure");
```


Now the kernel can be built, UEFI-enabled and run by entering:
```rust
cargo run -p add_uefi_boot
```

## Summary

No changes were made to the kernel in this phase, but the build system was automated so that the kernel can be built, UEFI-enabled and run with qemu using a single command.
