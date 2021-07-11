    .equ REGBYTES, 8
    .macro STORE reg, offset
        sd  \reg, \offset * REGBYTES(a0)
    .endm
    .macro LOAD reg, offset
        ld  \reg, \offset * REGBYTES(a1)
        sd  zero, \offset * REGBYTES(a1)
    .endm
    .macro TPSTORE reg, offset
        sd  \reg, \offset * REGBYTES(tp)
    .endm
    .macro TPLOAD reg, offset
        ld  \reg, \offset * REGBYTES(tp)
        sd  zero, \offset * REGBYTES(tp)
    .endm

    .text
    .global ktask_enter
    .global ktask_leave
    .global task_entry

ktask_enter:
    STORE   sp,   0
    STORE   ra,   1
    STORE   tp,   2
    STORE   s0,   3
    STORE   s1,   4
    STORE   s2,   5
    STORE   s3,   6
    STORE   s4,   7
    STORE   s5,   8
    STORE   s6,   9
    STORE   s7,  10
    STORE   s8,  11
    STORE   s9,  12
    STORE   s10, 13
    STORE   s11, 14

    LOAD    sp,   0
    LOAD    ra,   1
    LOAD    tp,   2
    LOAD    s0,   3
    LOAD    s1,   4
    LOAD    s2,   5
    LOAD    s3,   6
    LOAD    s4,   7
    LOAD    s5,   8
    LOAD    s6,   9
    LOAD    s7,  10
    LOAD    s8,  11
    LOAD    s9,  12
    LOAD    s10, 13
    LOAD    s11, 14

    ## if we're in a Task, save the context pointers to tp
    beqz    tp, 1f
    TPSTORE a0, 2
    TPSTORE a1, 3

1:
    ret

ktask_leave:
    # load the context pointers from tp
    TPLOAD  a0, 3
    TPLOAD  a1, 2
    # switch back to the scheduler
    j       ktask_enter

task_entry:
    # switch kernel sp and user sp
    TPSTORE sp, 1
    TPLOAD  sp, 0

    # write kernel tp to sscratch
    csrw    sscratch, tp
    # avoid exposing kernel tp
    mv      tp, x0
    # jump to user main
    sret
