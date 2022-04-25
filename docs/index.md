# LOS Kernel
The LOS kernel is a monolithic kernel written in Rust for x86_64 architecture.

## Modules
The kernel is divided into a series of modules. Each module pertains to a certain function in the kernel.
 1. Kernel - Executable which controls the startup of flow of the kernel and performs the final compilation.
 2. [Base](../base/docs/index.md) - Contains fundemental definitions which many other modules rely upon.
 3. [Interrupts](../interrupts/docs/index.md) - Controls the execution of exceptions, IRQs, and system calls.