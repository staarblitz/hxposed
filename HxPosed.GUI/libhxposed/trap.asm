; gracefully export(!) our functions
PUBLIC HxpTrap
option casemap:none

.code
; refer to intern/instructions.rs
; using Microsoft x64 calling convention.

; args:
; HX_REQUEST_RESPONSE* rcx
; HX_ASYNC_INFO* rdx

; return:
; -1 if hypervisor not loaded
; 0 if hypervisor catched the trap
HxpTrap proc
	; first, let's respect to ms x64 abi.
	pinsrq xmm4, rsi, 0
	pinsrq xmm4, rdi, 1
	pinsrq xmm5, r12, 0

	; in our hypervisor calling convention, the args are in this order:
	; r8, r9, r10
	; result and response will be in rsi register.
	mov rdi, rcx

	; extract args from the HX_REQUEST_RESPONSE
	mov r8, [rdi + 8]
	mov r9, [rdi + 16]
	mov r10, [rdi + 24]

	mov esi, [rdi]	; dereference the HX_CALL inside HX_REQUEST_RESPONSE
	
	; check if HX_ASYNC_INFO* is present
	cmp rdx, 0
	jz no_async	; nope, no async today.

	; set the async flag
	bts rsi, 20

	; move handle
	mov r11, qword ptr [rdx]
	; move pointer to shared memory region
	add rdx, 8
	mov r12, rdx
	sub rdx, 8

no_async:

	; check if extended args are present
	bt rsi, 21
	jnc make_the_call

	; extract extended args too
	movaps xmm0, [rdi + 32]
	movaps xmm1, [rdi + 48]
	movaps xmm2, [rdi + 64]
	movaps xmm3, [rdi + 80]

make_the_call:

	mov rcx, 2009h	; we want our hypervisor to catch this trap

	cpuid	; where we were?

	cmp rcx, 2009h	; the normal cpuid behavior resets the rcx. in this case, it should stay the same.
	je call_ok	
	mov rax, -1	; hypervisor did NOT catch our trap
	jmp end_fn

call_ok:
	mov dword ptr [rdi + 4], esi	; save result to second field of HX_REQUEST_RESPONSE
									; use esi instead of rsi, because HX_RESPONSE is 4 bytes long

	cmp rdx, 0	; if not async, gett the regs immediately
	jz fetch_regs

	mov rax, 0
	jmp end_fn

fetch_regs:

	; fetch regs returned by hypervisor
	mov qword ptr [rdi + 8], r8
	mov qword ptr [rdi + 16], r9
	mov qword ptr [rdi + 24], r10

end_fn:
	
	; get non-volatile rsi, r12, and rdi back
	pextrq rsi, xmm4, 0
	pextrq rdi, xmm4, 1
	pextrq r12, xmm5, 0


HxpTrap endp

end