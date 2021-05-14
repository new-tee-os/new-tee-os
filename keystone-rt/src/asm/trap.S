    .equ REGBYTES, 8
    .macro STORE reg, offset
        sd  \reg, \offset * REGBYTES(sp)
    .endm
    .macro LOAD reg, offset
        ld  \reg, \offset * REGBYTES(sp)
    .endm

    .text
    .global trap_entry

trap_entry:
    # save user sp and load kernel sp
    csrrw   sp, sscratch, sp
    bnez    sp, 1f
    # double fault occurred, keep sp unchanged
    csrrw   sp, sscratch, zero
1:
    # save context
    addi    sp, sp, -16 * REGBYTES
    STORE   ra, 0
    STORE   t0, 1
    STORE   t1, 2
    STORE   t2, 3
    STORE   t3, 4
    STORE   t4, 5
    STORE   t5, 6
    STORE   t6, 7
    STORE   a0, 8
    STORE   a1, 9
    STORE   a2, 10
    STORE   a3, 11
    STORE   a4, 12
    STORE   a5, 13
    STORE   a6, 14
    STORE   a7, 15

    # call Rust's trap handler
    mv      a0, sp
    call    trap_handler

    # restore context
    LOAD    ra, 0
    LOAD    t0, 1
    LOAD    t1, 2
    LOAD    t2, 3
    LOAD    t3, 4
    LOAD    t4, 5
    LOAD    t5, 6
    LOAD    t6, 7
    LOAD    a0, 8
    LOAD    a1, 9
    LOAD    a2, 10
    LOAD    a3, 11
    LOAD    a4, 12
    LOAD    a5, 13
    LOAD    a6, 14
    LOAD    a7, 15
    addi    sp, sp, 16 * REGBYTES

    # save kernel sp and load user sp
    csrrw   sp, sscratch, sp
    sret
