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

%macro PopAllRegisters 0
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
    pop rax
%endmacro

GLOBAL float_save
float_save:
    fxsave [rdi]
    ret

GLOBAL float_load
float_load:
    fxrstor [rdi]
    ret

EXTERN switch_thread
EXTERN CURRENT_KERNEL_STACK

GLOBAL perform_yield
perform_yield:
    PushAllRegisters

    ; Switch stackes
    mov [rdi], rsp
    mov rsp, [rsi]

    ; Change syscall stack base
    mov rax, CURRENT_KERNEL_STACK
    mov [rax], rdx

    ; Switch bookkeeping
    call switch_thread

    PopAllRegisters

    ret

GLOBAL thread_enter_user
thread_enter_user:
    mov rbx, LOCAL_CRITICAL_COUNT
    mov rax, [rbx]
    dec rax
    mov [rbx], rax
    iretq

GLOBAL set_fs_base
set_fs_base:
    mov eax, edi
    shr rdi, 32
    mov edx, edi
    mov ecx, 0xC0000100
    wrmsr
    ret

SECTION .data

GLOBAL LOCAL_CRITICAL_COUNT
LOCAL_CRITICAL_COUNT: dq 0