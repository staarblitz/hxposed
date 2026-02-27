.extern ORIGINAL_GP_HANDLER

.align 16
.global hx_gp_handler
hx_gp_handler:
    cmp r9, 0x2009         # check if called by us
    je handle_fail

    jmp [rip + ORIGINAL_GP_HANDLER]

handle_fail:
    xor r9, r9            # signal that it failed
    add rsp, 8              # ignore the error code
    add qword ptr [rsp], 2  # wrmsr/rdmsr is 2 bytes long. since this is a fault, we need to increment rip manually.
    iretq

.align 16
.global rdmsr_failsafe_naked
# rcx is msr id
# rax is returned msr value
# rdx defines if msr exists. -1 if not, 0 if exists.
rdmsr_failsafe_naked:
    pinsrq xmm4, r9, 0 # r9 is nonvolatile, save it
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
    pextrq r9, xmm4, 0  # get it back
    ret

.align 16
.global wrmsr_failsafe_naked
# rcx is msr id
# rdx is msr value
# rax defines if msr exists. -1 if not, 0 if exists
wrmsr_failsafe_naked:
    mov rax, rcx        # save the rcx
    mov rcx, rdx
    mov rdx, rax
    shr rax, 32         # bit shift ecx to low
    mov rcx, rdx

    pinsrq xmm4, r9, 0 # save r9
    mov r9, 0x2009     # same deal
    wrmsr

    xor rax, rax        # init rax to 0 before cmove
    mov rcx, -1
    cmp r9, 0          # check if it resulted in a #GP
    cmove rax, rcx      # branchless!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

    pextrq r9, xmm4, 0

    ret