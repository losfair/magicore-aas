OUTPUT_ARCH(riscv)

ENTRY(rust_main)

SECTIONS
{
    . = 0x11000000;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    .bss : {
        *(.sbss .bss .bss.*)
    }

    /DISCARD/ : { *(.eh_frame) }
}