%macro PushAllRegisters 0
    push rax
    push rbx
    push rcx
    push rdx
    push rsi
    push rdi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
%endmacro

EXTERN CURRENT_KERNEL_STACK

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

    PushAllRegisters

    ; Retrieve moved parameter
    mov rcx, r10

    ; Perform Syscall
    call system_call

    ; Pop signal context
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rdi
    pop rsi
    pop rdx
    pop rcx
    pop rbx
    pop rcx ; Need to remove pushed RAX without losing returned value

    ; Retrieve required registers
    pop r11 ; User flags
    pop rcx ; Return address

    cli

    ; Get user stack
    pop rsp

    o64 sysret