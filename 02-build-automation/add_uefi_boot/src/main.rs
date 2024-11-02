/// Adds UEFI information to a kernel file to make it bootable via UEFI.
///
/// The original kernel file must be named "kernel" and must be located in the directory this
/// command is run from. A UEFI-enabled copy of this file is saved in the same directory and named
/// "kernel_uefi".
///
/// This file can be run with:
/// `qemu-system-x86_64 -bios /usr/share/ovmf/OVMF.fd -drive file=kernel_uefi,format=raw,index=0,media=disk`
///
/// The "OVMF" part should point to the UEFI implementation you wish to use for booting. The path
/// is correct for Kubuntu 24.04 with the ovmf package installed.

use bootloader::UefiBoot;
use std::path::Path;
use std::process::Command;

const UEFI_EXTENSION: &str = "_uefi";

fn main() {
    let kernel_path_env: &'static str = env!("CARGO_BIN_FILE_KERNEL_kernel");
    let uefi_kernel_path = [kernel_path_env, UEFI_EXTENSION].concat();
    let kernel_path = Path::new(kernel_path_env);
    let uefi_boot = UefiBoot::new(&kernel_path);
    let bootable_kernel_path = Path::new(&uefi_kernel_path);

    uefi_boot
        .create_disk_image(&bootable_kernel_path)
        .expect("Failed to create a UEFI-enabled version of your kernel image");

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-bios").arg("/usr/share/ovmf/OVMF.fd");
    cmd.arg("-drive").arg(format!("file={},format=raw,index=0,media=disk", bootable_kernel_path.display()));

    let mut child = cmd.spawn().expect("Failed to run 'qemu' on the bootable kernel image");
    child.wait().expect("qemu terminated with an exit status indicating a failure");
}
