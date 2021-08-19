OUTPUT_ARCH("riscv")

SECTIONS {
    . = 0x400000;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    . = DATA_SEGMENT_ALIGN (CONSTANT (MAXPAGESIZE), CONSTANT (COMMONPAGESIZE));
    .rodata : {
        *(.rodata .rodata.*)
    }
    . = ALIGN(8);
    .data : {
        *(.sdata .sdata.*)
        *(.data .data.*)
    }
    . = ALIGN(8);
    .bss : {
        _bss_start = .;
        *(.sbss .sbss.*) *(.bss .bss.*)
        _bss_end = .;
    }
    _end = .;
}
