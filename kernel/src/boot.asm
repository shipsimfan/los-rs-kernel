SECTION .data.low

;======================================
; PAGE TABLES
;======================================
ALIGN 4096
pml4: times 512 dq 0

SECTION .bss

;======================================
; ACPI ROOT TABLE
;======================================
GLOBAL acpiRootPointer
acpiRootPointer: resq 1

;======================================
; STACK
;======================================
stackBottom: resb 32768
GLOBAL stackTop
stackTop:

SECTION .text.low

;======================================
; BOOT ENTRY POINT
;======================================
GLOBAL _start

EXTERN kmain

_start:
    ; Disable interrupts
    cli

    mov rdi, rcx
    mov rsi, rdx
    mov rdx, r8

    ; Copy page structures from UEFI
    mov rbx, cr3
    mov r8, pml4
    mov rcx, 256

    .copyLow:
        mov rax, [rbx]
        mov [r8], rax
        add rbx, 8
        add r8, 8
        loop .copyLow

    ; Copy page structures into higher half
    mov rbx, cr3
    mov rcx, 256
    
    .copyHigh:
        mov rax, [rbx]
        mov [r8], rax
        add rbx, 8
        add r8, 8
        loop .copyHigh

    ; Set page structures
    mov rax, pml4
    mov cr3, rax

    mov rax, higherHalf
    jmp rax

SECTION .text

higherHalf:
    ; Setup the stack    
    mov rsp, stackTop

    ; Move arguments to higher half
    mov rax, 0xFFFF800000000000
    add rdi, rax
    add rsi, rax

    ; Enable SSE for floats
    mov rax, cr0
    and ax, 0xFFFB
    or ax, 0x2
    mov cr0, rax
    mov rax, cr4
    or ax, 3 << 9
    mov cr4, rax

    ; Save ACPI Root table
    mov rax, acpiRootPointer
    mov [rax], rdx

    ; Call kernel main
    mov rax, kmain
    call rax