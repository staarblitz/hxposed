.align 16
.global hv_vm_run
# ms x64 calling convention
# rcx is `&mut Registers`
hv_vm_run:
    call restore_context
    vmlaunch
    hlt # vmlaunch failed

.align 16
.global hv_vm_exit

# we should handle dr7

hv_vm_exit:
    push r15                # save r15
    rdfsbase r15            # set r15 to our HvCpu

    # save gprs
    mov [r15 + rax_offset], rax
    mov [r15 + rbx_offset], rbx
    mov [r15 + rcx_offset], rcx
    mov [r15 + rdx_offset], rdx
    mov [r15 + rsi_offset], rsi
    mov [r15 + rdi_offset], rdi
    mov [r15 + r8_offset],  r8
    mov [r15 + r9_offset],  r9
    mov [r15 + r10_offset], r10
    mov [r15 + r11_offset], r11
    mov [r15 + r12_offset], r12
    mov [r15 + r13_offset], r13
    mov [r15 + r14_offset], r14
    mov [r15 + rbp_offset], rbp

    # get old r15 from stack
    mov rax, [rsp]
    mov [r15 + r15_offset], rax

    pushfq
    pop rax
    mov [r15 + rflags_offset], rax

    # we need all xmms because they are volatile
    movdqa [r15 + xmm0_offset], xmm0
    movdqa [r15 + xmm1_offset], xmm1
    movdqa [r15 + xmm2_offset], xmm2
    movdqa [r15 + xmm3_offset], xmm3
    movdqa [r15 + xmm4_offset], xmm4
    movdqa [r15 + xmm5_offset], xmm5

    # set guest rsp in HvCpu
    mov rax, 0x681C     # GUEST_RSP
    vmread rax, rax
    mov [r15 + rsp_offset], rax

    # set guest rip in HvCpu
    mov rax, 0x681E     # GUEST_RIP
    vmread rax, rax
    mov [r15 + rip_offset], rax

    sub rsp, 32         # shadow space smh
    sub rsp, 8          # we saved r15, so our stack was 8 byte aligned. 8 bytes more to be 16-byte aligned.

    call vmexit_handler

    add rsp, 32
    add rsp, 8

    # vmexit_handler returns new RIP. set it
    mov rbx, 0x681E     # GUEST_RIP
    vmwrite rbx, rax

    mov rax, [r15 + rflags_offset]
    push rax
    popfq

    movdqa xmm0, [r15 + xmm0_offset]
    movdqa xmm1, [r15 + xmm1_offset]
    movdqa xmm2, [r15 + xmm2_offset]
    movdqa xmm3, [r15 + xmm3_offset]
    movdqa xmm4, [r15 + xmm4_offset]
    movdqa xmm5, [r15 + xmm5_offset]

    # gprs
    mov rax, [r15 + rax_offset]
    mov rbx, [r15 + rbx_offset]
    mov rcx, [r15 + rcx_offset]
    mov rdx, [r15 + rdx_offset]
    mov rsi, [r15 + rsi_offset]
    mov rdi, [r15 + rdi_offset]
    mov r8, [r15 + r8_offset]
    mov r9, [r15 + r9_offset]
    mov r10, [r15 + r10_offset]
    mov r11, [r15 + r11_offset]
    mov r12, [r15 + r12_offset]
    mov r13, [r15 + r13_offset]
    mov r14, [r15 + r14_offset]
    mov rbp, [r15 + rbp_offset]

    # last but not least, actual r15
    pop r15

    vmresume
    hlt                 # vmresume failed