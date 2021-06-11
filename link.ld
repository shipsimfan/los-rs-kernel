ENTRY(_start)
SECTIONS {
    __KERNEL_LMA = 0x100000;
    __KERNEL_VMA = 0xFFFF800000000000;
    __KERNEL_BOTTOM = __KERNEL_LMA + __KERNEL_VMA;
    
    . = __KERNEL_LMA;

    .text.low : AT(ADDR(.text.low)) {
        *(.text.low)
    }

    . = ALIGN (4096);
    .data.low : AT(ADDR(.data.low)) {
        *(.data.low)
    }

    . += __KERNEL_VMA;
    . = ALIGN(4096);

    .text : AT(ADDR(.text) - __KERNEL_VMA) {
        *(.text*)
    }

    . = ALIGN(4096);

    .data : AT(ADDR(.data) - __KERNEL_VMA) {
        *(.data*)
    }

    . = ALIGN(4096);

    .rodata : AT(ADDR(.rodata) - __KERNEL_VMA) {
        *(.rodata*)
    }

    . = ALIGN(4096);

    .eh_frame : AT(ADDR(.eh_frame) - __KERNEL_VMA) {
        *(.eh_frame*)
    }

    . = ALIGN(4096);

    .bss : AT(ADDR(.bss) - __KERNEL_VMA) {
        *(.bss*)
        *(COMMON)
    }

    . = ALIGN(4096);

    .got : AT(ADDR(.got) - __KERNEL_VMA) {
        *(.got*)
    }

    . = ALIGN(4096);

    __KERNEL_TOP = .;
    
    /DISCARD/ : {
        *(.comment)
    }
}

