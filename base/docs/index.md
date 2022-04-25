# Base Module
The base module contains definitions that other modules rely on.

 1. [Bootloader](bootloader.md) - Definitions of information passed to the kernel by the bootloader
 2. [Critical](critical.md) - Controls whether the current thread can be interrupted
 3. [Error](error.md) - Provides a standard definition for errors in the kernel
 4. [Hash Map](hash_map.md) - Provides a hash map data structure which associates keys with values
 5. [Logging](logging.md) - Allows logging for modules in the kernel.
 6. [Map](map.md) - Provides a map data structure which automatically associates ids with values
 7. [Multi-Owner](multi_owner.md) - Provides a simple interface for items with multiple owners and references
 8. [Pinned Box](pinned_box.md) - Provides a simple interface over Pin\<Box\<T>>
 9. [Queue](queue.md) - Provides a simple first-in-first-out data structure