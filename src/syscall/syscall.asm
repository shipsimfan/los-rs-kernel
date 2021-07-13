EXTERN get_kernel_stack_pointer_and_save_user_stack_pointer
EXTERN get_user_stack_pointer
EXTERN system_call

system_call_handler:
    cli ; Until we enter the kernel stack

    ; Save registers
    ; RCX contains RIP, R11 contains RFLAGS
    push rcx
    push r11

    ; Move parameter
    mov rcx, r10

    ; Get kernel stack pointer and save user stack 
    mov rax, rsp
    push rdi
    push rsi
    push rdx
    push rcx
    push r8
    push r9
    mov rdi, rax
    call get_kernel_stack_pointer_and_save_user_stack_pointer
    pop r9
    pop r8
    pop rcx
    pop rdx
    pop rsi
    pop rdi

    mov rsp, rax
    sti

    ; Perform Syscall
    call system_call

    ; Get user stack pointer
    cli
    push rax
    call get_user_stack_pointer
    mov rdi, rax
    pop rax
    mov rsp, rdi

    ; Restore registers (R11 -> RFLAGS, RCX -> RIP)
    pop r11
    pop rcx

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