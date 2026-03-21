.global hv_int_pf
.global hv_int_bp
.global hv_int_gp
.global hv_int_df
.global hv_int_handler

.align 16
hv_int_gp:
    jmp hx_gp_handler

.align 16
hv_int_df:
    push 8  # #DF
    jmp hv_int_handler

.align 16
hv_int_pf:
    push 14 # #PF
    jmp hv_int_handler

.align 16
hv_int_bp:
    push 0  # push dummy
    push 3  # #BP

    # make sure we wont fallthrough if i add something after this label
    jmp hv_int_handler

hv_int_handler:
    # when at this point, RSP is like this:
    # marked with x means its by us
    # [RSP] = exception vector      x
    # [RSP + 8] = error code        x/
    # [RSP + 16] = RIP
    # [RSP + 24] = CS
    # [RSP + 32] = RFLAGS
    # [RSP + 40] = RSP

    push rcx
    rdfsbase rcx
    mov rcx, [rcx]  # dereference registers inside HvFs

    call capture_context

    mov rdx, [rsp + 16]    # get RIP
    mov [rcx + rip_offset], rdx
    mov rdx, [rsp + 32]    # get rflags
    mov [rcx + rflags_offset], rdx
    mov rdx, [rsp + 40]    # get RSP
    mov [rcx + rsp_offset], rdx

    # use r15 since its nonvolatile and will survive the vm_int_handler call
    mov r15, rcx
    pop rcx     # get original rcx

    mov [r15 + rcx_offset], rcx # finally get back rcx

    pop rdx    # set second arg to exception vector
    pop r8     # set third arg to error code

    sub rsp, 32     # allocate shadow space
    call vm_int_handler # fresh air
    add rsp, 32

    # we dont need cli since we are in a vmexit
    hlt             # time to debug

    # set rip here to continue from exception i guess
    mov rcx, r15
    call restore_context    # where guest were?

    iretq           # where we were?

.align 16
hx_gp_handler:
    cmp r9, 0x2009  # check if called by us
    je handle_fail

    push 13         # push #GP
    jmp hv_int_handler  # now time for real handling

handle_fail:
    xor r9, r9      # signal that it failed

    add rsp, 8      # ignore the error code

    add qword ptr [rsp], 2  # wrmsr/rdmsr is 2 bytes long. since this is a fault, we need to increment rip manually.

    iretq           # where we were?