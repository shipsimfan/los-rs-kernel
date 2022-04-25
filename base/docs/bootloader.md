# Bootloader
Contains definitions of information passed to the kernel by the bootloader.

Two main pieces of information are passed to the kernel by the bootloader:
 1. GraphicsMode - Contains information about the UEFI framebuffer for boot video.
 2. MemoryMap - Contains information about the current state of memory in the system.