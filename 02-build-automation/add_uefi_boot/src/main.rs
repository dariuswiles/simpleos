/// Adds UEFI information to a kernel file to make it bootable via UEFI.
///
/// The kernel source needs to be compiled before it can be made bootable and this must be done
/// using Cargo's binary artifact dependency functionality so that its location is set in an
/// environment variable before this file is built. The UEFI-enabled kernel is saved in the same
/// directory as the kernel object and has the same name with "_uefi" appended.
use bootloader::UefiBoot;
use std::path::Path;
use std::process::Command;

const UEFI_EXTENSION: &str = "_uefi";
const UEFI_FIRMWARE_PATH: &str = "/usr/share/ovmf/OVMF.fd"; // Set to location of OVMF firmware

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
    cmd.arg("-bios").arg(UEFI_FIRMWARE_PATH);
    cmd.arg("-drive").arg(format!(
        "file={},format=raw,index=0,media=disk",
        bootable_kernel_path.display()
    ));

    let mut child = cmd
        .spawn()
        .expect("Failed to run 'qemu' on the bootable kernel image");
    child
        .wait()
        .expect("qemu terminated with an exit status indicating a failure");
}
