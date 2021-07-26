EXTERN system_call
GLOBAL system_call_handler
system_call_handler:
    sti
    ; Perform Syscall
    call system_call

    iretq