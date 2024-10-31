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

const KERNEL_IMAGE_NAME: &str = "kernel";
const UEFI_EXTENSION: &str = "_uefi";

fn main() {
    let kernel_path = Path::new(KERNEL_IMAGE_NAME);
    let uefi_kernel_path = [KERNEL_IMAGE_NAME, UEFI_EXTENSION].concat();
    let uefi_boot = UefiBoot::new(&kernel_path);
    let bootable_kernel_path = Path::new(&uefi_kernel_path);

    uefi_boot
        .create_disk_image(&bootable_kernel_path)
        .expect("Failed to create a UEFI-enabled version of your kernel image");
}
