; gracefully export(!) our functions
PUBLIC HxpTrap
option casemap:none

.code
; refer to intern/instructions.rs
; using Microsoft x64 calling convention.

; args:
; HX_REQUEST_RESPONSE* rcx

; return value on rax:
; -1 if hypervisor not loaded
; 0 if hypervisor catched the trap
HxpTrap proc
	; first, let's respect to ms x64 abi.
	pinsrq xmm4, rsi, 0
	pinsrq xmm4, rdi, 1
	pinsrq xmm5, r12, 0
	pinsrq xmm5, rbx, 1

	; in our hypervisor calling convention, the args are in this order:
	; r8, r9, r10
	; result and response will be in rsi register.
	mov rdi, rcx

	; extract args from the HX_REQUEST_RESPONSE
	mov r8, [rdi + 16]
	mov r9, [rdi + 24]
	mov r10, [rdi + 32]

	mov rsi, [rdi]	; dereference the HX_CALL inside HX_REQUEST_RESPONSE

	; check if extended args are present
	bt rsi, 17
	jnc make_the_call

	; extract extended args too
	movaps xmm0, [rdi + 48]
	movaps xmm1, [rdi + 64]
	movaps xmm2, [rdi + 80]
	movaps xmm3, [rdi + 96]

make_the_call:

	mov rcx, 2009h	; we want our hypervisor to catch this trap

	cpuid	; where we were?

	cmp rcx, 2009h	; the normal cpuid behavior resets the rcx. in this case, it should stay the same.
	je call_ok	
	mov rax, -1	; hypervisor did NOT catch our trap
	jmp return

call_ok:
	xor rax, rax ; good to go!
	mov qword ptr [rdi + 8], rsi	; save result to second field of HX_REQUEST_RESPONSE

	; fetch regs returned by hypervisor
	mov qword ptr [rdi + 16], r8
	mov qword ptr [rdi + 24], r9
	mov qword ptr [rdi + 32], r10
	
return:

	; get non-volatile rsi, r12, and rdi back
	pextrq rsi, xmm4, 0
	pextrq rdi, xmm4, 1
	pextrq r12, xmm5, 0
	pextrq rbx, xmm5, 1
	

	ret


HxpTrap endp

end