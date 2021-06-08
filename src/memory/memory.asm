GLOBAL set_current_pml4
set_current_pml4:
    mov cr3, rdi
    ret

GLOBAL get_cr2
get_cr2:
    mov rax, cr2
    ret

GLOBAL get_current_address_space
get_current_address_space:
    mov rax, cr3
    ret