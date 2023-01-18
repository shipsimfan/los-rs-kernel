# Init
This crate contains the runtime code which starts the operating system.

## Boot procedure
The kernel is given control of the system by the bootloader and begins executing at `_start` in boot.asm. The environment is then prepared for jumping to Rust as described [here](./boot.asm.md). The Rust part of the init crate then begins executing from kmain. The kernel's first goal is to get the fundemental parts of the system operating running.

After the fundemental components are started, the operating system can start the first real process: `init`. The goal of init is to initialize the early drivers required to identify and load other drivers from the boot drive and establish the first session. These drivers will eventually be loaded by the bootloader and passed to the kernel, but for now they are directly compiled into the kernel. After the early drivers are initialized, the `init` process will start the first session and execute the shell inside it. After that, it will load the remaining drivers.