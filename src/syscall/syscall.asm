EXTERN system_call
EXTERN KERNEL_STACK_TOP

system_call_handler:
    cli ; Until we enter the kernel stack

    ; Get kernel stack pointer and save user stack 
    mov rax, rsp
    mov r9, KERNEL_STACK_TOP
    mov rsp, [r9]
    push rax
    sti

    ; Save registers
    ; RCX contains RIP, R11 contains RFLAGS
    push rcx
    push r11

    ; Move parameter
    mov rcx, r10

    ; Perform Syscall
    call system_call

    ; Restore registers (R11 -> RFLAGS, RCX -> RIP)
    pop r11
    pop rcx

    ; Get user stack pointer
    cli
    pop rsp

    ; Return
    o64 sysret

GLOBAL init_system_calls
init_system_calls:
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1
    wrmsr

    xor eax, eax
    mov edx, (0x10 << 16) | 0x08
    mov ecx, 0xC0000081
    wrmsr

    mov ecx, 0xC0000082
    mov rdx, system_call_handler
    mov rax, rdx
    shr rdx, 32
    wrmsr
    
    ret