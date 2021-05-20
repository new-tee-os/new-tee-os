    .section .text.init
    .global _start

_start:
    la      sp, _stack_end
    la      t0, trap_entry
    csrw    stvec, t0
    # call vm_init to re-initialize page tables
    jal     vm_init
    # start the kernel
    jal     rt_main
