.section .text.low
reload_cs_location: .8byte .reload_cs

.section .text
.global set_active_gdt
set_active_gdt:
    lgdt [rdi]
    
    .reload_segments:
        push rsi

        mov rax, qword ptr [reload_cs_location]
        push rax
        
        retfq

    .reload_cs:
        mov ds, dx
        mov es, dx
        mov fs, dx
        mov gs, dx
        mov ss, dx
        ret