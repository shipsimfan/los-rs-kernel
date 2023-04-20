.section .data.low

.align 4096
PML4: .fill 4096

STACK_BOTTOM: .fill 524288

.global STACK_TOP
STACK_TOP:

.section .text.low

HIGHER_HALF_LOCATION: .8byte higherHalf
KMAIN_LOCATION: .8byte kmain

.global _start
_start:
    cli

    mov rdi, rcx
    mov rsi, rdx
    mov rdx, r8
    
    mov rbx, cr3
    lea r8, qword ptr [PML4]
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

    lea rax, qword ptr [PML4]
    mov cr3, rax

    mov rax, qword ptr [HIGHER_HALF_LOCATION]
    jmp rax

.section .text

higherHalf:

adjust_argument_ptrs:
    mov rbx, 0xFFFF800000000000

    add rdi, rbx
    add rsi, rbx
    add rdx, rbx

setup_stack:
    lea rax, qword ptr [STACK_TOP]
    add rax, rbx
    mov rsp, rax
    
setup_cr0_and_cr2:
    mov rax, cr0
    and ax, 0xFFFB
    or ax, 0x2
    mov cr0, rax
    mov rax, cr4
    or ax, 3 << 9
    mov cr4, rax

setup_null_gs:
    xor rax, rax
    push rax
    mov rbx, rsp
    
    push rbx
    push rdx
        
    mov eax, ebx
    shr rbx, 32
    mov edx, ebx
    mov ecx, 0xC0000101
    wrmsr

    pop rdx
    pop rcx

goto_kmain:
    mov rax, qword ptr [KMAIN_LOCATION]
    call rax