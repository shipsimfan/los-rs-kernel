SECTION .data

ALIGN 16
GLOBAL CURRENT_KERNEL_STACK
CURRENT_KERNEL_STACK: dq 0

SECTION .text

EXTERN system_call
GLOBAL system_call_handler
system_call_handler:
    ; Get kernel stack
    mov rax, CURRENT_KERNEL_STACK
    mov r12, rsp ; Prepare to save user stack pointer
    mov rsp, [rax] ; Switch stacks

    sti

    ; Save required registers
    push r12 ; User stack pointer
    push rcx ; Return address
    push r11 ; User flags

    ; Retrieve moved parameter
    mov rcx, r10

    ; Perform Syscall
    call system_call

    ; Retrieve required registers
    pop r11 ; User flags
    pop rcx ; Return address

    cli

    ; Get user stack
    pop rsp

    o64 sysret