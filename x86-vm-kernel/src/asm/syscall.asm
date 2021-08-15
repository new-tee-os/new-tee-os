    .text
    .global syscall_entry

syscall_entry:
    # save user sp & load kernel sp
    swapgs
    xchg    gs:[0], rsp

    # save clobbered registers
    # push    rax
    push    rcx
    push    rdx
    push    rsi
    push    rdi
    push    r8
    push    r9
    push    r10
    push    r11

    # construct C ABI arguments
    mov     rcx, rax

    # jump to Rust code
    call    handle_syscall
    # the return value is stored in rax

    # restore registers
    pop     r11
    pop     r10
    pop     r9
    pop     r8
    pop     rdi
    pop     rsi
    pop     rdx
    pop     rcx
    # pop     rax

    # save kernel sp & load user sp
    xchg    gs:[0], rsp
    swapgs

    # return to userspace
    sysret
