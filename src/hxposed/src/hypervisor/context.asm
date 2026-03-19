.align 16
.global restore_context
# ms x64 calling convention
# rcx is `&mut Registers`
restore_context:
    mov rax, [rcx + rax_offset]
    mov rbx, [rcx + rbx_offset]
    mov rdx, [rcx + rdx_offset]
    mov rsi, [rcx + rsi_offset]
    mov rdi, [rcx + rdi_offset]
    mov r8, [rcx + r8_offset]
    mov r9, [rcx + r9_offset]
    mov r10, [rcx + r10_offset]
    mov r11, [rcx + r11_offset]
    mov r12, [rcx + r12_offset]
    mov r13, [rcx + r13_offset]
    mov r14, [rcx + r14_offset]
    mov r15, [rcx + r15_offset]
    mov rbp, [rcx + rbp_offset]
    mov rcx, [rcx + rcx_offset]

    movdqu xmm0, [rcx + xmm0_offset]
    movdqu xmm1, [rcx + xmm1_offset]
    movdqu xmm2, [rcx + xmm2_offset]
    movdqu xmm3, [rcx + xmm3_offset]
    movdqu xmm4, [rcx + xmm4_offset]
    movdqu xmm5, [rcx + xmm5_offset]
    ret

.align 16
.global capture_context
# ms x64 calling convention
# rcx is `&mut Registers`
capture_context:
    mov [rcx + rax_offset], rax
    mov [rcx + rbx_offset], rbx
    mov [rcx + rcx_offset], rcx
    mov [rcx + rdx_offset], rdx
    mov [rcx + rsi_offset], rsi
    mov [rcx + rdi_offset], rdi
    mov [rcx + r8_offset], r8
    mov [rcx + r9_offset], r9
    mov [rcx + r10_offset], r10
    mov [rcx + r11_offset], r11
    mov [rcx + r12_offset], r12
    mov [rcx + r13_offset], r13
    mov [rcx + r14_offset], r14
    mov [rcx + r15_offset], r15
    mov [rcx + rbp_offset], rbp

    pushfq
    pop rax
    mov [rcx + rflags_offset], rax

    movdqu [rcx + xmm0_offset], xmm0
    movdqu [rcx + xmm1_offset], xmm1
    movdqu [rcx + xmm2_offset], xmm2
    movdqu [rcx + xmm3_offset], xmm3
    movdqu [rcx + xmm4_offset], xmm4
    movdqu [rcx + xmm5_offset], xmm5

    lea rax, [rsp + 8]
    mov [rcx + rsp_offset], rax

    # > is said to be CISC
    # > cant do memory to memory
    # > bruh

    mov rax, [rsp]
    mov [rcx + rip_offset], rax

    ret