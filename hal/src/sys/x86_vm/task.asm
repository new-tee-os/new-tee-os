    .text
    .global ktask_enter
    .global ktask_leave

ktask_enter:
    # save context
    mov     [rdi + 0x00], rsp
    mov     [rdi + 0x08], rbp
    mov     [rdi + 0x10], rbx
    mov     [rdi + 0x18], r12
    mov     [rdi + 0x20], r13
    mov     [rdi + 0x28], r14
    mov     [rdi + 0x30], r15

    # save GS base
    mov     ecx, 0xC0000101
    rdmsr
    mov     [rdi + 0x38], eax
    mov     [rdi + 0x3C], edx

    # load context
    mov     rsp, [rsi + 0x00]
    mov     rbp, [rsi + 0x08]
    mov     rbx, [rsi + 0x10]
    mov     r12, [rsi + 0x18]
    mov     r13, [rsi + 0x20]
    mov     r14, [rsi + 0x28]
    mov     r15, [rsi + 0x30]

    # load GS base
    mov     eax, [rsi + 0x38]
    mov     edx, [rsi + 0x3C]
    wrmsr

    test    eax, eax
    jnz     has_task_ctx
    test    edx, edx
    jz      no_task_ctx

has_task_ctx:
    ## if we are in a Task, save the context pointers to GS
    mov     gs:[0x08], rdi
    mov     gs:[0x10], rsi

no_task_ctx:
    ret

ktask_leave:
    # load the context pointers from GS
    mov     rdi, gs:[0x10]
    mov     rsi, gs:[0x08]

    jmp     ktask_enter
