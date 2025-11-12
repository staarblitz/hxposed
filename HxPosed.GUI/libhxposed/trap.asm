; gracefully export(!) our functions
PUBLIC trap
option casemap:none

.code
; refer to intern/instructions.rs
; using Microsoft x64 calling convention.

; args:
; hypervisor_req_resp_t* rcx
; arg1 r8
; arg2 r9
; arg3 r10

; return:
; none
trap proc
	; in our hypervisor calling convention, the args are in this order:
	; r8, r9, r10
	; result and response will be in rsi register.
	mov rdi, rcx

	; extract args from the hypervisor_req_resp_t
	mov r8, [rdi + 8]
	mov r9, [rdi + 16]
	mov r10, [rdi + 24]

	mov rcx, 2009h ; we want our hypervisor to catch this trap

	cpuid ; where we were?

	mov [rdi + 4], rsi ; save result to second field of hypervisor_req_resp_t

	; save regs returned by hypervisor
	mov [rdi + 8], r8
	mov [rdi + 16], r9
	mov [rdi + 24], r10
	ret ; we are done here.
trap endp

end