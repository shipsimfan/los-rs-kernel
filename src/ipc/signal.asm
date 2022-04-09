EXTERN LOCAL_CRITICAL_COUNT

GLOBAL handle_userspace_signal
handle_userspace_signal:
    cli

    ; Leave local critical
    mov rbx, LOCAL_CRITICAL_COUNT
    xor rax, rax
    mov [rbx], rax

    ; Setup sysret registers
    mov rcx, rsi   ; Return address
    mov r11, 0x202 ; RFLAGS
    mov rsp, rdi   ; Stack
    mov rdi, rdx   ; Signal number

    ; Return to userspace
    o64 sysret