.section .text
.global set_active_gdt
set_active_gdt:
    lgdt [rdi]
    
    .reload_segments:
        push rsi
        push rcx
        retfq

.global reload_cs
reload_cs:
    mov ds, dx
    mov es, dx
    mov fs, dx
    mov gs, dx
    mov ss, dx
    ret