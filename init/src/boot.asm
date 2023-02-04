.section .data.low

.align 4096
pml4: .fill 512

acpiRootPointer: .8byte 0

stackBottom: .fill 32768
stackTop:

.section .text.low

higherHalfLocation: .8byte higherHalf
kmainLocation: .8byte kmain

.global _start
_start:
    cli

    mov rdi, rcx
    mov rsi, rdx
    mov rdx, r8
    
    mov rbx, cr3
    mov r8, pml4
    mov rcx, 256

    .copyLow:
        mov rax, [rbx]
        mov [r8], rax
        add rbx, 8
        add r8, 8
        loop .copyLow

    mov rbx, cr3
    mov rcx, 256
    
    .copyHigh:
        mov rax, [rbx]
        mov [r8], rax
        add rbx, 8
        add r8, 8
        loop .copyHigh

    mov rax, pml4
    mov cr3, rax

    mov rax, qword ptr [higherHalfLocation]
    jmp rax

.section .text

higherHalf:
    mov rbx, 0xFFFF800000000000

    mov rsp, stackTop
    add rsp, rbx

    add rsp, rbx
    add rdi, rbx
    add rsi, rbx
    
    mov rax, cr0
    and ax, 0xFFFB
    or ax, 0x2
    mov cr0, rax
    mov rax, cr4
    or ax, 3 << 9
    mov cr4, rax

    mov rax, acpiRootPointer
    add rax, rbx
    mov [rax], rdx
    
    mov rax, qword ptr [kmainLocation]
    call rax