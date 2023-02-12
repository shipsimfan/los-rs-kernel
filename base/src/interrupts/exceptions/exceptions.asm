.section .text.low
exception_handler_location: .8byte exception_handler

.section .text

common_exception_handler:
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

    mov rax, 0xFFFF800000000000
    lea rbx, exception_handler_location
    add rbx, rax
    mov rax, [rbx]
    call rax

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

    add rsp, 16

    iretq

.global exception_handler_0
exception_handler_0:
    push 0
    push 0
    jmp common_exception_handler

.global exception_handler_1
exception_handler_1:
    push 0
    push 1
    jmp common_exception_handler

.global exception_handler_2
exception_handler_2:
    push 0
    push 2
    jmp common_exception_handler

.global exception_handler_3
exception_handler_3:
    push 0
    push 3
    jmp common_exception_handler

.global exception_handler_4
exception_handler_4:
    push 0
    push 4
    jmp common_exception_handler

.global exception_handler_5
exception_handler_5:
    push 0
    push 5
    jmp common_exception_handler

.global exception_handler_6
exception_handler_6:
    push 0
    push 6
    jmp common_exception_handler

.global exception_handler_7
exception_handler_7:
    push 0
    push 7
    jmp common_exception_handler

.global exception_handler_8
exception_handler_8:
    push 8
    jmp common_exception_handler
    
.global exception_handler_9
exception_handler_9:
    push 0
    push 9
    jmp common_exception_handler

.global exception_handler_10
exception_handler_10:
    push 10
    jmp common_exception_handler

.global exception_handler_11
exception_handler_11:
    push 11
    jmp common_exception_handler

.global exception_handler_12
exception_handler_12:
    push 12
    jmp common_exception_handler

.global exception_handler_13
exception_handler_13:
    push 13
    jmp common_exception_handler

.global exception_handler_14
exception_handler_14:
    push 14
    jmp common_exception_handler

.global exception_handler_15
exception_handler_15:
    push 0
    push 15
    jmp common_exception_handler

.global exception_handler_16
exception_handler_16:
    push 0
    push 16
    jmp common_exception_handler

.global exception_handler_17
exception_handler_17:
    push 17
    jmp common_exception_handler

.global exception_handler_18
exception_handler_18:
    push 0
    push 18
    jmp common_exception_handler

.global exception_handler_19
exception_handler_19:
    push 0
    push 19
    jmp common_exception_handler

.global exception_handler_20
exception_handler_20:
    push 0
    push 20
    jmp common_exception_handler

.global exception_handler_21
exception_handler_21:
    push 0
    push 21
    jmp common_exception_handler

.global exception_handler_22
exception_handler_22:
    push 0
    push 22
    jmp common_exception_handler

.global exception_handler_23
exception_handler_23:
    push 0
    push 23
    jmp common_exception_handler

.global exception_handler_24
exception_handler_24:
    push 0
    push 24
    jmp common_exception_handler

.global exception_handler_25
exception_handler_25:
    push 0
    push 25
    jmp common_exception_handler

.global exception_handler_26
exception_handler_26:
    push 0
    push 26
    jmp common_exception_handler

.global exception_handler_27
exception_handler_27:
    push 0
    push 27
    jmp common_exception_handler

.global exception_handler_28
exception_handler_28:
    push 0
    push 28
    jmp common_exception_handler

.global exception_handler_29
exception_handler_29:
    push 0
    push 29
    jmp common_exception_handler

.global exception_handler_30
exception_handler_30:
    push 30
    jmp common_exception_handler

.global exception_handler_31
exception_handler_31:
    push 0
    push 31
    jmp common_exception_handler