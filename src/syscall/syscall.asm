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
    mov rdx, rsp ; Prepare to save user stack pointer
    mov rsp, [rax] ; Switch stacks

    ; Save required registers
    push rdx ; User stack pointer
    push rcx ; Return address
    push r11 ; User flags
    
    sti

    ; Perform Syscall
    call system_call

    cli

    ; Retrieve required registers
    pop r11 ; User flags
    pop rcx ; Return address

    ; Get user stack
    pop rsp

    o64 sysret