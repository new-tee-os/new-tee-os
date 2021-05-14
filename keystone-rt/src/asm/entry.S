    .section .text.init
    .global _start

_start:
    la      sp, _stack_end
    la      t0, trap_entry
    csrw    stvec, t0
    jal     rt_init
