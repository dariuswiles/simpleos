# SimpleOS

This git repository contains a very basic kernel that I wrote purely to learn more about low-level kernel development. It's capable of running in a qemu virtual environment. It's written in the Rust programming language and is inspired by Philipp Oppermann's [__Writing an OS in Rust__ blog](https://os.phil-opp.com/), but whereas Philipp's code boots from BIOS, this code attempts to achieve similar goals with a kernel booted via UEFI. Some techniques are based on examples in the [bootloader](https://docs.rs/bootloader/latest/bootloader/) and [bootloader_api](https://docs.rs/bootloader_api/latest/bootloader_api/) documentation, and other GitHub repos that also implement custom kernels from scratch.

The repository is separated into phases, each a self-contained Git project containing a working kernel. Each phase improves on the previous one, and the each has a README.md that explains the objectives of the phase and the modifications made to the code to achieve this.

## License

Everything in this repository is released under _The Unlicense_. See [LICENSE](LICENSE) for the license text, or https://unlicense.org/ for more details.
