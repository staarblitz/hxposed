.align 16
.global rdmsr_failsafe_naked
# ms x64 calling convention
# rcx is msr id
# rax is returned msr value
# rdx defines if msr exists. -1 if not, 0 if exists.
rdmsr_failsafe_naked:
    mov r9, 0x2009     # put our beloved
    rdmsr
    cmp r9, 0          # check if this triggered a #GP
    jz fail
    shl rdx, 32
    or rax, rdx         # combine with some bitshift
    xor rdx, rdx        # beautiful
    jmp end
fail:
    mov rdx, -1         # no such msr
end:
    ret

.align 16
.global wrmsr_failsafe_naked
# ms x64 calling convention
# rcx is msr id
# rdx is msr value
# rax defines if msr exists. -1 if not, 0 if exists
wrmsr_failsafe_naked:
    mov rax, rdx
    shr rdx, 32

    mov r9, 0x2009     # same deal
    wrmsr

    xor rax, rax        # init rax to 0 before cmove
    mov rcx, -1
    cmp r9, 0          # check if it resulted in a #GP
    cmove rax, rcx      # branchless!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

    ret

.align 16
.global hw_bp
# not really a bp, but best we can do.
hw_bp:
    jmp hw_bp
    ret