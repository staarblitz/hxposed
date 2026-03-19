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

    # fall through....
    # should we put an implicit jmp?
    # naaaaah

hv_int_handler:
    # when at this point, RSP is like this:
    # marked with x means its by us
    # [RSP] = exception vector      x
    # [RSP + 8] = error code        x/
    # [RSP + 16] = RIP
    # [RSP + 24] = CS
    # [RSP + 32] = RFLAGS
    # [RSP + 40] = RSP

    # first, save context
    sub rsp, 240    # size of Registers
    push rcx

    lea rcx, [rsp + 8]  # this is cool
    call capture_context

    pop rcx     # rcx was "dirty"

    mov [rsp + rcx_offset], rcx
    mov rcx, [rsp + 256]    # get RIP
    mov [rsp + rip_offset], rcx
    mov rcx, [rsp + 280]    # get RSP
    mov [rsp + rsp_offset], rcx

    mov rdx, [rsp + 240]    # set second arg to exception vector
    mov r8, [rsp + 248]     # set third arg to error code

    mov rcx, rsp    # set the first arg to Registers structure

    # looks like we trashed RSP
    mov rbp, rsp
    sub rsp, 32     # make buffer area so we dont hurt Registers structure
    and rsp, -16    # make sure its aligned

    sub rsp, 32     # now the shadow space
    call vm_int_handler # fresh air

    # we dont need cli since we are in a vmexit
    hlt             # time to debug

    # set rip here to continue from exception i guess
    mov rsp, rbp    # where we were?
    add rsp, 240    # we dont need that struct anymore
    add rsp, 16     # skip the error code and exception vector pushed by us or the cpu

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