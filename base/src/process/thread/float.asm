.global save_float
save_float:
    fxsave [rdi]
    ret

.global load_float
load_float:
    fxrstor [rdi]
    ret