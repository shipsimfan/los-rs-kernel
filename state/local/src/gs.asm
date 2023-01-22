GLOBAL get_gs
get_gs:
    mov rax, [gs:0]
    ret

GLOBAL set_gs
set_gs:
    mov eax, edi
    shr rdi, 32
    mov edx, edi
    mov ecx, 0xC0000101
    wrmsr
    ret