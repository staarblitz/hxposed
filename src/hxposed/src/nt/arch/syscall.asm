.align 16
.global hx_shadow_syscall_entry
hx_shadow_syscall_entry:
    mov gs:[0x0B010], rsp
    mov rsp, gs:[0x0B000]
    bt dword ptr gs:[0x0B018], 1
    jb no_swap

    mov cr3, rsp
no_swap:
    mov rsp, gs:[0x0B008]

.align 16
.global hx_syscall_entry
hx_syscall_entry:
    cmp rax, 0x2009
    jne nt_entry

    # cmp byte ptr [rip + KiKvaShadow], 1
    # je hx_shadow_syscall_entry

    swapgs

    cli     # we dont want to be interrupted

    mov gs:[0x10], rsp  # save user stack
    mov rsp, gs:[0x1A8] # get kernel stack

    push gs:[0x10]      # save user stack again, since gs:[0x10] is volatile

    push rcx            # save user rip

    sub rsp, 240        # allocate space for Registers structure
    mov rcx, rsp
    call hx_capture_context

    mov rcx, [rsp + 240]        # "pop" rip back
    mov [rsp + rcx_offset], rcx # save it to our structure
    mov [rsp + r11_offset], r11 # rflags is important as well

    mov rcx, rsp    # set rcx to Registers structure

    sub rsp, 32     # shadow space for msx64 abi

    # kpti is assumed to be OFF! otherwise we would load real kernel CR3
    sti # enable interrupts. we want to be in PASSIVE_LEVEL
    stac    # allow user-mode acceess since SMAP is probably on

    call syscall_handler    # fresh air

    clac    # unallow
    cli

    add rsp, 32

    mov rcx, rsp
    call hx_restore_context

    # rcx and r11 is already restored by hx_restore_context
    # rsp must be restored manually
    add rsp, 248
    pop rsp     # get user stack back

    swapgs

    mov rax, 0x2009     # right here

    sysretq     # where we were?

.align 16
nt_entry:
    jmp qword ptr [rip + NT_KI_SYSTEM_CALL64]