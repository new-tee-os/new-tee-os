    .equ REGBYTES, 8

    .section .text.init
    .global _start

_start:
    la      sp, _stack_end
    la      t0, trap_entry
    csrw    stvec, t0
    # allocate space for struct VmInfo and pass it as the first argument
    addi    sp, sp, -3 * REGBYTES
    mv      a0, sp
    # call vm_init to re-initialize page tables
    jal     vm_init
    # start the kernel
    mv      a0, sp
    jal     rt_main
