.align 16
.global hx_syscall_entry
hx_syscall_entry:
    cmp rax, 0x2009
    jne nt_entry

    cli     # we dont want to be interrupted
    swapgs

    mov gs:[0x68], rsp  # put user stack to _KPCR's Unused
    mov rsp, gs:[0x70]  # get our kernel stack

    push rcx            # save user rip
    mov rcx, gs:[0x78]  # load HxFs ptr
    mov rcx, [rcx]      # dereference it
    mov rcx, [rcx]      # get the registers inside

    call hx_capture_context
    pop rbx
    mov [rcx + rcx_offset], rbx     # load the actual rcx
    mov [rcx + rflags_offset], r11  # r11 is the rflags

    mov rax, gs:[0x68]
    mov [rcx + rsp_offset], rax     # set the old guest rsp

    sti

    mov rcx, gs:[0x78]
    mov rcx, [rcx]          # set first arg to hxfs

    sub rsp, 32

    # kpti is assumed to be OFF! otherwise we would load real kernel CR3 from gs:0B000h
    # disable kpti through setting FeatureSettingsOverrideMask to 3 in HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Memory Management

    call syscall_handler    # fresh air
    add rsp, 32

    mov rcx, gs:[0x78]
    call hx_restore_context

    # rcx and r11 is already restored by hx_restore_context
    # rsp must be restored manually
    mov rsp, gs:[0x68]

    swapgs
    sysretq

.align 16
nt_entry:
    jmp qword ptr [rip + NT_KI_SYSTEM_CALL64]