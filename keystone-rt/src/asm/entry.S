    .section .text.entry
    .global _start

_start:
    la      sp, _stack_end
    jal     rt_init
