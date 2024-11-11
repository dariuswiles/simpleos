# SimpleOS

This git repository contains a very basic kernel that I wrote purely to learn more about low-level kernel development. It's designed to run in a QEMU virtual environment on an x86-64 host. It's written in the Rust programming language and is inspired by Philipp Oppermann's [__Writing an OS in Rust__ blog](https://os.phil-opp.com/), but whereas Philipp's code boots from BIOS, this code attempts to achieve similar goals with a kernel booted via UEFI. Some techniques are based on examples in the [bootloader](https://docs.rs/bootloader/latest/bootloader/) and [bootloader_api](https://docs.rs/bootloader_api/latest/bootloader_api/) documentation, and other GitHub repos that also implement custom kernels from scratch.

The repository is separated into phases, each a self-contained Git project containing a working kernel. Each phase improves on the previous one, and each has a README.md that explains the objectives of the phase and the modifications made to the code to achieve this.

## Kernels

| Phase name | Description |
| --- | --- |
| [01-minimal-kernel](01-minimal-kernel) | A tiny kernel that just loops forever. The main goal with this phase is to create a kernel that builds and runs. |
| [02-build-automation](02-build-automation) | The build system is automated so that the kernel can be built and run with qemu in a single command. No changes are made to the kernel. |
| [03-display-data-on-host](03-display-data-on-host) | Add the ability to output text to the host's console from which QEMU was run. |



## License

Everything in this repository is released under _The Unlicense_. See [LICENSE](LICENSE) for the license text, or https://unlicense.org/ for more details.
