	.file	"loop_aly_tmp.c"
	.text
	.section	.rodata.str1.1,"aMS",@progbits,1
.LC0:
	.string	"Error: Modulo by zero\n"
	.text
	.p2align 4
	.type	aly_mod.part.0, @function
aly_mod.part.0:
.LFB114:
	.cfi_startproc
	subq	$8, %rsp
	.cfi_def_cfa_offset 16
	movl	$.LC0, %edi
	movl	$22, %edx
	movq	stderr(%rip), %rcx
	movl	$1, %esi
	call	fwrite
	movl	$1, %edi
	call	exit
	.cfi_endproc
.LFE114:
	.size	aly_mod.part.0, .-aly_mod.part.0
	.p2align 4
	.globl	native_tomb
	.type	native_tomb, @function
native_tomb:
.LFB108:
	.cfi_startproc
	testl	%esi, %esi
	movq	%rdi, %rax
	jg	.L7
	movl	$6, (%rdi)
	movq	$0, 8(%rdi)
	movq	$0, 16(%rdi)
	ret
	.p2align 4,,10
	.p2align 3
.L7:
	vmovdqu	(%rdx), %xmm0
	movq	16(%rdx), %rdx
	vmovdqu	%xmm0, (%rdi)
	movq	%rdx, 16(%rdi)
	ret
	.cfi_endproc
.LFE108:
	.size	native_tomb, .-native_tomb
	.p2align 4
	.globl	native_len
	.type	native_len, @function
native_len:
.LFB109:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	testl	%esi, %esi
	movq	%rdi, %rcx
	jg	.L9
.L13:
	xorl	%eax, %eax
.L10:
	movq	%rax, 8(%rcx)
	movq	%rcx, %rax
	movl	$0, (%rcx)
	movq	$0, 16(%rcx)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L9:
	.cfi_restore_state
	movl	(%rdx), %eax
	movq	8(%rdx), %rdi
	cmpl	$4, %eax
	je	.L12
	cmpl	$5, %eax
	je	.L12
	cmpl	$3, %eax
	jne	.L13
	testq	%rdi, %rdi
	je	.L13
	movq	%rcx, 8(%rsp)
	call	strlen
	movq	8(%rsp), %rcx
	cltq
	jmp	.L10
	.p2align 4,,10
	.p2align 3
.L12:
	testq	%rdi, %rdi
	je	.L13
	movslq	8(%rdi), %rax
	jmp	.L10
	.cfi_endproc
.LFE109:
	.size	native_len, .-native_len
	.p2align 4
	.type	aly_object_get.constprop.2.isra.0, @function
aly_object_get.constprop.2.isra.0:
.LFB118:
	.cfi_startproc
	cmpl	$5, %esi
	movq	%rdi, %r8
	jne	.L27
	testq	%rdx, %rdx
	movq	%rdx, %rdi
	je	.L27
	movl	12(%rdx), %esi
	testl	%esi, %esi
	jle	.L27
	movl	$177678, %eax
	xorl	%edx, %edx
	movq	(%rdi), %rdi
	divl	%esi
	movl	%edx, %ecx
	leal	(%rsi,%rdx), %r9d
	jmp	.L38
	.p2align 4,,10
	.p2align 3
.L46:
	movq	(%rax), %rdx
	cmpb	$105, (%rdx)
	jne	.L31
	cmpb	$0, 1(%rdx)
	je	.L45
.L31:
	addl	$1, %ecx
	cmpl	%r9d, %ecx
	je	.L27
.L38:
	xorl	%edx, %edx
	movl	%ecx, %eax
	divl	%esi
	leaq	(%rdx,%rdx,4), %rax
	leaq	(%rdi,%rax,8), %rax
	movl	32(%rax), %edx
	testl	%edx, %edx
	jne	.L46
.L27:
	movl	$6, (%r8)
	movq	%r8, %rax
	movq	$0, 8(%r8)
	movq	$0, 16(%r8)
	ret
	.p2align 4,,10
	.p2align 3
.L45:
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	vmovdqu	8(%rax), %xmm0
	movq	24(%rax), %rdx
	movl	8(%rax), %eax
	movq	%rdx, 32(%rsp)
	cmpl	$2, %eax
	vmovdqa	%xmm0, 16(%rsp)
	je	.L32
	ja	.L33
	testl	%eax, %eax
	je	.L47
	vmovsd	24(%rsp), %xmm0
	movl	$1, (%r8)
	movq	$0, 16(%r8)
	vmovsd	%xmm0, 8(%r8)
.L26:
	movq	%r8, %rax
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
.L47:
	.cfi_restore_state
	movq	24(%rsp), %rax
	movl	$0, (%r8)
	movq	$0, 16(%r8)
	movq	%rax, 8(%r8)
	jmp	.L26
	.p2align 4,,10
	.p2align 3
.L33:
	cmpl	$3, %eax
	jne	.L36
	movq	24(%rsp), %rdi
	movq	%r8, 8(%rsp)
	call	strdup
	movq	8(%rsp), %r8
	movl	$3, (%r8)
	movq	%rax, 8(%r8)
	movq	$0, 16(%r8)
	jmp	.L26
.L32:
	movl	24(%rsp), %eax
	movl	$2, (%r8)
	movq	$0, 16(%r8)
	movl	%eax, 8(%r8)
	jmp	.L26
.L36:
	movq	%rdx, 16(%r8)
	vmovdqu	%xmm0, (%r8)
	jmp	.L26
	.cfi_endproc
.LFE118:
	.size	aly_object_get.constprop.2.isra.0, .-aly_object_get.constprop.2.isra.0
	.p2align 4
	.type	aly_object_get.constprop.1.isra.0, @function
aly_object_get.constprop.1.isra.0:
.LFB120:
	.cfi_startproc
	cmpl	$5, %esi
	movq	%rdi, %r8
	jne	.L49
	testq	%rdx, %rdx
	movq	%rdx, %rdi
	je	.L49
	movl	12(%rdx), %esi
	testl	%esi, %esi
	jle	.L49
	movl	$177679, %eax
	xorl	%edx, %edx
	movq	(%rdi), %rdi
	divl	%esi
	movl	%edx, %ecx
	leal	(%rsi,%rdx), %r9d
	jmp	.L60
	.p2align 4,,10
	.p2align 3
.L68:
	movq	(%rax), %rdx
	cmpb	$106, (%rdx)
	jne	.L53
	cmpb	$0, 1(%rdx)
	je	.L67
.L53:
	addl	$1, %ecx
	cmpl	%r9d, %ecx
	je	.L49
.L60:
	xorl	%edx, %edx
	movl	%ecx, %eax
	divl	%esi
	leaq	(%rdx,%rdx,4), %rax
	leaq	(%rdi,%rax,8), %rax
	movl	32(%rax), %edx
	testl	%edx, %edx
	jne	.L68
.L49:
	movl	$6, (%r8)
	movq	%r8, %rax
	movq	$0, 8(%r8)
	movq	$0, 16(%r8)
	ret
	.p2align 4,,10
	.p2align 3
.L67:
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	vmovdqu	8(%rax), %xmm0
	movq	24(%rax), %rdx
	movl	8(%rax), %eax
	movq	%rdx, 32(%rsp)
	cmpl	$2, %eax
	vmovdqa	%xmm0, 16(%rsp)
	je	.L54
	ja	.L55
	testl	%eax, %eax
	je	.L69
	vmovsd	24(%rsp), %xmm0
	movl	$1, (%r8)
	movq	$0, 16(%r8)
	vmovsd	%xmm0, 8(%r8)
.L48:
	movq	%r8, %rax
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
.L69:
	.cfi_restore_state
	movq	24(%rsp), %rax
	movl	$0, (%r8)
	movq	$0, 16(%r8)
	movq	%rax, 8(%r8)
	jmp	.L48
	.p2align 4,,10
	.p2align 3
.L55:
	cmpl	$3, %eax
	jne	.L58
	movq	24(%rsp), %rdi
	movq	%r8, 8(%rsp)
	call	strdup
	movq	8(%rsp), %r8
	movl	$3, (%r8)
	movq	%rax, 8(%r8)
	movq	$0, 16(%r8)
	jmp	.L48
.L54:
	movl	24(%rsp), %eax
	movl	$2, (%r8)
	movq	$0, 16(%r8)
	movl	%eax, 8(%r8)
	jmp	.L48
.L58:
	movq	%rdx, 16(%r8)
	vmovdqu	%xmm0, (%r8)
	jmp	.L48
	.cfi_endproc
.LFE120:
	.size	aly_object_get.constprop.1.isra.0, .-aly_object_get.constprop.1.isra.0
	.p2align 4
	.type	aly_object_get.constprop.0.isra.0, @function
aly_object_get.constprop.0.isra.0:
.LFB122:
	.cfi_startproc
	cmpl	$5, %esi
	movq	%rdi, %r8
	jne	.L71
	testq	%rdx, %rdx
	movq	%rdx, %rdi
	je	.L71
	movl	12(%rdx), %esi
	testl	%esi, %esi
	jle	.L71
	movl	$177680, %eax
	xorl	%edx, %edx
	movq	(%rdi), %rdi
	divl	%esi
	movl	%edx, %ecx
	leal	(%rsi,%rdx), %r9d
	jmp	.L82
	.p2align 4,,10
	.p2align 3
.L90:
	movq	(%rax), %rdx
	cmpb	$107, (%rdx)
	jne	.L75
	cmpb	$0, 1(%rdx)
	je	.L89
.L75:
	addl	$1, %ecx
	cmpl	%r9d, %ecx
	je	.L71
.L82:
	xorl	%edx, %edx
	movl	%ecx, %eax
	divl	%esi
	leaq	(%rdx,%rdx,4), %rax
	leaq	(%rdi,%rax,8), %rax
	movl	32(%rax), %edx
	testl	%edx, %edx
	jne	.L90
.L71:
	movl	$6, (%r8)
	movq	%r8, %rax
	movq	$0, 8(%r8)
	movq	$0, 16(%r8)
	ret
	.p2align 4,,10
	.p2align 3
.L89:
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	vmovdqu	8(%rax), %xmm0
	movq	24(%rax), %rdx
	movl	8(%rax), %eax
	movq	%rdx, 32(%rsp)
	cmpl	$2, %eax
	vmovdqa	%xmm0, 16(%rsp)
	je	.L76
	ja	.L77
	testl	%eax, %eax
	je	.L91
	vmovsd	24(%rsp), %xmm0
	movl	$1, (%r8)
	movq	$0, 16(%r8)
	vmovsd	%xmm0, 8(%r8)
.L70:
	movq	%r8, %rax
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
.L91:
	.cfi_restore_state
	movq	24(%rsp), %rax
	movl	$0, (%r8)
	movq	$0, 16(%r8)
	movq	%rax, 8(%r8)
	jmp	.L70
	.p2align 4,,10
	.p2align 3
.L77:
	cmpl	$3, %eax
	jne	.L80
	movq	24(%rsp), %rdi
	movq	%r8, 8(%rsp)
	call	strdup
	movq	8(%rsp), %r8
	movl	$3, (%r8)
	movq	%rax, 8(%r8)
	movq	$0, 16(%r8)
	jmp	.L70
.L76:
	movl	24(%rsp), %eax
	movl	$2, (%r8)
	movq	$0, 16(%r8)
	movl	%eax, 8(%r8)
	jmp	.L70
.L80:
	movq	%rdx, 16(%r8)
	vmovdqu	%xmm0, (%r8)
	jmp	.L70
	.cfi_endproc
.LFE122:
	.size	aly_object_get.constprop.0.isra.0, .-aly_object_get.constprop.0.isra.0
	.p2align 4
	.globl	aly_int
	.type	aly_int, @function
aly_int:
.LFB24:
	.cfi_startproc
	movl	$0, (%rdi)
	movq	%rdi, %rax
	movq	%rsi, 8(%rdi)
	movq	$0, 16(%rdi)
	ret
	.cfi_endproc
.LFE24:
	.size	aly_int, .-aly_int
	.p2align 4
	.globl	aly_float
	.type	aly_float, @function
aly_float:
.LFB25:
	.cfi_startproc
	movl	$1, (%rdi)
	movq	%rdi, %rax
	movq	$0, 16(%rdi)
	vmovsd	%xmm0, 8(%rdi)
	ret
	.cfi_endproc
.LFE25:
	.size	aly_float, .-aly_float
	.p2align 4
	.globl	aly_bool
	.type	aly_bool, @function
aly_bool:
.LFB26:
	.cfi_startproc
	movl	$2, (%rdi)
	movq	%rdi, %rax
	movl	%esi, 8(%rdi)
	movq	$0, 16(%rdi)
	ret
	.cfi_endproc
.LFE26:
	.size	aly_bool, .-aly_bool
	.p2align 4
	.globl	aly_none
	.type	aly_none, @function
aly_none:
.LFB27:
	.cfi_startproc
	movl	$6, (%rdi)
	movq	%rdi, %rax
	movq	$0, 8(%rdi)
	movq	$0, 16(%rdi)
	ret
	.cfi_endproc
.LFE27:
	.size	aly_none, .-aly_none
	.p2align 4
	.globl	aly_function
	.type	aly_function, @function
aly_function:
.LFB28:
	.cfi_startproc
	vmovq	%rsi, %xmm0
	movl	$7, (%rdi)
	movq	%rdi, %rax
	vmovdqu	%xmm0, 8(%rdi)
	ret
	.cfi_endproc
.LFE28:
	.size	aly_function, .-aly_function
	.p2align 4
	.globl	aly_string
	.type	aly_string, @function
aly_string:
.LFB29:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movq	%rdi, %rbx
	movq	%rsi, %rdi
	call	strdup
	movl	$3, (%rbx)
	movq	%rax, 8(%rbx)
	movq	%rbx, %rax
	movq	$0, 16(%rbx)
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE29:
	.size	aly_string, .-aly_string
	.p2align 4
	.globl	aly_string_alloc
	.type	aly_string_alloc, @function
aly_string_alloc:
.LFB30:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movq	%rdi, %rbx
	leal	1(%rsi), %edi
	movl	$1, %esi
	movslq	%edi, %rdi
	call	calloc
	movl	$3, (%rbx)
	movq	%rax, 8(%rbx)
	movq	%rbx, %rax
	movq	$0, 16(%rbx)
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE30:
	.size	aly_string_alloc, .-aly_string_alloc
	.p2align 4
	.globl	aly_free
	.type	aly_free, @function
aly_free:
.LFB31:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movl	48(%rsp), %eax
	movq	%rbx, 16(%rsp)
	movq	%rbp, 24(%rsp)
	.cfi_offset 3, -32
	.cfi_offset 6, -24
	movq	56(%rsp), %rbx
	cmpl	$4, %eax
	movq	64(%rsp), %rbp
	je	.L102
	cmpl	$5, %eax
	je	.L103
	cmpl	$3, %eax
	jne	.L104
.L122:
	movq	%rbx, %rdi
	call	free
.L104:
	testq	%rbp, %rbp
	je	.L119
	movq	16(%rsp), %rbx
	movq	%rbp, %rdi
	movq	24(%rsp), %rbp
	addq	$40, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	jmp	free
	.p2align 4,,10
	.p2align 3
.L103:
	.cfi_restore_state
	testq	%rbx, %rbx
	je	.L104
	movl	12(%rbx), %ecx
	movq	(%rbx), %rdi
	testl	%ecx, %ecx
	jle	.L107
	movq	%r12, 32(%rsp)
	.cfi_offset 12, -16
	xorl	%r12d, %r12d
.L110:
	leaq	(%r12,%r12,4), %rax
	movl	32(%rdi,%rax,8), %edx
	testl	%edx, %edx
	jne	.L123
	addq	$1, %r12
	cmpl	%r12d, %ecx
	jg	.L110
.L121:
	movq	32(%rsp), %r12
	.cfi_restore 12
.L107:
	call	free
	jmp	.L122
	.p2align 4,,10
	.p2align 3
.L119:
	movq	16(%rsp), %rbx
	movq	24(%rsp), %rbp
	addq	$40, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L102:
	.cfi_restore_state
	testq	%rbx, %rbx
	je	.L104
	movl	8(%rbx), %esi
	testl	%esi, %esi
	jle	.L105
	movq	%r12, 32(%rsp)
	.cfi_offset 12, -16
	xorl	%r12d, %r12d
.L106:
	movq	(%rbx), %rcx
	leaq	(%r12,%r12,2), %rax
	subq	$32, %rsp
	.cfi_def_cfa_offset 80
	addq	$1, %r12
	leaq	(%rcx,%rax,8), %rax
	vmovdqu	(%rax), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%rax), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 48
	cmpl	%r12d, 8(%rbx)
	jg	.L106
	movq	32(%rsp), %r12
	.cfi_restore 12
.L105:
	movq	(%rbx), %rdi
	call	free
	movq	%rbx, %rdi
	call	free
	jmp	.L104
	.p2align 4,,10
	.p2align 3
.L123:
	.cfi_offset 12, -16
	movq	(%rdi,%rax,8), %rdi
	movq	%rax, 8(%rsp)
	addq	$1, %r12
	call	free
	subq	$32, %rsp
	.cfi_def_cfa_offset 80
	movq	(%rbx), %rdx
	movq	40(%rsp), %rax
	vmovdqu	8(%rdx,%rax,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	24(%rdx,%rax,8), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	movl	12(%rbx), %ecx
	addq	$32, %rsp
	.cfi_def_cfa_offset 48
	movq	(%rbx), %rdi
	cmpl	%r12d, %ecx
	jg	.L110
	jmp	.L121
	.cfi_endproc
.LFE31:
	.size	aly_free, .-aly_free
	.p2align 4
	.globl	aly_is_truthy
	.type	aly_is_truthy, @function
aly_is_truthy:
.LFB33:
	.cfi_startproc
	cmpl	$5, 8(%rsp)
	ja	.L125
	movl	8(%rsp), %eax
	jmp	*.L127(,%rax,8)
	.section	.rodata
	.align 8
	.align 4
.L127:
	.quad	.L132
	.quad	.L131
	.quad	.L130
	.quad	.L129
	.quad	.L126
	.quad	.L126
	.text
	.p2align 4,,10
	.p2align 3
.L126:
	movq	16(%rsp), %rax
	testq	%rax, %rax
	je	.L125
	movl	8(%rax), %eax
	testl	%eax, %eax
	setg	%al
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L129:
	movq	16(%rsp), %rax
	testq	%rax, %rax
	je	.L125
	cmpb	$0, (%rax)
	setne	%al
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L132:
	xorl	%eax, %eax
	cmpq	$0, 16(%rsp)
	setne	%al
	ret
	.p2align 4,,10
	.p2align 3
.L131:
	vxorpd	%xmm0, %xmm0, %xmm0
	xorl	%eax, %eax
	movl	$1, %edx
	vucomisd	16(%rsp), %xmm0
	setp	%al
	cmovne	%edx, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L130:
	movl	16(%rsp), %eax
	ret
	.p2align 4,,10
	.p2align 3
.L125:
	xorl	%eax, %eax
	ret
	.cfi_endproc
.LFE33:
	.size	aly_is_truthy, .-aly_is_truthy
	.section	.rodata.str1.1
.LC2:
	.string	"unknown"
	.text
	.p2align 4
	.globl	aly_type_name
	.type	aly_type_name, @function
aly_type_name:
.LFB34:
	.cfi_startproc
	movl	8(%rsp), %eax
	movl	$.LC2, %edx
	cmpl	$7, %eax
	ja	.L145
	movq	CSWTCH.102(,%rax,8), %rdx
.L145:
	movq	%rdx, %rax
	ret
	.cfi_endproc
.LFE34:
	.size	aly_type_name, .-aly_type_name
	.p2align 4
	.globl	aly_to_float
	.type	aly_to_float, @function
aly_to_float:
.LFB36:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	cmpl	$1, %edx
	je	.L149
	cmpl	$3, %edx
	movq	40(%rsp), %rdi
	je	.L150
	testl	%edx, %edx
	movl	$1, (%rax)
	jne	.L151
	vxorps	%xmm0, %xmm0, %xmm0
	movq	$0, 16(%rax)
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vmovlpd	%xmm0, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L150:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rax
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	vmovsd	%xmm0, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L149:
	.cfi_restore_state
	vmovdqu	32(%rsp), %xmm0
	movq	48(%rsp), %rdx
	movq	%rdx, 16(%rdi)
	vmovdqu	%xmm0, (%rdi)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L151:
	.cfi_restore_state
	movq	$0x000000000, 8(%rax)
	movq	$0, 16(%rax)
	addq	$24, %rsp
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE36:
	.size	aly_to_float, .-aly_to_float
	.section	.rodata.str1.1
.LC3:
	.string	"true"
.LC4:
	.string	"false"
.LC5:
	.string	"%lld"
.LC6:
	.string	"%g"
.LC7:
	.string	"None"
.LC8:
	.string	"[]"
.LC9:
	.string	"{}"
	.text
	.p2align 4
	.globl	aly_to_str
	.type	aly_to_str, @function
aly_to_str:
.LFB37:
	.cfi_startproc
	subq	$568, %rsp
	.cfi_def_cfa_offset 576
	cmpl	$6, 576(%rsp)
	movq	%rbx, 544(%rsp)
	.cfi_offset 3, -32
	movq	%rdi, %rbx
	ja	.L155
	movl	576(%rsp), %eax
	jmp	*.L157(,%rax,8)
	.section	.rodata
	.align 8
	.align 4
.L157:
	.quad	.L163
	.quad	.L162
	.quad	.L161
	.quad	.L160
	.quad	.L159
	.quad	.L158
	.quad	.L155
	.text
	.p2align 4,,10
	.p2align 3
.L155:
	movl	$.LC7, %edi
	call	strdup
.L164:
	movq	%rax, 8(%rbx)
	movq	%rbx, %rax
	movl	$3, (%rbx)
	movq	$0, 16(%rbx)
	movq	544(%rsp), %rbx
	addq	$568, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L158:
	.cfi_restore_state
	movq	584(%rsp), %rcx
	testq	%rcx, %rcx
	je	.L172
	movl	8(%rcx), %edx
	testl	%edx, %edx
	je	.L172
	movl	$1024, %esi
	movl	$1, %edi
	movq	%rcx, 8(%rsp)
	movq	%r14, 552(%rsp)
	.cfi_offset 14, -24
	call	calloc
	movq	8(%rsp), %rcx
	movw	$123, (%rax)
	movq	%rax, %r14
	movl	12(%rcx), %esi
	testl	%esi, %esi
	jle	.L174
	movq	%r15, 560(%rsp)
	xorl	%edx, %edx
	movl	$1, %r9d
	.cfi_offset 15, -16
.L177:
	movq	(%rcx), %rdi
	leaq	(%rdx,%rdx,4), %rax
	leaq	(%rdi,%rax,8), %r8
	movl	32(%r8), %eax
	testl	%eax, %eax
	je	.L175
	testl	%r9d, %r9d
	je	.L187
.L176:
	movq	%r14, %rdi
	movq	%rdx, 24(%rsp)
	movq	%rcx, 16(%rsp)
	movq	%r8, 8(%rsp)
	call	strlen
	movq	8(%rsp), %r8
	leaq	(%r14,%rax), %rdi
	movq	(%r8), %rsi
	call	stpcpy
	subq	$32, %rsp
	.cfi_def_cfa_offset 608
	movq	40(%rsp), %r8
	movw	$8250, (%rax)
	movq	%rax, %r15
	leaq	64(%rsp), %rdi
	movb	$0, 2(%rax)
	vmovdqu	8(%r8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	24(%r8), %rax
	movq	%rax, 16(%rsp)
	call	aly_to_str
	movq	72(%rsp), %rsi
	addq	$32, %rsp
	.cfi_def_cfa_offset 576
	leaq	2(%r15), %rdi
	call	strcpy
	subq	$32, %rsp
	.cfi_def_cfa_offset 608
	movq	80(%rsp), %rax
	vmovdqa	64(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	movq	48(%rsp), %rcx
	movq	56(%rsp), %rdx
	addq	$32, %rsp
	.cfi_def_cfa_offset 576
	xorl	%r9d, %r9d
	movl	12(%rcx), %esi
.L175:
	addq	$1, %rdx
	cmpl	%edx, %esi
	jg	.L177
	movq	560(%rsp), %r15
	.cfi_restore 15
.L174:
	movq	%r14, %rdi
	call	strlen
	movq	%r14, %rdi
	movw	$125, (%r14,%rax)
	call	strdup
	movq	%r14, %rdi
	movq	%rax, 8(%rsp)
	call	free
	movq	8(%rsp), %rax
	movq	552(%rsp), %r14
	.cfi_restore 14
	jmp	.L164
	.p2align 4,,10
	.p2align 3
.L163:
	movq	584(%rsp), %rcx
	leaq	32(%rsp), %rdi
	movl	$.LC5, %edx
	xorl	%eax, %eax
	movl	$512, %esi
	call	snprintf
	leaq	32(%rsp), %rdi
	call	strdup
	jmp	.L164
	.p2align 4,,10
	.p2align 3
.L162:
	leaq	32(%rsp), %rdi
	movl	$.LC6, %edx
	movl	$512, %esi
	vmovsd	584(%rsp), %xmm0
	movl	$1, %eax
	call	snprintf
	leaq	32(%rsp), %rdi
	call	strdup
	jmp	.L164
	.p2align 4,,10
	.p2align 3
.L161:
	movl	584(%rsp), %esi
	movl	$.LC4, %edi
	movl	$.LC3, %eax
	testl	%esi, %esi
	cmovne	%rax, %rdi
	call	strdup
	jmp	.L164
	.p2align 4,,10
	.p2align 3
.L160:
	movq	584(%rsp), %rdi
	call	strdup
	jmp	.L164
	.p2align 4,,10
	.p2align 3
.L159:
	movq	584(%rsp), %rax
	movq	%r15, 560(%rsp)
	testq	%rax, %rax
	.cfi_offset 15, -16
	movq	%rax, %r15
	je	.L166
	movl	8(%rax), %edx
	testl	%edx, %edx
	je	.L166
	movl	$1024, %esi
	movl	$1, %edi
	movq	%r14, 552(%rsp)
	.cfi_offset 14, -24
	movl	%edx, 8(%rsp)
	call	calloc
	movl	8(%rsp), %ecx
	movw	$91, (%rax)
	movq	%rax, %r14
	testl	%ecx, %ecx
	jle	.L168
	xorl	%edx, %edx
.L171:
	leaq	(%rdx,%rdx,2), %rax
	subq	$32, %rsp
	.cfi_def_cfa_offset 608
	salq	$3, %rax
	addq	(%r15), %rax
	movq	%rdx, 40(%rsp)
	leaq	64(%rsp), %rdi
	vmovdqu	(%rax), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%rax), %rax
	movq	%rax, 16(%rsp)
	call	aly_to_str
	addq	$32, %rsp
	.cfi_def_cfa_offset 576
	cmpq	$0, 8(%rsp)
	jne	.L188
	movq	40(%rsp), %rsi
	movq	%r14, %rdi
	call	strcat
	subq	$32, %rsp
	.cfi_def_cfa_offset 608
	movq	80(%rsp), %rax
	vmovdqa	64(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 576
	cmpl	$1, 8(%r15)
	jle	.L168
	movl	$1, %edx
	jmp	.L171
	.p2align 4,,10
	.p2align 3
.L188:
	movq	%r14, %rdi
	call	strlen
	movq	40(%rsp), %rsi
	movw	$8236, (%r14,%rax)
	leaq	2(%r14,%rax), %rdi
	movb	$0, 2(%r14,%rax)
	call	strcpy
	subq	$32, %rsp
	.cfi_def_cfa_offset 608
	movq	80(%rsp), %rax
	vmovdqa	64(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	movq	40(%rsp), %rdx
	addq	$32, %rsp
	.cfi_def_cfa_offset 576
	addq	$1, %rdx
	cmpl	%edx, 8(%r15)
	jg	.L171
.L168:
	movq	%r14, %rdi
	call	strlen
	movq	%r14, %rdi
	movw	$93, (%r14,%rax)
	call	strdup
	movq	%r14, %rdi
	movq	%rax, 8(%rsp)
	call	free
	movq	8(%rsp), %rax
	movq	552(%rsp), %r14
	.cfi_restore 14
	movq	560(%rsp), %r15
	.cfi_restore 15
	jmp	.L164
	.p2align 4,,10
	.p2align 3
.L166:
	.cfi_offset 15, -16
	movl	$.LC8, %edi
	call	strdup
	movq	560(%rsp), %r15
	.cfi_restore 15
	jmp	.L164
	.p2align 4,,10
	.p2align 3
.L172:
	movl	$.LC9, %edi
	call	strdup
	jmp	.L164
	.p2align 4,,10
	.p2align 3
.L187:
	.cfi_offset 14, -24
	.cfi_offset 15, -16
	movq	%r14, %rdi
	movq	%rdx, 24(%rsp)
	movq	%rcx, 16(%rsp)
	movq	%r8, 8(%rsp)
	call	strlen
	movq	24(%rsp), %rdx
	movq	16(%rsp), %rcx
	movw	$8236, (%r14,%rax)
	movq	8(%rsp), %r8
	movb	$0, 2(%r14,%rax)
	jmp	.L176
	.cfi_endproc
.LFE37:
	.size	aly_to_str, .-aly_to_str
	.p2align 4
	.globl	native_print
	.type	native_print, @function
native_print:
.LFB106:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	testl	%esi, %esi
	movq	%rdi, %rcx
	jg	.L193
.L190:
	movl	$6, (%rcx)
	movq	%rcx, %rax
	movq	$0, 8(%rcx)
	movq	$0, 16(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L193:
	.cfi_restore_state
	cmpl	$3, (%rdx)
	je	.L194
	movq	16(%rdx), %rax
	vmovdqu	(%rdx), %xmm0
	movq	%rdi, 8(%rsp)
	leaq	16(%rsp), %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 96
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	56(%rsp), %rdi
	addq	$32, %rsp
	.cfi_def_cfa_offset 64
	call	puts
	subq	$32, %rsp
	.cfi_def_cfa_offset 96
	movq	64(%rsp), %rax
	vmovdqa	48(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 64
	movq	8(%rsp), %rcx
	jmp	.L190
	.p2align 4,,10
	.p2align 3
.L194:
	movq	%rdi, 8(%rsp)
	movq	8(%rdx), %rdi
	call	puts
	movq	8(%rsp), %rcx
	jmp	.L190
	.cfi_endproc
.LFE106:
	.size	native_print, .-native_print
	.p2align 4
	.globl	aly_band
	.type	aly_band, @function
aly_band:
.LFB38:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, %rcx
	movq	80(%rsp), %rax
	vmovdqu	64(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	64(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L196
	ja	.L197
	testl	%eax, %eax
	movq	72(%rsp), %r8
	je	.L201
	vcvttsd2siq	24(%rsp), %r8
.L201:
	movq	104(%rsp), %rax
	vmovdqu	88(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	88(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L202
	ja	.L203
	testl	%eax, %eax
	je	.L212
	vcvttsd2siq	24(%rsp), %rax
	andq	%rax, %r8
.L207:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r8, 8(%rcx)
	movq	$0, 16(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L197:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L201
	movq	%rdi, (%rsp)
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	call	strtoll
	movq	(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L201
	.p2align 4,,10
	.p2align 3
.L203:
	cmpl	$3, %eax
	jne	.L213
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	movq	%r8, (%rsp)
	movq	%rcx, 8(%rsp)
	call	strtoll
	movq	(%rsp), %r8
	movq	8(%rsp), %rcx
	andq	%rax, %r8
	jmp	.L207
	.p2align 4,,10
	.p2align 3
.L202:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	andq	%rax, %r8
	jmp	.L207
	.p2align 4,,10
	.p2align 3
.L196:
	movl	24(%rsp), %esi
	xorl	%r8d, %r8d
	testl	%esi, %esi
	setne	%r8b
	jmp	.L201
	.p2align 4,,10
	.p2align 3
.L212:
	andq	96(%rsp), %r8
	jmp	.L207
	.p2align 4,,10
	.p2align 3
.L213:
	xorl	%r8d, %r8d
	jmp	.L207
	.cfi_endproc
.LFE38:
	.size	aly_band, .-aly_band
	.p2align 4
	.globl	aly_bor
	.type	aly_bor, @function
aly_bor:
.LFB39:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, %rcx
	movq	80(%rsp), %rax
	vmovdqu	64(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	64(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L215
	ja	.L216
	testl	%eax, %eax
	movq	72(%rsp), %r8
	je	.L220
	vcvttsd2siq	24(%rsp), %r8
.L220:
	movq	104(%rsp), %rax
	vmovdqu	88(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	88(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L221
	ja	.L222
	testl	%eax, %eax
	je	.L231
	vcvttsd2siq	24(%rsp), %rax
	orq	%rax, %r8
.L226:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r8, 8(%rcx)
	movq	$0, 16(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L216:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L220
	movq	%rdi, (%rsp)
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	call	strtoll
	movq	(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L220
	.p2align 4,,10
	.p2align 3
.L222:
	cmpl	$3, %eax
	jne	.L226
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	movq	%r8, (%rsp)
	movq	%rcx, 8(%rsp)
	call	strtoll
	movq	(%rsp), %r8
	movq	8(%rsp), %rcx
	orq	%rax, %r8
	jmp	.L226
	.p2align 4,,10
	.p2align 3
.L221:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	orq	%rax, %r8
	jmp	.L226
	.p2align 4,,10
	.p2align 3
.L215:
	movl	24(%rsp), %esi
	xorl	%r8d, %r8d
	testl	%esi, %esi
	setne	%r8b
	jmp	.L220
	.p2align 4,,10
	.p2align 3
.L231:
	orq	96(%rsp), %r8
	jmp	.L226
	.cfi_endproc
.LFE39:
	.size	aly_bor, .-aly_bor
	.p2align 4
	.globl	aly_bxor
	.type	aly_bxor, @function
aly_bxor:
.LFB40:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, %rcx
	movq	80(%rsp), %rax
	vmovdqu	64(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	64(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L233
	ja	.L234
	testl	%eax, %eax
	movq	72(%rsp), %r8
	je	.L238
	vcvttsd2siq	24(%rsp), %r8
.L238:
	movq	104(%rsp), %rax
	vmovdqu	88(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	88(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L239
	ja	.L240
	testl	%eax, %eax
	je	.L249
	vcvttsd2siq	24(%rsp), %rax
	xorq	%rax, %r8
.L244:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r8, 8(%rcx)
	movq	$0, 16(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L234:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L238
	movq	%rdi, (%rsp)
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	call	strtoll
	movq	(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L238
	.p2align 4,,10
	.p2align 3
.L240:
	cmpl	$3, %eax
	jne	.L244
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	movq	%r8, (%rsp)
	movq	%rcx, 8(%rsp)
	call	strtoll
	movq	(%rsp), %r8
	movq	8(%rsp), %rcx
	xorq	%rax, %r8
	jmp	.L244
	.p2align 4,,10
	.p2align 3
.L239:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	xorq	%rax, %r8
	jmp	.L244
	.p2align 4,,10
	.p2align 3
.L233:
	movl	24(%rsp), %esi
	xorl	%r8d, %r8d
	testl	%esi, %esi
	setne	%r8b
	jmp	.L238
	.p2align 4,,10
	.p2align 3
.L249:
	xorq	96(%rsp), %r8
	jmp	.L244
	.cfi_endproc
.LFE40:
	.size	aly_bxor, .-aly_bxor
	.p2align 4
	.globl	aly_bnot
	.type	aly_bnot, @function
aly_bnot:
.LFB41:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, %rcx
	movl	64(%rsp), %eax
	vmovdqu	64(%rsp), %xmm0
	cmpl	$2, %eax
	vmovdqa	%xmm0, 16(%rsp)
	je	.L251
	ja	.L252
	testl	%eax, %eax
	je	.L259
	vcvttsd2siq	24(%rsp), %rax
	notq	%rax
.L256:
	movq	%rax, 8(%rcx)
	movq	%rcx, %rax
	movl	$0, (%rcx)
	movq	$0, 16(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L252:
	.cfi_restore_state
	cmpl	$3, %eax
	jne	.L260
	movq	%rdi, 8(%rsp)
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	call	strtoll
	movq	8(%rsp), %rcx
	notq	%rax
	jmp	.L256
	.p2align 4,,10
	.p2align 3
.L251:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	notq	%rax
	jmp	.L256
	.p2align 4,,10
	.p2align 3
.L259:
	movq	72(%rsp), %rax
	notq	%rax
	jmp	.L256
	.p2align 4,,10
	.p2align 3
.L260:
	movq	$-1, %rax
	jmp	.L256
	.cfi_endproc
.LFE41:
	.size	aly_bnot, .-aly_bnot
	.p2align 4
	.globl	aly_shl
	.type	aly_shl, @function
aly_shl:
.LFB42:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, %rcx
	movq	80(%rsp), %rax
	vmovdqu	64(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	64(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L262
	ja	.L263
	testl	%eax, %eax
	movq	72(%rsp), %r8
	je	.L267
	vcvttsd2siq	24(%rsp), %r8
.L267:
	movq	104(%rsp), %rax
	vmovdqu	88(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	88(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L268
	ja	.L269
	testl	%eax, %eax
	je	.L278
	vcvttsd2siq	24(%rsp), %rax
	shlx	%rax, %r8, %r8
.L273:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r8, 8(%rcx)
	movq	$0, 16(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L263:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L267
	movq	%rdi, (%rsp)
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	call	strtoll
	movq	(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L267
	.p2align 4,,10
	.p2align 3
.L269:
	cmpl	$3, %eax
	jne	.L273
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	movq	%r8, (%rsp)
	movq	%rcx, 8(%rsp)
	call	strtoll
	movq	(%rsp), %r8
	movq	8(%rsp), %rcx
	shlx	%rax, %r8, %r8
	jmp	.L273
	.p2align 4,,10
	.p2align 3
.L268:
	movl	24(%rsp), %eax
	testl	%eax, %eax
	setne	%al
	shlx	%rax, %r8, %r8
	jmp	.L273
	.p2align 4,,10
	.p2align 3
.L262:
	movl	24(%rsp), %edx
	xorl	%r8d, %r8d
	testl	%edx, %edx
	setne	%r8b
	jmp	.L267
	.p2align 4,,10
	.p2align 3
.L278:
	movq	96(%rsp), %rax
	shlx	%rax, %r8, %r8
	jmp	.L273
	.cfi_endproc
.LFE42:
	.size	aly_shl, .-aly_shl
	.p2align 4
	.globl	aly_shr
	.type	aly_shr, @function
aly_shr:
.LFB43:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, %rcx
	movq	80(%rsp), %rax
	vmovdqu	64(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	64(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L280
	ja	.L281
	testl	%eax, %eax
	movq	72(%rsp), %r8
	je	.L285
	vcvttsd2siq	24(%rsp), %r8
.L285:
	movq	104(%rsp), %rax
	vmovdqu	88(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	88(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L286
	ja	.L287
	testl	%eax, %eax
	je	.L296
	vcvttsd2siq	24(%rsp), %rax
	sarx	%rax, %r8, %r8
.L291:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r8, 8(%rcx)
	movq	$0, 16(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L281:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L285
	movq	%rdi, (%rsp)
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	call	strtoll
	movq	(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L285
	.p2align 4,,10
	.p2align 3
.L287:
	cmpl	$3, %eax
	jne	.L291
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	movq	%r8, (%rsp)
	movq	%rcx, 8(%rsp)
	call	strtoll
	movq	(%rsp), %r8
	movq	8(%rsp), %rcx
	sarx	%rax, %r8, %r8
	jmp	.L291
	.p2align 4,,10
	.p2align 3
.L286:
	movl	24(%rsp), %eax
	testl	%eax, %eax
	setne	%al
	sarx	%rax, %r8, %r8
	jmp	.L291
	.p2align 4,,10
	.p2align 3
.L280:
	movl	24(%rsp), %edx
	xorl	%r8d, %r8d
	testl	%edx, %edx
	setne	%r8b
	jmp	.L285
	.p2align 4,,10
	.p2align 3
.L296:
	movq	96(%rsp), %rax
	sarx	%rax, %r8, %r8
	jmp	.L291
	.cfi_endproc
.LFE43:
	.size	aly_shr, .-aly_shr
	.section	.rodata.str1.1
.LC10:
	.string	"%s%s"
	.text
	.p2align 4
	.globl	aly_add
	.type	aly_add, @function
aly_add:
.LFB44:
	.cfi_startproc
	subq	$104, %rsp
	.cfi_def_cfa_offset 112
	movq	%rdi, %r9
	movl	136(%rsp), %edx
	movl	112(%rsp), %eax
	cmpl	$3, %edx
	je	.L313
	cmpl	$3, %eax
	je	.L313
	cmpl	$1, %eax
	vxorps	%xmm0, %xmm0, %xmm0
	je	.L301
	cmpl	$1, %edx
	je	.L318
	vmovdqu	112(%rsp), %xmm0
	cmpl	$2, %eax
	vmovdqa	%xmm0, 48(%rsp)
	je	.L307
	cmpl	$3, %eax
	movl	$0, %eax
	cmovb	120(%rsp), %rax
.L308:
	vmovdqu	136(%rsp), %xmm0
	cmpl	$2, %edx
	vmovdqa	%xmm0, 48(%rsp)
	je	.L309
	ja	.L310
	addq	144(%rsp), %rax
.L310:
	movq	%rax, 8(%r9)
	movq	%r9, %rax
	movl	$0, (%r9)
	movq	$0, 16(%r9)
	addq	$104, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L318:
	.cfi_restore_state
	testl	%eax, %eax
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L303
	vcvtsi2sdq	120(%rsp), %xmm0, %xmm0
	vmovapd	%xmm0, %xmm1
.L303:
	vmovsd	144(%rsp), %xmm0
	jmp	.L306
	.p2align 4,,10
	.p2align 3
.L313:
	movq	%r9, 8(%rsp)
	leaq	16(%rsp), %rdi
	movq	%rbx, 80(%rsp)
	movq	%r14, 88(%rsp)
	movq	%r15, 96(%rsp)
	.cfi_offset 3, -32
	.cfi_offset 14, -24
	.cfi_offset 15, -16
	subq	$32, %rsp
	.cfi_def_cfa_offset 144
	movq	160(%rsp), %rax
	vmovdqu	144(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	56(%rsp), %rbx
	leaq	80(%rsp), %rdi
	movq	184(%rsp), %rax
	vmovdqu	168(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	88(%rsp), %r14
	movq	%rbx, %rdi
	addq	$32, %rsp
	.cfi_def_cfa_offset 112
	call	strlen
	movq	%r14, %rdi
	movq	%rax, %r15
	call	strlen
	leal	1(%r15,%rax), %esi
	movslq	%esi, %rsi
	movq	%rsi, %rdi
	movq	%rsi, (%rsp)
	call	malloc
	movq	(%rsp), %rsi
	movq	%r14, %r8
	movq	%rbx, %rcx
	movl	$.LC10, %edx
	movq	%rax, %rdi
	movq	%rax, %rbx
	xorl	%eax, %eax
	call	snprintf
	subq	$32, %rsp
	.cfi_def_cfa_offset 144
	movq	64(%rsp), %rax
	vmovdqa	48(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	movq	96(%rsp), %rax
	vmovdqa	80(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	%rbx, %rdi
	call	strdup
	movq	%rbx, %rdi
	movq	%rax, (%rsp)
	call	free
	movq	8(%rsp), %r9
	movq	(%rsp), %rax
	movq	80(%rsp), %rbx
	.cfi_restore 3
	movq	88(%rsp), %r14
	.cfi_restore 14
	movq	%rax, 8(%r9)
	movq	96(%rsp), %r15
	.cfi_restore 15
	movq	%r9, %rax
	movl	$3, (%r9)
	movq	$0, 16(%r9)
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L301:
	.cfi_def_cfa_offset 112
	cmpl	$3, %edx
	vmovsd	120(%rsp), %xmm1
	ja	.L304
	testl	%edx, %edx
	jne	.L319
	vcvtsi2sdq	144(%rsp), %xmm0, %xmm0
.L306:
	vaddsd	%xmm1, %xmm0, %xmm0
	movl	$1, (%r9)
	movq	%r9, %rax
	movq	$0, 16(%r9)
	vmovsd	%xmm0, 8(%r9)
	addq	$104, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L319:
	.cfi_restore_state
	cmpl	$1, %edx
	je	.L303
.L304:
	vxorpd	%xmm0, %xmm0, %xmm0
	jmp	.L306
	.p2align 4,,10
	.p2align 3
.L309:
	cmpl	$1, 56(%rsp)
	sbbq	$-1, %rax
	jmp	.L310
	.p2align 4,,10
	.p2align 3
.L307:
	movl	56(%rsp), %ecx
	xorl	%eax, %eax
	testl	%ecx, %ecx
	setne	%al
	jmp	.L308
	.cfi_endproc
.LFE44:
	.size	aly_add, .-aly_add
	.p2align 4
	.globl	aly_sub
	.type	aly_sub, @function
aly_sub:
.LFB45:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	vxorps	%xmm0, %xmm0, %xmm0
	movq	%rdi, %rcx
	movl	64(%rsp), %eax
	movl	88(%rsp), %r8d
	cmpl	$1, %eax
	je	.L321
	cmpl	$1, %r8d
	je	.L344
	vmovdqu	64(%rsp), %xmm0
	movq	80(%rsp), %rdx
	cmpl	$2, %eax
	movq	%rdx, 32(%rsp)
	vmovdqa	%xmm0, 16(%rsp)
	je	.L331
	movq	72(%rsp), %r9
	ja	.L345
.L334:
	vmovdqu	88(%rsp), %xmm0
	movq	104(%rsp), %rax
	cmpl	$2, %r8d
	movq	%rax, 32(%rsp)
	vmovdqa	%xmm0, 16(%rsp)
	je	.L335
	ja	.L346
	subq	96(%rsp), %r9
.L338:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r9, 8(%rcx)
	movq	$0, 16(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L321:
	.cfi_restore_state
	cmpl	$3, %r8d
	vmovsd	72(%rsp), %xmm1
	movq	96(%rsp), %rdi
	je	.L325
	ja	.L327
	testl	%r8d, %r8d
	jne	.L347
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vsubsd	%xmm0, %xmm1, %xmm1
.L327:
	movl	$1, (%rcx)
	vmovsd	%xmm1, 8(%rcx)
.L348:
	movq	$0, 16(%rcx)
	movq	%rcx, %rax
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L344:
	.cfi_restore_state
	cmpl	$3, %eax
	movq	72(%rsp), %rdi
	je	.L323
	testl	%eax, %eax
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L324
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vmovapd	%xmm0, %xmm1
.L324:
	vsubsd	96(%rsp), %xmm1, %xmm1
	movl	$1, (%rcx)
	vmovsd	%xmm1, 8(%rcx)
	jmp	.L348
	.p2align 4,,10
	.p2align 3
.L331:
	movl	24(%rsp), %eax
	xorl	%r9d, %r9d
	testl	%eax, %eax
	setne	%r9b
	jmp	.L334
	.p2align 4,,10
	.p2align 3
.L345:
	xorl	%r9d, %r9d
	cmpl	$3, %eax
	jne	.L334
	movq	%rdi, 8(%rsp)
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	movl	%r8d, (%rsp)
	call	strtoll
	movq	8(%rsp), %rcx
	movl	(%rsp), %r8d
	movq	%rax, %r9
	jmp	.L334
	.p2align 4,,10
	.p2align 3
.L346:
	cmpl	$3, %r8d
	jne	.L338
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	movq	%r9, (%rsp)
	movq	%rcx, 8(%rsp)
	call	strtoll
	movq	(%rsp), %r9
	movq	8(%rsp), %rcx
	subq	%rax, %r9
	jmp	.L338
	.p2align 4,,10
	.p2align 3
.L335:
	cmpl	$1, 24(%rsp)
	adcq	$-1, %r9
	jmp	.L338
	.p2align 4,,10
	.p2align 3
.L323:
	xorl	%esi, %esi
	movq	%rcx, (%rsp)
	call	strtod
	movq	(%rsp), %rcx
	vmovapd	%xmm0, %xmm1
	jmp	.L324
	.p2align 4,,10
	.p2align 3
.L325:
	xorl	%esi, %esi
	movq	%rcx, 8(%rsp)
	vmovsd	%xmm1, (%rsp)
	call	strtod
	vmovsd	(%rsp), %xmm1
	movq	8(%rsp), %rcx
	vsubsd	%xmm0, %xmm1, %xmm1
	movl	$1, (%rcx)
	vmovsd	%xmm1, 8(%rcx)
	jmp	.L348
	.p2align 4,,10
	.p2align 3
.L347:
	cmpl	$1, %r8d
	jne	.L327
	jmp	.L324
	.cfi_endproc
.LFE45:
	.size	aly_sub, .-aly_sub
	.p2align 4
	.globl	aly_mul
	.type	aly_mul, @function
aly_mul:
.LFB46:
	.cfi_startproc
	subq	$88, %rsp
	.cfi_def_cfa_offset 96
	movl	96(%rsp), %eax
	movl	120(%rsp), %ecx
	movq	%rbx, 56(%rsp)
	.cfi_offset 3, -40
	movq	%rdi, %rbx
	movq	%rbp, 64(%rsp)
	.cfi_offset 6, -32
	movq	128(%rsp), %rbp
	cmpl	$3, %eax
	je	.L386
	cmpl	$1, %eax
	vxorps	%xmm0, %xmm0, %xmm0
	je	.L358
	cmpl	$1, %ecx
	je	.L359
	vmovdqu	96(%rsp), %xmm0
	cmpl	$1, %eax
	vmovdqa	%xmm0, 16(%rsp)
	jbe	.L387
	xorl	%r8d, %r8d
	cmpl	$2, %eax
	jne	.L369
	movl	24(%rsp), %edx
	xorl	%r8d, %r8d
	testl	%edx, %edx
	setne	%r8b
.L369:
	vmovdqu	120(%rsp), %xmm0
	movq	136(%rsp), %rax
	cmpl	$2, %ecx
	movq	%rax, 32(%rsp)
	vmovdqa	%xmm0, 16(%rsp)
	je	.L353
	imulq	%r8, %rbp
	cmpl	$2, %ecx
	ja	.L354
.L372:
	movl	$0, (%rbx)
	movq	%rbp, 8(%rbx)
	movq	$0, 16(%rbx)
	jmp	.L349
	.p2align 4,,10
	.p2align 3
.L386:
	testl	%ecx, %ecx
	je	.L351
	cmpl	$1, %ecx
	je	.L352
	movq	112(%rsp), %rax
	xorl	%esi, %esi
	movl	$10, %edx
	movl	%ecx, 8(%rsp)
	vmovdqu	96(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	vmovdqa	%xmm0, 16(%rsp)
	movq	24(%rsp), %rdi
	call	strtoll
	movl	8(%rsp), %ecx
	vmovdqu	120(%rsp), %xmm0
	movq	%rax, %r8
	movq	136(%rsp), %rax
	cmpl	$2, %ecx
	vmovdqa	%xmm0, 16(%rsp)
	movq	%rax, 32(%rsp)
	je	.L353
.L354:
	xorl	%ebp, %ebp
	cmpl	$3, %ecx
	jne	.L372
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	movq	%r8, 8(%rsp)
	call	strtoll
	imulq	8(%rsp), %rax
	movq	%rax, %rbp
	jmp	.L372
	.p2align 4,,10
	.p2align 3
.L358:
	cmpl	$3, %ecx
	vmovsd	104(%rsp), %xmm1
	je	.L363
	ja	.L367
	testl	%ecx, %ecx
	jne	.L388
	vcvtsi2sdq	%rbp, %xmm0, %xmm0
.L365:
	vmulsd	%xmm1, %xmm0, %xmm0
	movl	$1, (%rbx)
	movq	$0, 16(%rbx)
	vmovsd	%xmm0, 8(%rbx)
.L349:
	movq	%rbx, %rax
	movq	64(%rsp), %rbp
	movq	56(%rsp), %rbx
	addq	$88, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L359:
	.cfi_restore_state
	testl	%eax, %eax
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L362
	vcvtsi2sdq	104(%rsp), %xmm0, %xmm0
	vmovapd	%xmm0, %xmm1
.L362:
	vmovq	%rbp, %xmm0
	jmp	.L365
	.p2align 4,,10
	.p2align 3
.L387:
	movq	104(%rsp), %r8
	jmp	.L369
	.p2align 4,,10
	.p2align 3
.L363:
	xorl	%esi, %esi
	movq	%rbp, %rdi
	vmovsd	%xmm1, 8(%rsp)
	call	strtod
	vmovsd	8(%rsp), %xmm1
	jmp	.L365
	.p2align 4,,10
	.p2align 3
.L352:
	movq	104(%rsp), %rdi
	xorl	%esi, %esi
	call	strtod
	vmovapd	%xmm0, %xmm1
	jmp	.L362
	.p2align 4,,10
	.p2align 3
.L353:
	movl	24(%rsp), %eax
	xorl	%ebp, %ebp
	testl	%eax, %eax
	cmovne	%r8, %rbp
	jmp	.L372
	.p2align 4,,10
	.p2align 3
.L388:
	cmpl	$1, %ecx
	je	.L362
.L367:
	vxorpd	%xmm0, %xmm0, %xmm0
	jmp	.L365
	.p2align 4,,10
	.p2align 3
.L351:
	movq	%r14, 72(%rsp)
	leaq	16(%rsp), %rdi
	movq	%r15, 80(%rsp)
	.cfi_offset 14, -24
	.cfi_offset 15, -16
	subq	$32, %rsp
	.cfi_def_cfa_offset 128
	movq	144(%rsp), %rax
	vmovdqu	128(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	56(%rsp), %rdi
	movq	56(%rsp), %r15
	addq	$32, %rsp
	.cfi_def_cfa_offset 96
	call	strlen
	movl	%eax, %edi
	imull	%ebp, %edi
	addl	$1, %edi
	movslq	%edi, %rdi
	call	malloc
	testl	%ebp, %ebp
	movb	$0, (%rax)
	movq	%rax, %r14
	jle	.L355
	xorl	%edx, %edx
	.p2align 4,,10
	.p2align 3
.L356:
	movq	%r15, %rsi
	movq	%r14, %rdi
	movl	%edx, 8(%rsp)
	call	strcat
	movl	8(%rsp), %edx
	addl	$1, %edx
	cmpl	%edx, %ebp
	jne	.L356
.L355:
	subq	$32, %rsp
	.cfi_def_cfa_offset 128
	movq	64(%rsp), %rax
	vmovdqa	48(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 96
	movq	%r14, %rdi
	call	strdup
	movq	%r14, %rdi
	movq	%rax, %rbp
	call	free
	movl	$3, (%rbx)
	movq	72(%rsp), %r14
	.cfi_restore 14
	movq	%rbp, 8(%rbx)
	movq	80(%rsp), %r15
	.cfi_restore 15
	movq	$0, 16(%rbx)
	jmp	.L349
	.cfi_endproc
.LFE46:
	.size	aly_mul, .-aly_mul
	.section	.rodata.str1.1
.LC11:
	.string	"Error: Division by zero\n"
	.text
	.p2align 4
	.globl	aly_div
	.type	aly_div, @function
aly_div:
.LFB47:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	vxorps	%xmm2, %xmm2, %xmm2
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	vmovq	%rdi, %xmm1
	je	.L392
	cmpl	$3, %edx
	je	.L391
	testl	%edx, %edx
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L392
	vcvtsi2sdq	%rdi, %xmm2, %xmm1
.L392:
	movl	56(%rsp), %edx
	movq	64(%rsp), %rdi
	cmpl	$1, %edx
	je	.L393
.L402:
	cmpl	$3, %edx
	je	.L394
	testl	%edx, %edx
	je	.L401
.L395:
	movl	$.LC11, %edi
	movl	$24, %edx
	movl	$1, %esi
	movq	stderr(%rip), %rcx
	call	fwrite
	movl	$1, %edi
	call	exit
	.p2align 4,,10
	.p2align 3
.L391:
	xorl	%esi, %esi
	movq	%rax, (%rsp)
	call	strtod
	movl	56(%rsp), %edx
	movq	(%rsp), %rax
	vxorps	%xmm2, %xmm2, %xmm2
	movq	64(%rsp), %rdi
	vmovapd	%xmm0, %xmm1
	cmpl	$1, %edx
	jne	.L402
.L393:
	vmovq	%rdi, %xmm0
	jmp	.L396
	.p2align 4,,10
	.p2align 3
.L394:
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	vmovsd	%xmm1, (%rsp)
	call	strtod
	movq	8(%rsp), %rax
	vmovsd	(%rsp), %xmm1
.L396:
	vxorpd	%xmm2, %xmm2, %xmm2
	vucomisd	%xmm2, %xmm0
	jp	.L397
	je	.L395
.L397:
	vdivsd	%xmm0, %xmm1, %xmm1
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	vmovsd	%xmm1, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L401:
	.cfi_restore_state
	vcvtsi2sdq	%rdi, %xmm2, %xmm2
	vmovapd	%xmm2, %xmm0
	jmp	.L396
	.cfi_endproc
.LFE47:
	.size	aly_div, .-aly_div
	.p2align 4
	.globl	aly_mod
	.type	aly_mod, @function
aly_mod:
.LFB48:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, %rcx
	movl	48(%rsp), %edx
	movl	72(%rsp), %esi
	movq	56(%rsp), %rax
	movq	80(%rsp), %rdi
	movl	%edx, %r8d
	orl	%esi, %r8d
	jne	.L404
	testq	%rdi, %rdi
	je	.L419
	cqto
	movl	$0, (%rcx)
	idivq	%rdi
	movq	$0, 16(%rcx)
	movq	%rcx, %rax
	movq	%rdx, 8(%rcx)
	addq	$40, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L404:
	.cfi_restore_state
	cmpl	$1, %esi
	vxorps	%xmm1, %xmm1, %xmm1
	je	.L407
	cmpl	$3, %esi
	je	.L408
	movq	$0x000000000, 8(%rsp)
	testl	%esi, %esi
	je	.L420
.L409:
	cmpl	$1, %edx
	je	.L410
.L423:
	cmpl	$3, %edx
	je	.L411
	testl	%edx, %edx
	movq	$0x000000000, (%rsp)
	je	.L421
.L412:
	fldl	8(%rsp)
	fldl	(%rsp)
.L413:
	fprem
	fnstsw	%ax
	sahf
	jp	.L413
	fstp	%st(1)
	fstpl	16(%rsp)
	vmovsd	16(%rsp), %xmm2
	vucomisd	%xmm2, %xmm2
	jp	.L422
.L414:
	movl	$1, (%rcx)
	movq	%rcx, %rax
	movq	$0, 16(%rcx)
	vmovsd	%xmm2, 8(%rcx)
	addq	$40, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L421:
	.cfi_restore_state
	vcvtsi2sdq	%rax, %xmm1, %xmm1
	vmovlpd	%xmm1, (%rsp)
	jmp	.L412
	.p2align 4,,10
	.p2align 3
.L420:
	cmpl	$1, %edx
	vcvtsi2sdq	%rdi, %xmm1, %xmm0
	vmovlpd	%xmm0, 8(%rsp)
	jne	.L423
.L410:
	movq	%rax, (%rsp)
	jmp	.L412
	.p2align 4,,10
	.p2align 3
.L407:
	movq	%rdi, 8(%rsp)
	jmp	.L409
	.p2align 4,,10
	.p2align 3
.L408:
	xorl	%esi, %esi
	movq	%rcx, 24(%rsp)
	movq	%rax, 16(%rsp)
	movl	%edx, (%rsp)
	call	strtod
	movq	24(%rsp), %rcx
	movl	(%rsp), %edx
	vxorps	%xmm1, %xmm1, %xmm1
	movq	16(%rsp), %rax
	vmovsd	%xmm0, 8(%rsp)
	jmp	.L409
	.p2align 4,,10
	.p2align 3
.L411:
	xorl	%esi, %esi
	movq	%rax, %rdi
	movq	%rcx, 16(%rsp)
	call	strtod
	movq	16(%rsp), %rcx
	vmovsd	%xmm0, (%rsp)
	jmp	.L412
.L419:
	call	aly_mod.part.0
.L422:
	vmovsd	8(%rsp), %xmm1
	vmovsd	(%rsp), %xmm0
	movq	%rcx, 24(%rsp)
	call	fmod
	movq	24(%rsp), %rcx
	vmovsd	16(%rsp), %xmm2
	jmp	.L414
	.cfi_endproc
.LFE48:
	.size	aly_mod, .-aly_mod
	.p2align 4
	.globl	aly_neg
	.type	aly_neg, @function
aly_neg:
.LFB49:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, %rcx
	movl	64(%rsp), %eax
	vmovsd	72(%rsp), %xmm0
	cmpl	$1, %eax
	je	.L432
	vmovdqu	64(%rsp), %xmm1
	cmpl	$2, %eax
	vmovdqa	%xmm1, 16(%rsp)
	je	.L427
	ja	.L433
	vmovq	%xmm0, %rax
	negq	%rax
.L430:
	movq	%rax, 8(%rcx)
	movq	%rcx, %rax
	movl	$0, (%rcx)
	movq	$0, 16(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L432:
	.cfi_restore_state
	vxorpd	.LC13(%rip), %xmm0, %xmm0
	movl	$1, (%rdi)
	movq	%rcx, %rax
	movq	$0, 16(%rcx)
	vmovsd	%xmm0, 8(%rdi)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L433:
	.cfi_restore_state
	cmpl	$3, %eax
	jne	.L434
	movq	%rdi, 8(%rsp)
	movl	$10, %edx
	xorl	%esi, %esi
	vmovq	%xmm0, %rdi
	call	strtoll
	movq	8(%rsp), %rcx
	negq	%rax
	jmp	.L430
	.p2align 4,,10
	.p2align 3
.L427:
	movl	24(%rsp), %eax
	negl	%eax
	sbbq	%rax, %rax
	jmp	.L430
	.p2align 4,,10
	.p2align 3
.L434:
	xorl	%eax, %eax
	jmp	.L430
	.cfi_endproc
.LFE49:
	.size	aly_neg, .-aly_neg
	.p2align 4
	.globl	aly_not
	.type	aly_not, @function
aly_not:
.LFB50:
	.cfi_startproc
	vmovdqu	8(%rsp), %xmm0
	movq	24(%rsp), %rdx
	movq	%rdi, %rax
	cmpl	$5, 8(%rsp)
	movq	%rdx, -24(%rsp)
	vmovdqa	%xmm0, -40(%rsp)
	ja	.L436
	movl	8(%rsp), %edx
	jmp	*.L438(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L438:
	.quad	.L443
	.quad	.L442
	.quad	.L441
	.quad	.L440
	.quad	.L439
	.quad	.L439
	.text
	.p2align 4,,10
	.p2align 3
.L439:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L436
	movl	8(%rdx), %edx
	testl	%edx, %edx
	setg	%dl
	movzbl	%dl, %edx
.L444:
	testl	%edx, %edx
	movl	$2, (%rax)
	sete	%dl
	movq	$0, 16(%rax)
	movzbl	%dl, %edx
	movl	%edx, 8(%rax)
	ret
	.p2align 4,,10
	.p2align 3
.L443:
	xorl	%edx, %edx
	cmpq	$0, -32(%rsp)
	setne	%dl
	jmp	.L444
	.p2align 4,,10
	.p2align 3
.L442:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-32(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L444
	.p2align 4,,10
	.p2align 3
.L441:
	movl	-32(%rsp), %edx
	jmp	.L444
	.p2align 4,,10
	.p2align 3
.L440:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L436
	cmpb	$0, (%rdx)
	setne	%dl
	movzbl	%dl, %edx
	jmp	.L444
	.p2align 4,,10
	.p2align 3
.L436:
	xorl	%edx, %edx
	jmp	.L444
	.cfi_endproc
.LFE50:
	.size	aly_not, .-aly_not
	.p2align 4
	.globl	aly_and
	.type	aly_and, @function
aly_and:
.LFB51:
	.cfi_startproc
	vmovdqu	8(%rsp), %xmm0
	movq	24(%rsp), %rdx
	movq	%rdi, %rax
	cmpl	$5, 8(%rsp)
	movq	%rdx, -24(%rsp)
	vmovdqa	%xmm0, -40(%rsp)
	ja	.L462
	movl	8(%rsp), %edx
	jmp	*.L456(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L456:
	.quad	.L461
	.quad	.L460
	.quad	.L459
	.quad	.L458
	.quad	.L455
	.quad	.L455
	.text
	.p2align 4,,10
	.p2align 3
.L455:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	jne	.L504
.L462:
	xorl	%edx, %edx
.L467:
	movl	$2, (%rax)
	movl	%edx, 8(%rax)
	movq	$0, 16(%rax)
	ret
	.p2align 4,,10
	.p2align 3
.L504:
	movl	8(%rdx), %ecx
	testl	%ecx, %ecx
	jle	.L462
.L466:
	vmovdqu	32(%rsp), %xmm0
	movq	48(%rsp), %rdx
	cmpl	$5, 32(%rsp)
	movq	%rdx, -24(%rsp)
	vmovdqa	%xmm0, -40(%rsp)
	ja	.L462
	movl	32(%rsp), %edx
	jmp	*.L470(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L470:
	.quad	.L475
	.quad	.L474
	.quad	.L473
	.quad	.L472
	.quad	.L469
	.quad	.L469
	.text
	.p2align 4,,10
	.p2align 3
.L469:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L462
	movl	8(%rdx), %edx
	testl	%edx, %edx
	jle	.L462
.L478:
	movl	$1, %edx
	jmp	.L467
	.p2align 4,,10
	.p2align 3
.L461:
	xorl	%edx, %edx
	cmpq	$0, -32(%rsp)
	setne	%dl
.L463:
	testl	%edx, %edx
	jne	.L466
	jmp	.L467
	.p2align 4,,10
	.p2align 3
.L460:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-32(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L463
	.p2align 4,,10
	.p2align 3
.L459:
	movl	-32(%rsp), %edx
	jmp	.L463
	.p2align 4,,10
	.p2align 3
.L458:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L462
	cmpb	$0, (%rdx)
	jne	.L466
	jmp	.L462
	.p2align 4,,10
	.p2align 3
.L472:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L462
	cmpb	$0, (%rdx)
	jne	.L478
	jmp	.L462
	.p2align 4,,10
	.p2align 3
.L473:
	movl	-32(%rsp), %edx
.L476:
	testl	%edx, %edx
	setne	%dl
	movzbl	%dl, %edx
	jmp	.L467
	.p2align 4,,10
	.p2align 3
.L474:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-32(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L476
	.p2align 4,,10
	.p2align 3
.L475:
	xorl	%edx, %edx
	cmpq	$0, -32(%rsp)
	setne	%dl
	jmp	.L476
	.cfi_endproc
.LFE51:
	.size	aly_and, .-aly_and
	.p2align 4
	.globl	aly_or
	.type	aly_or, @function
aly_or:
.LFB52:
	.cfi_startproc
	vmovdqu	8(%rsp), %xmm0
	movq	24(%rsp), %rdx
	movq	%rdi, %rax
	cmpl	$5, 8(%rsp)
	movq	%rdx, -24(%rsp)
	vmovdqa	%xmm0, -40(%rsp)
	ja	.L514
	movl	8(%rsp), %edx
	jmp	*.L508(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L508:
	.quad	.L513
	.quad	.L512
	.quad	.L511
	.quad	.L510
	.quad	.L507
	.quad	.L507
	.text
	.p2align 4,,10
	.p2align 3
.L513:
	xorl	%edx, %edx
	cmpq	$0, -32(%rsp)
	setne	%dl
.L515:
	testl	%edx, %edx
	jne	.L518
.L514:
	vmovdqu	32(%rsp), %xmm0
	movq	48(%rsp), %rdx
	cmpl	$5, 32(%rsp)
	movq	%rdx, -24(%rsp)
	vmovdqa	%xmm0, -40(%rsp)
	ja	.L531
	movl	32(%rsp), %edx
	jmp	*.L521(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L521:
	.quad	.L526
	.quad	.L525
	.quad	.L524
	.quad	.L523
	.quad	.L520
	.quad	.L520
	.text
	.p2align 4,,10
	.p2align 3
.L507:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L514
	movl	8(%rdx), %ecx
	testl	%ecx, %ecx
	jle	.L514
.L518:
	movl	$1, %edx
	jmp	.L530
	.p2align 4,,10
	.p2align 3
.L520:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L531
	movl	8(%rdx), %edx
	testl	%edx, %edx
	jg	.L518
.L531:
	xorl	%edx, %edx
.L530:
	movl	$2, (%rax)
	movl	%edx, 8(%rax)
	movq	$0, 16(%rax)
	ret
	.p2align 4,,10
	.p2align 3
.L512:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-32(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L515
	.p2align 4,,10
	.p2align 3
.L511:
	movl	-32(%rsp), %edx
	jmp	.L515
	.p2align 4,,10
	.p2align 3
.L510:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L514
	cmpb	$0, (%rdx)
	jne	.L518
	jmp	.L514
	.p2align 4,,10
	.p2align 3
.L526:
	xorl	%edx, %edx
	cmpq	$0, -32(%rsp)
	setne	%dl
.L527:
	testl	%edx, %edx
	setne	%dl
	movzbl	%dl, %edx
	jmp	.L530
	.p2align 4,,10
	.p2align 3
.L523:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L531
	cmpb	$0, (%rdx)
	jne	.L518
	jmp	.L531
	.p2align 4,,10
	.p2align 3
.L525:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-32(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L527
	.p2align 4,,10
	.p2align 3
.L524:
	movl	-32(%rsp), %edx
	jmp	.L527
	.cfi_endproc
.LFE52:
	.size	aly_or, .-aly_or
	.p2align 4
	.globl	aly_xor
	.type	aly_xor, @function
aly_xor:
.LFB53:
	.cfi_startproc
	vmovdqu	8(%rsp), %xmm0
	movq	24(%rsp), %rdx
	movq	%rdi, %rax
	cmpl	$5, 8(%rsp)
	movq	%rdx, -24(%rsp)
	vmovdqa	%xmm0, -40(%rsp)
	ja	.L556
	movl	8(%rsp), %edx
	jmp	*.L558(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L558:
	.quad	.L563
	.quad	.L562
	.quad	.L561
	.quad	.L560
	.quad	.L559
	.quad	.L559
	.text
	.p2align 4,,10
	.p2align 3
.L559:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L556
	movl	8(%rdx), %esi
	xorl	%edx, %edx
	testl	%esi, %esi
	setg	%dl
.L564:
	vmovdqu	32(%rsp), %xmm0
	movq	48(%rsp), %rcx
	cmpl	$5, 32(%rsp)
	movq	%rcx, -24(%rsp)
	vmovdqa	%xmm0, -40(%rsp)
	ja	.L574
	movl	32(%rsp), %ecx
	jmp	*.L568(,%rcx,8)
	.section	.rodata
	.align 8
	.align 4
.L568:
	.quad	.L573
	.quad	.L572
	.quad	.L571
	.quad	.L570
	.quad	.L567
	.quad	.L567
	.text
	.p2align 4,,10
	.p2align 3
.L567:
	movq	-32(%rsp), %rcx
	testq	%rcx, %rcx
	je	.L574
	movl	8(%rcx), %ecx
	testl	%ecx, %ecx
	setg	%cl
	movzbl	%cl, %ecx
	xorl	%ecx, %edx
.L574:
	movl	$2, (%rax)
	movl	%edx, 8(%rax)
	movq	$0, 16(%rax)
	ret
	.p2align 4,,10
	.p2align 3
.L563:
	xorl	%edx, %edx
	cmpq	$0, -32(%rsp)
	setne	%dl
	jmp	.L564
	.p2align 4,,10
	.p2align 3
.L562:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-32(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L564
	.p2align 4,,10
	.p2align 3
.L561:
	movl	-32(%rsp), %edx
	jmp	.L564
	.p2align 4,,10
	.p2align 3
.L560:
	movq	-32(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L556
	cmpb	$0, (%rdx)
	setne	%dl
	movzbl	%dl, %edx
	jmp	.L564
	.p2align 4,,10
	.p2align 3
.L570:
	movq	-32(%rsp), %rcx
	testq	%rcx, %rcx
	je	.L574
	cmpb	$0, (%rcx)
	setne	%cl
	movzbl	%cl, %ecx
	xorl	%ecx, %edx
	jmp	.L574
	.p2align 4,,10
	.p2align 3
.L571:
	xorl	-32(%rsp), %edx
	jmp	.L574
	.p2align 4,,10
	.p2align 3
.L572:
	xorl	%ecx, %ecx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %esi
	vucomisd	-32(%rsp), %xmm0
	setp	%cl
	cmovne	%esi, %ecx
	xorl	%ecx, %edx
	jmp	.L574
	.p2align 4,,10
	.p2align 3
.L573:
	xorl	%ecx, %ecx
	cmpq	$0, -32(%rsp)
	setne	%cl
	xorl	%ecx, %edx
	jmp	.L574
	.p2align 4,,10
	.p2align 3
.L556:
	xorl	%edx, %edx
	jmp	.L564
	.cfi_endproc
.LFE53:
	.size	aly_xor, .-aly_xor
	.p2align 4
	.globl	aly_percent
	.type	aly_percent, @function
aly_percent:
.LFB54:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, %rcx
	movl	64(%rsp), %eax
	vmovsd	72(%rsp), %xmm0
	cmpl	$1, %eax
	je	.L599
	vmovdqu	64(%rsp), %xmm2
	cmpl	$2, %eax
	vxorps	%xmm1, %xmm1, %xmm1
	vmovdqa	%xmm2, 16(%rsp)
	je	.L594
	ja	.L600
	vmovq	%xmm0, %rax
	vcvtsi2sdq	%rax, %xmm1, %xmm1
	vdivsd	.LC14(%rip), %xmm1, %xmm1
	jmp	.L593
	.p2align 4,,10
	.p2align 3
.L599:
	vdivsd	.LC14(%rip), %xmm0, %xmm1
.L593:
	movl	$1, (%rcx)
	movq	%rcx, %rax
	movq	$0, 16(%rcx)
	vmovsd	%xmm1, 8(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L600:
	.cfi_restore_state
	cmpl	$3, %eax
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L593
	movq	%rdi, 8(%rsp)
	movl	$10, %edx
	xorl	%esi, %esi
	vmovq	%xmm0, %rdi
	call	strtoll
	vxorps	%xmm1, %xmm1, %xmm1
	movq	8(%rsp), %rcx
	vcvtsi2sdq	%rax, %xmm1, %xmm1
	vdivsd	.LC14(%rip), %xmm1, %xmm1
	jmp	.L593
	.p2align 4,,10
	.p2align 3
.L594:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	vcvtsi2sdq	%rax, %xmm1, %xmm1
	vdivsd	.LC14(%rip), %xmm1, %xmm1
	jmp	.L593
	.cfi_endproc
.LFE54:
	.size	aly_percent, .-aly_percent
	.p2align 4
	.globl	aly_compare_int
	.type	aly_compare_int, @function
aly_compare_int:
.LFB55:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, %rcx
	movq	80(%rsp), %rax
	vmovdqu	64(%rsp), %xmm0
	movq	%rax, 32(%rsp)
	movl	64(%rsp), %eax
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$2, %eax
	je	.L602
	ja	.L603
	testl	%eax, %eax
	movq	72(%rsp), %r8
	je	.L607
	vcvttsd2siq	24(%rsp), %r8
.L607:
	movl	88(%rsp), %edx
	vmovdqu	88(%rsp), %xmm0
	movq	104(%rsp), %rax
	cmpl	$2, %edx
	vmovdqa	%xmm0, 16(%rsp)
	movq	%rax, 32(%rsp)
	je	.L608
	ja	.L609
	testl	%edx, %edx
	movq	96(%rsp), %rax
	je	.L613
	vcvttsd2siq	24(%rsp), %rax
.L613:
	movzbl	(%rcx), %edx
	cmpl	$61, %edx
	jne	.L615
	cmpb	$61, 1(%rcx)
	jne	.L615
	cmpb	$0, 2(%rcx)
	jne	.L615
	cmpq	%rax, %r8
	sete	%al
	movzbl	%al, %eax
.L601:
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L603:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L607
	movq	%rdi, (%rsp)
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	call	strtoll
	movq	(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L607
	.p2align 4,,10
	.p2align 3
.L615:
	cmpl	$33, %edx
	jne	.L618
	cmpb	$61, 1(%rcx)
	je	.L632
.L618:
	movzbl	(%rcx), %esi
	cmpl	$60, %esi
	jne	.L620
	cmpb	$0, 1(%rcx)
	jne	.L620
	cmpq	%rax, %r8
	setl	%al
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L620:
	.cfi_restore_state
	cmpl	$60, %edx
	je	.L633
.L622:
	cmpl	$62, %esi
	jne	.L624
	cmpb	$0, 1(%rcx)
	jne	.L624
	cmpq	%rax, %r8
	setg	%al
	movzbl	%al, %eax
	jmp	.L601
	.p2align 4,,10
	.p2align 3
.L609:
	xorl	%eax, %eax
	cmpl	$3, %edx
	jne	.L613
	movq	24(%rsp), %rdi
	movl	$10, %edx
	xorl	%esi, %esi
	movq	%r8, (%rsp)
	movq	%rcx, 8(%rsp)
	call	strtoll
	movq	8(%rsp), %rcx
	movq	(%rsp), %r8
	jmp	.L613
	.p2align 4,,10
	.p2align 3
.L602:
	movl	24(%rsp), %esi
	xorl	%r8d, %r8d
	testl	%esi, %esi
	setne	%r8b
	jmp	.L607
	.p2align 4,,10
	.p2align 3
.L608:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	jmp	.L613
	.p2align 4,,10
	.p2align 3
.L632:
	cmpb	$0, 2(%rcx)
	jne	.L618
	cmpq	%rax, %r8
	setne	%al
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L633:
	.cfi_restore_state
	cmpb	$61, 1(%rcx)
	jne	.L622
	cmpb	$0, 2(%rcx)
	jne	.L622
	cmpq	%rax, %r8
	setle	%al
	movzbl	%al, %eax
	jmp	.L601
	.p2align 4,,10
	.p2align 3
.L624:
	cmpl	$62, %edx
	jne	.L626
	cmpb	$61, 1(%rcx)
	jne	.L626
	cmpb	$0, 2(%rcx)
	jne	.L626
	cmpq	%rax, %r8
	setge	%al
	movzbl	%al, %eax
	jmp	.L601
	.p2align 4,,10
	.p2align 3
.L626:
	xorl	%eax, %eax
	jmp	.L601
	.cfi_endproc
.LFE55:
	.size	aly_compare_int, .-aly_compare_int
	.p2align 4
	.globl	aly_compare
	.type	aly_compare, @function
aly_compare:
.LFB56:
	.cfi_startproc
	subq	$104, %rsp
	.cfi_def_cfa_offset 112
	movq	%rsi, %rdx
	movl	136(%rsp), %eax
	movl	112(%rsp), %ecx
	movq	%rbx, 88(%rsp)
	.cfi_offset 3, -24
	movq	%rdi, %rbx
	cmpl	$3, %eax
	je	.L671
	cmpl	$3, %ecx
	je	.L671
	cmpl	$1, %ecx
	vxorps	%xmm0, %xmm0, %xmm0
	je	.L650
	cmpl	$1, %eax
	je	.L677
	subq	$48, %rsp
	.cfi_def_cfa_offset 160
	movq	%rsi, %rdi
	movq	200(%rsp), %rax
	vmovdqu	184(%rsp), %xmm0
	movq	%rax, 40(%rsp)
	movq	176(%rsp), %rax
	vmovdqu	%xmm0, 24(%rsp)
	vmovdqu	160(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_compare_int
	addq	$48, %rsp
	.cfi_def_cfa_offset 112
	movl	$2, (%rbx)
	movl	%eax, 8(%rbx)
	movq	%rbx, %rax
	movq	$0, 16(%rbx)
	movq	88(%rsp), %rbx
	addq	$104, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L677:
	.cfi_restore_state
	testl	%ecx, %ecx
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L652
	vcvtsi2sdq	120(%rsp), %xmm0, %xmm0
	vmovapd	%xmm0, %xmm1
.L652:
	vmovsd	144(%rsp), %xmm0
	jmp	.L655
	.p2align 4,,10
	.p2align 3
.L671:
	movq	%rdx, 8(%rsp)
	leaq	16(%rsp), %rdi
	movq	%r14, 96(%rsp)
	.cfi_offset 14, -16
	subq	$32, %rsp
	.cfi_def_cfa_offset 144
	movq	160(%rsp), %rax
	vmovdqu	144(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	184(%rsp), %rax
	leaq	80(%rsp), %rdi
	vmovdqu	168(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	88(%rsp), %rsi
	movq	56(%rsp), %rdi
	addq	$32, %rsp
	.cfi_def_cfa_offset 112
	call	strcmp
	movq	8(%rsp), %rdx
	movl	%eax, %ecx
	movzbl	(%rdx), %eax
	cmpl	$61, %eax
	jne	.L638
	cmpb	$61, 1(%rdx)
	jne	.L638
	cmpb	$0, 2(%rdx)
	jne	.L638
	xorl	%r14d, %r14d
	testl	%ecx, %ecx
	sete	%r14b
.L639:
	subq	$32, %rsp
	.cfi_def_cfa_offset 144
	movq	64(%rsp), %rax
	vmovdqa	48(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	movq	96(%rsp), %rax
	vmovdqa	80(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 112
	movl	%r14d, 8(%rbx)
	movq	%rbx, %rax
	movl	$2, (%rbx)
	movq	96(%rsp), %r14
	.cfi_restore 14
	movq	$0, 16(%rbx)
	movq	88(%rsp), %rbx
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L650:
	.cfi_def_cfa_offset 112
	cmpl	$3, %eax
	vmovsd	120(%rsp), %xmm1
	ja	.L653
	testl	%eax, %eax
	jne	.L678
	vcvtsi2sdq	144(%rsp), %xmm0, %xmm0
.L655:
	movzbl	(%rdx), %eax
	cmpl	$61, %eax
	jne	.L657
	cmpb	$61, 1(%rdx)
	jne	.L657
	movzbl	2(%rdx), %ecx
	testl	%ecx, %ecx
	jne	.L657
	xorl	%eax, %eax
	vucomisd	%xmm1, %xmm0
	setnp	%al
	cmovne	%ecx, %eax
.L658:
	movl	%eax, 8(%rbx)
	movq	%rbx, %rax
	movl	$2, (%rbx)
	movq	$0, 16(%rbx)
	movq	88(%rsp), %rbx
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L638:
	.cfi_def_cfa_offset 112
	.cfi_offset 14, -16
	cmpl	$33, %eax
	jne	.L641
	cmpb	$61, 1(%rdx)
	jne	.L641
	cmpb	$0, 2(%rdx)
	jne	.L641
	xorl	%r14d, %r14d
	testl	%ecx, %ecx
	setne	%r14b
	jmp	.L639
	.p2align 4,,10
	.p2align 3
.L678:
	.cfi_restore 14
	cmpl	$1, %eax
	je	.L652
.L653:
	vxorpd	%xmm0, %xmm0, %xmm0
	jmp	.L655
	.p2align 4,,10
	.p2align 3
.L641:
	.cfi_offset 14, -16
	movzbl	(%rdx), %esi
	cmpl	$60, %esi
	je	.L679
.L643:
	cmpl	$60, %eax
	jne	.L645
	cmpb	$61, 1(%rdx)
	jne	.L645
	cmpb	$0, 2(%rdx)
	jne	.L645
	xorl	%r14d, %r14d
	testl	%ecx, %ecx
	setle	%r14b
	jmp	.L639
	.p2align 4,,10
	.p2align 3
.L657:
	.cfi_restore 14
	cmpl	$33, %eax
	jne	.L660
	cmpb	$61, 1(%rdx)
	jne	.L660
	cmpb	$0, 2(%rdx)
	jne	.L660
	xorl	%eax, %eax
	vucomisd	%xmm1, %xmm0
	movl	$1, %edx
	setp	%al
	cmovne	%edx, %eax
	jmp	.L658
	.p2align 4,,10
	.p2align 3
.L660:
	movzbl	(%rdx), %esi
	cmpl	$60, %esi
	je	.L680
.L662:
	cmpl	$60, %eax
	jne	.L664
	cmpb	$61, 1(%rdx)
	jne	.L664
	cmpb	$0, 2(%rdx)
	jne	.L664
	xorl	%eax, %eax
	vcomisd	%xmm1, %xmm0
	setnb	%al
	jmp	.L658
	.p2align 4,,10
	.p2align 3
.L679:
	.cfi_offset 14, -16
	cmpb	$0, 1(%rdx)
	jne	.L643
.L676:
	shrl	$31, %ecx
	movl	%ecx, %r14d
	jmp	.L639
	.p2align 4,,10
	.p2align 3
.L680:
	.cfi_restore 14
	cmpb	$0, 1(%rdx)
	jne	.L662
	xorl	%eax, %eax
	vcomisd	%xmm1, %xmm0
	seta	%al
	jmp	.L658
	.p2align 4,,10
	.p2align 3
.L645:
	.cfi_offset 14, -16
	cmpl	$62, %esi
	jne	.L647
	cmpb	$0, 1(%rdx)
	jne	.L647
	xorl	%r14d, %r14d
	testl	%ecx, %ecx
	setg	%r14b
	jmp	.L639
	.p2align 4,,10
	.p2align 3
.L664:
	.cfi_restore 14
	cmpl	$62, %esi
	jne	.L666
	cmpb	$0, 1(%rdx)
	jne	.L666
	xorl	%eax, %eax
	vcomisd	%xmm0, %xmm1
	seta	%al
	jmp	.L658
.L647:
	.cfi_offset 14, -16
	cmpl	$62, %eax
	jne	.L668
	cmpb	$61, 1(%rdx)
	jne	.L668
	cmpb	$0, 2(%rdx)
	jne	.L668
	notl	%ecx
	jmp	.L676
.L666:
	.cfi_restore 14
	cmpl	$62, %eax
	jne	.L670
	cmpb	$61, 1(%rdx)
	jne	.L670
	cmpb	$0, 2(%rdx)
	jne	.L670
	xorl	%eax, %eax
	vcomisd	%xmm0, %xmm1
	setnb	%al
	jmp	.L658
.L668:
	.cfi_offset 14, -16
	xorl	%r14d, %r14d
	jmp	.L639
.L670:
	.cfi_restore 14
	xorl	%eax, %eax
	jmp	.L658
	.cfi_endproc
.LFE56:
	.size	aly_compare, .-aly_compare
	.section	.rodata.str1.1
.LC15:
	.string	"=="
	.text
	.p2align 4
	.globl	aly_eq
	.type	aly_eq, @function
aly_eq:
.LFB57:
	.cfi_startproc
	subq	$152, %rsp
	.cfi_def_cfa_offset 160
	movq	200(%rsp), %rax
	movl	160(%rsp), %edx
	vmovdqu	160(%rsp), %xmm0
	movq	176(%rsp), %rcx
	vmovdqu	184(%rsp), %xmm1
	movq	%rax, 64(%rsp)
	movl	184(%rsp), %eax
	movq	%rcx, 32(%rsp)
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$3, %eax
	vmovdqa	%xmm1, 48(%rsp)
	je	.L692
	cmpl	$3, %edx
	je	.L692
	cmpl	$1, %edx
	vxorps	%xmm1, %xmm1, %xmm1
	je	.L685
	cmpl	$1, %eax
	je	.L697
	movq	%rcx, 176(%rsp)
	movl	$.LC15, %edi
	vmovdqu	%xmm0, 160(%rsp)
	addq	$152, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	jmp	aly_compare_int
	.p2align 4,,10
	.p2align 3
.L697:
	.cfi_restore_state
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L687
	vcvtsi2sdq	24(%rsp), %xmm1, %xmm1
	vmovapd	%xmm1, %xmm0
.L687:
	vmovsd	56(%rsp), %xmm1
	jmp	.L690
	.p2align 4,,10
	.p2align 3
.L692:
	leaq	80(%rsp), %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 192
	movq	64(%rsp), %rax
	vmovdqa	48(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	96(%rsp), %rax
	vmovdqa	80(%rsp), %xmm0
	leaq	144(%rsp), %rdi
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	120(%rsp), %rdi
	addq	$32, %rsp
	.cfi_def_cfa_offset 160
	movq	120(%rsp), %rsi
	call	strcmp
	testl	%eax, %eax
	sete	%al
	subq	$32, %rsp
	.cfi_def_cfa_offset 192
	movzbl	%al, %eax
	movl	%eax, 44(%rsp)
	movq	128(%rsp), %rdx
	vmovdqa	112(%rsp), %xmm0
	movq	%rdx, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	movq	160(%rsp), %rdx
	vmovdqa	144(%rsp), %xmm0
	movq	%rdx, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 160
	movl	12(%rsp), %eax
	addq	$152, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L685:
	.cfi_restore_state
	cmpl	$3, %eax
	vmovsd	24(%rsp), %xmm0
	ja	.L688
	testl	%eax, %eax
	jne	.L698
	vcvtsi2sdq	56(%rsp), %xmm1, %xmm1
.L690:
	xorl	%eax, %eax
	vucomisd	%xmm1, %xmm0
	movl	$0, %edx
	setnp	%al
	cmovne	%edx, %eax
	addq	$152, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L698:
	.cfi_restore_state
	cmpl	$1, %eax
	je	.L687
.L688:
	vxorpd	%xmm1, %xmm1, %xmm1
	jmp	.L690
	.cfi_endproc
.LFE57:
	.size	aly_eq, .-aly_eq
	.section	.rodata.str1.1
.LC16:
	.string	">="
	.text
	.p2align 4
	.globl	aly_gte
	.type	aly_gte, @function
aly_gte:
.LFB58:
	.cfi_startproc
	subq	$152, %rsp
	.cfi_def_cfa_offset 160
	movq	200(%rsp), %rax
	movl	160(%rsp), %edx
	vmovdqu	160(%rsp), %xmm0
	movq	176(%rsp), %rcx
	vmovdqu	184(%rsp), %xmm1
	movq	%rax, 64(%rsp)
	movl	184(%rsp), %eax
	movq	%rcx, 32(%rsp)
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$3, %eax
	vmovdqa	%xmm1, 48(%rsp)
	je	.L710
	cmpl	$3, %edx
	je	.L710
	cmpl	$1, %edx
	vxorps	%xmm1, %xmm1, %xmm1
	je	.L703
	cmpl	$1, %eax
	je	.L715
	movq	%rcx, 176(%rsp)
	movl	$.LC16, %edi
	vmovdqu	%xmm0, 160(%rsp)
	addq	$152, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	jmp	aly_compare_int
	.p2align 4,,10
	.p2align 3
.L715:
	.cfi_restore_state
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L705
	vcvtsi2sdq	24(%rsp), %xmm1, %xmm1
	vmovapd	%xmm1, %xmm0
.L705:
	vmovsd	56(%rsp), %xmm1
	jmp	.L708
	.p2align 4,,10
	.p2align 3
.L710:
	leaq	80(%rsp), %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 192
	movq	64(%rsp), %rax
	vmovdqa	48(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	96(%rsp), %rax
	vmovdqa	80(%rsp), %xmm0
	leaq	144(%rsp), %rdi
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	120(%rsp), %rdi
	addq	$32, %rsp
	.cfi_def_cfa_offset 160
	movq	120(%rsp), %rsi
	call	strcmp
	subq	$32, %rsp
	.cfi_def_cfa_offset 192
	notl	%eax
	shrl	$31, %eax
	movl	%eax, 44(%rsp)
	movq	128(%rsp), %rdx
	vmovdqa	112(%rsp), %xmm0
	movq	%rdx, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	movq	160(%rsp), %rdx
	vmovdqa	144(%rsp), %xmm0
	movq	%rdx, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 160
	movl	12(%rsp), %eax
	addq	$152, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L703:
	.cfi_restore_state
	cmpl	$3, %eax
	vmovsd	24(%rsp), %xmm0
	ja	.L706
	testl	%eax, %eax
	jne	.L716
	vcvtsi2sdq	56(%rsp), %xmm1, %xmm1
.L708:
	xorl	%eax, %eax
	vcomisd	%xmm1, %xmm0
	setnb	%al
	addq	$152, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L716:
	.cfi_restore_state
	cmpl	$1, %eax
	je	.L705
.L706:
	vxorpd	%xmm1, %xmm1, %xmm1
	jmp	.L708
	.cfi_endproc
.LFE58:
	.size	aly_gte, .-aly_gte
	.section	.rodata.str1.1
.LC17:
	.string	"<="
	.text
	.p2align 4
	.globl	aly_lte
	.type	aly_lte, @function
aly_lte:
.LFB59:
	.cfi_startproc
	subq	$152, %rsp
	.cfi_def_cfa_offset 160
	movq	200(%rsp), %rax
	movl	160(%rsp), %edx
	vmovdqu	160(%rsp), %xmm0
	movq	176(%rsp), %rcx
	vmovdqu	184(%rsp), %xmm1
	movq	%rax, 64(%rsp)
	movl	184(%rsp), %eax
	movq	%rcx, 32(%rsp)
	vmovdqa	%xmm0, 16(%rsp)
	cmpl	$3, %eax
	vmovdqa	%xmm1, 48(%rsp)
	je	.L728
	cmpl	$3, %edx
	je	.L728
	cmpl	$1, %edx
	vxorps	%xmm1, %xmm1, %xmm1
	je	.L721
	cmpl	$1, %eax
	je	.L733
	movq	%rcx, 176(%rsp)
	movl	$.LC17, %edi
	vmovdqu	%xmm0, 160(%rsp)
	addq	$152, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	jmp	aly_compare_int
	.p2align 4,,10
	.p2align 3
.L733:
	.cfi_restore_state
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L723
	vcvtsi2sdq	24(%rsp), %xmm1, %xmm1
	vmovapd	%xmm1, %xmm0
.L723:
	vmovsd	56(%rsp), %xmm1
	jmp	.L726
	.p2align 4,,10
	.p2align 3
.L728:
	leaq	80(%rsp), %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 192
	movq	64(%rsp), %rax
	vmovdqa	48(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	96(%rsp), %rax
	vmovdqa	80(%rsp), %xmm0
	leaq	144(%rsp), %rdi
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	120(%rsp), %rdi
	addq	$32, %rsp
	.cfi_def_cfa_offset 160
	movq	120(%rsp), %rsi
	call	strcmp
	testl	%eax, %eax
	setle	%al
	subq	$32, %rsp
	.cfi_def_cfa_offset 192
	movzbl	%al, %eax
	movl	%eax, 44(%rsp)
	movq	128(%rsp), %rdx
	vmovdqa	112(%rsp), %xmm0
	movq	%rdx, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	movq	160(%rsp), %rdx
	vmovdqa	144(%rsp), %xmm0
	movq	%rdx, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 160
	movl	12(%rsp), %eax
	addq	$152, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L721:
	.cfi_restore_state
	cmpl	$3, %eax
	vmovsd	24(%rsp), %xmm0
	ja	.L724
	testl	%eax, %eax
	jne	.L734
	vcvtsi2sdq	56(%rsp), %xmm1, %xmm1
.L726:
	xorl	%eax, %eax
	vcomisd	%xmm0, %xmm1
	setnb	%al
	addq	$152, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L734:
	.cfi_restore_state
	cmpl	$1, %eax
	je	.L723
.L724:
	vxorpd	%xmm1, %xmm1, %xmm1
	jmp	.L726
	.cfi_endproc
.LFE59:
	.size	aly_lte, .-aly_lte
	.p2align 4
	.globl	aly_print
	.type	aly_print, @function
aly_print:
.LFB60:
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	cmpl	$3, 80(%rsp)
	je	.L756
	movq	%rbx, 56(%rsp)
	leaq	16(%rsp), %rdi
	movq	%rbp, 64(%rsp)
	.cfi_offset 3, -24
	.cfi_offset 6, -16
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	128(%rsp), %rax
	vmovdqu	112(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	56(%rsp), %rbx
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	movq	%rbx, %rdi
	call	puts
	movl	16(%rsp), %eax
	movq	32(%rsp), %rbp
	cmpl	$4, %eax
	je	.L737
	cmpl	$5, %eax
	je	.L738
	cmpl	$3, %eax
	je	.L755
.L740:
	testq	%rbp, %rbp
	je	.L753
.L758:
	movq	56(%rsp), %rbx
	.cfi_remember_state
	.cfi_restore 3
	movq	%rbp, %rdi
	movq	64(%rsp), %rbp
	.cfi_restore 6
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	jmp	free
	.p2align 4,,10
	.p2align 3
.L738:
	.cfi_restore_state
	movl	12(%rbx), %esi
	xorl	%eax, %eax
	movq	(%rbx), %rdi
	testl	%esi, %esi
	jle	.L742
.L741:
	leaq	(%rax,%rax,4), %rdx
	movl	32(%rdi,%rdx,8), %ecx
	testl	%ecx, %ecx
	jne	.L757
	addq	$1, %rax
	cmpl	%eax, %esi
	jg	.L741
.L742:
	call	free
.L755:
	movq	%rbx, %rdi
	call	free
	testq	%rbp, %rbp
	jne	.L758
.L753:
	movq	56(%rsp), %rbx
	.cfi_restore 3
	movq	64(%rsp), %rbp
	.cfi_restore 6
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L756:
	.cfi_def_cfa_offset 80
	movq	88(%rsp), %rdi
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	jmp	puts
	.p2align 4,,10
	.p2align 3
.L737:
	.cfi_def_cfa_offset 80
	.cfi_offset 3, -24
	.cfi_offset 6, -16
	movl	8(%rbx), %edi
	xorl	%edx, %edx
	testl	%edi, %edi
	jle	.L744
.L743:
	movq	(%rbx), %rsi
	leaq	(%rdx,%rdx,2), %rax
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	%rdx, 32(%rsp)
	leaq	(%rsi,%rax,8), %rax
	vmovdqu	(%rax), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%rax), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	movq	32(%rsp), %rdx
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	addq	$1, %rdx
	cmpl	%edx, 8(%rbx)
	jg	.L743
.L744:
	movq	(%rbx), %rdi
	call	free
	movq	%rbx, %rdi
	call	free
	jmp	.L740
	.p2align 4,,10
	.p2align 3
.L757:
	movq	(%rdi,%rdx,8), %rdi
	movq	%rdx, 8(%rsp)
	movq	%rax, (%rsp)
	call	free
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	(%rbx), %rcx
	movq	40(%rsp), %rdx
	vmovdqu	8(%rcx,%rdx,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	24(%rcx,%rdx,8), %rdx
	movq	%rdx, 16(%rsp)
	call	aly_free
	movq	32(%rsp), %rax
	movl	12(%rbx), %esi
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	movq	(%rbx), %rdi
	addq	$1, %rax
	cmpl	%eax, %esi
	jg	.L741
	jmp	.L742
	.cfi_endproc
.LFE60:
	.size	aly_print, .-aly_print
	.section	.rodata.str1.1
.LC18:
	.string	"%s"
	.text
	.p2align 4
	.globl	aly_print_raw
	.type	aly_print_raw, @function
aly_print_raw:
.LFB61:
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	cmpl	$3, 80(%rsp)
	je	.L781
	movq	%rbx, 56(%rsp)
	leaq	16(%rsp), %rdi
	movq	%rbp, 64(%rsp)
	.cfi_offset 3, -24
	.cfi_offset 6, -16
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	128(%rsp), %rax
	vmovdqu	112(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	56(%rsp), %rbx
	xorl	%eax, %eax
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	movl	$.LC18, %edi
	movq	%rbx, %rsi
	call	printf
	movl	16(%rsp), %eax
	movq	32(%rsp), %rbp
	cmpl	$4, %eax
	je	.L761
	cmpl	$5, %eax
	je	.L762
	cmpl	$3, %eax
	je	.L780
.L763:
	testq	%rbp, %rbp
	je	.L778
.L783:
	movq	56(%rsp), %rbx
	.cfi_remember_state
	.cfi_restore 3
	movq	%rbp, %rdi
	movq	64(%rsp), %rbp
	.cfi_restore 6
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	jmp	free
	.p2align 4,,10
	.p2align 3
.L762:
	.cfi_restore_state
	testq	%rbx, %rbx
	je	.L763
	movl	12(%rbx), %esi
	movq	(%rbx), %rdi
	testl	%esi, %esi
	jle	.L766
	xorl	%eax, %eax
.L769:
	leaq	(%rax,%rax,4), %rdx
	movl	32(%rdi,%rdx,8), %ecx
	testl	%ecx, %ecx
	jne	.L782
	addq	$1, %rax
	cmpl	%eax, %esi
	jg	.L769
.L766:
	call	free
.L780:
	movq	%rbx, %rdi
	call	free
	testq	%rbp, %rbp
	jne	.L783
.L778:
	movq	56(%rsp), %rbx
	.cfi_restore 3
	movq	64(%rsp), %rbp
	.cfi_restore 6
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L781:
	.cfi_def_cfa_offset 80
	movq	88(%rsp), %rsi
	movl	$.LC18, %edi
	xorl	%eax, %eax
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	jmp	printf
	.p2align 4,,10
	.p2align 3
.L761:
	.cfi_def_cfa_offset 80
	.cfi_offset 3, -24
	.cfi_offset 6, -16
	testq	%rbx, %rbx
	je	.L763
	movl	8(%rbx), %edi
	testl	%edi, %edi
	jle	.L764
	xorl	%edx, %edx
.L765:
	movq	(%rbx), %rsi
	leaq	(%rdx,%rdx,2), %rax
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	%rdx, 32(%rsp)
	leaq	(%rsi,%rax,8), %rax
	vmovdqu	(%rax), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%rax), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	movq	32(%rsp), %rdx
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	addq	$1, %rdx
	cmpl	%edx, 8(%rbx)
	jg	.L765
.L764:
	movq	(%rbx), %rdi
	call	free
	movq	%rbx, %rdi
	call	free
	jmp	.L763
	.p2align 4,,10
	.p2align 3
.L782:
	movq	(%rdi,%rdx,8), %rdi
	movq	%rdx, 8(%rsp)
	movq	%rax, (%rsp)
	call	free
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	(%rbx), %rcx
	movq	40(%rsp), %rdx
	vmovdqu	8(%rcx,%rdx,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	24(%rcx,%rdx,8), %rdx
	movq	%rdx, 16(%rsp)
	call	aly_free
	movq	32(%rsp), %rax
	movl	12(%rbx), %esi
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	movq	(%rbx), %rdi
	addq	$1, %rax
	cmpl	%eax, %esi
	jg	.L769
	jmp	.L766
	.cfi_endproc
.LFE61:
	.size	aly_print_raw, .-aly_print_raw
	.section	.rodata.str1.1
.LC19:
	.string	"\n"
.LC20:
	.string	""
	.text
	.p2align 4
	.globl	aly_input
	.type	aly_input, @function
aly_input:
.LFB62:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movq	%rdi, %rbx
	subq	$4096, %rsp
	.cfi_def_cfa_offset 4112
	testq	%rsi, %rsi
	je	.L785
	movl	$.LC18, %edi
	xorl	%eax, %eax
	call	printf
.L785:
	movq	stdin(%rip), %rdx
	movl	$4096, %esi
	movq	%rsp, %rdi
	call	fgets
	testq	%rax, %rax
	je	.L786
	movq	%rsp, %rdi
	movl	$.LC19, %esi
	call	strcspn
	movq	%rsp, %rdi
	movb	$0, (%rsp,%rax)
	call	strdup
.L787:
	movq	%rax, 8(%rbx)
	movq	%rbx, %rax
	movl	$3, (%rbx)
	movq	$0, 16(%rbx)
	addq	$4096, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 16
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L786:
	.cfi_restore_state
	movl	$.LC20, %edi
	call	strdup
	jmp	.L787
	.cfi_endproc
.LFE62:
	.size	aly_input, .-aly_input
	.p2align 4
	.globl	native_input
	.type	native_input, @function
native_input:
.LFB107:
	.cfi_startproc
	testl	%esi, %esi
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movq	%rdi, %rbx
	jg	.L796
	movl	$.LC20, %esi
	call	aly_input
	movq	%rbx, %rax
	popq	%rbx
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L796:
	.cfi_restore_state
	movq	8(%rdx), %rsi
	call	aly_input
	movq	%rbx, %rax
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE107:
	.size	native_input, .-native_input
	.p2align 4
	.globl	aly_tomb
	.type	aly_tomb, @function
aly_tomb:
.LFB63:
	.cfi_startproc
	vmovdqu	(%rsi), %xmm0
	movq	16(%rsi), %rdx
	movq	%rdi, %rax
	movq	%rdx, 16(%rdi)
	vmovdqu	%xmm0, (%rdi)
	ret
	.cfi_endproc
.LFE63:
	.size	aly_tomb, .-aly_tomb
	.p2align 4
	.globl	aly_array_new
	.type	aly_array_new, @function
aly_array_new:
.LFB64:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movl	$16, %esi
	pushq	%rbx
	.cfi_def_cfa_offset 24
	.cfi_offset 3, -24
	movq	%rdi, %rbx
	movl	$1, %edi
	subq	$8, %rsp
	.cfi_def_cfa_offset 32
	call	calloc
	testq	%rbx, %rbx
	movq	%rax, %rbp
	jle	.L800
	movl	%ebx, %eax
	movslq	%ebx, %rdi
.L799:
	movl	%eax, 12(%rbp)
	movl	$24, %esi
	call	calloc
	movl	$0, 8(%rbp)
	movq	%rax, 0(%rbp)
	addq	$8, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 24
	movq	%rbp, %rax
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L800:
	.cfi_restore_state
	movl	$8, %edi
	movl	$8, %eax
	jmp	.L799
	.cfi_endproc
.LFE64:
	.size	aly_array_new, .-aly_array_new
	.p2align 4
	.globl	aly_array_init
	.type	aly_array_init, @function
aly_array_init:
.LFB65:
	.cfi_startproc
	subq	$136, %rsp
	.cfi_def_cfa_offset 144
	movq	%r13, 112(%rsp)
	.cfi_offset 13, -32
	movl	%esi, %r13d
	movl	$16, %esi
	movq	%rdi, 8(%rsp)
	movl	$1, %edi
	movq	%rbx, 88(%rsp)
	.cfi_offset 3, -56
	movq	%rdx, %rbx
	movq	%r15, 128(%rsp)
	.cfi_offset 15, -16
	call	calloc
	testl	%r13d, %r13d
	movq	%rax, %r15
	jle	.L816
	movq	%rbp, 96(%rsp)
	movl	%r13d, %edi
	movl	$24, %esi
	movq	%r12, 104(%rsp)
	.cfi_offset 6, -48
	.cfi_offset 12, -40
	movl	$1, %r12d
	movq	%r14, 120(%rsp)
	.cfi_offset 14, -24
	movl	%r13d, 12(%rax)
	call	calloc
	movl	$0, 8(%r15)
	movq	%rax, (%r15)
	movq	%rax, %rbp
	.p2align 4,,10
	.p2align 3
.L812:
	movl	(%rbx), %r14d
	vmovdqu	(%rbx), %xmm0
	movq	16(%rbx), %rax
	cmpl	$2, %r14d
	vmovdqa	%xmm0, 16(%rsp)
	je	.L805
	ja	.L806
	testl	%r14d, %r14d
	je	.L817
	vmovsd	24(%rsp), %xmm0
	vmovsd	%xmm0, 56(%rsp)
.L810:
	xorl	%eax, %eax
.L811:
	movl	%r14d, 48(%rsp)
	vmovdqa	48(%rsp), %xmm0
	addq	$24, %rbx
	addq	$24, %rbp
	movq	%rax, -8(%rbp)
	vmovdqu	%xmm0, -24(%rbp)
	cmpl	%r12d, %r13d
	movl	%r12d, 8(%r15)
	je	.L818
	addl	$1, %r12d
	jmp	.L812
	.p2align 4,,10
	.p2align 3
.L806:
	cmpl	$3, %r14d
	jne	.L809
	movq	24(%rsp), %rdi
	call	strdup
	movq	%rax, 56(%rsp)
	jmp	.L810
	.p2align 4,,10
	.p2align 3
.L817:
	movq	24(%rsp), %rax
	movq	%rax, 56(%rsp)
	jmp	.L810
	.p2align 4,,10
	.p2align 3
.L809:
	vmovdqa	%xmm0, 48(%rsp)
	jmp	.L811
	.p2align 4,,10
	.p2align 3
.L805:
	movl	24(%rsp), %eax
	movl	%eax, 56(%rsp)
	jmp	.L810
	.p2align 4,,10
	.p2align 3
.L818:
	movq	96(%rsp), %rbp
	.cfi_restore 6
	movq	104(%rsp), %r12
	.cfi_restore 12
	movq	120(%rsp), %r14
	.cfi_restore 14
.L804:
	movq	8(%rsp), %rax
	movq	88(%rsp), %rbx
	movq	112(%rsp), %r13
	movq	%r15, 8(%rax)
	movq	128(%rsp), %r15
	movl	$4, (%rax)
	movq	$0, 16(%rax)
	addq	$136, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L816:
	.cfi_restore_state
	movl	$24, %esi
	movl	$8, %edi
	call	calloc
	movq	%rax, (%r15)
	movabsq	$34359738368, %rax
	movq	%rax, 8(%r15)
	jmp	.L804
	.cfi_endproc
.LFE65:
	.size	aly_array_init, .-aly_array_init
	.p2align 4
	.globl	aly_array_push
	.type	aly_array_push, @function
aly_array_push:
.LFB73:
	.cfi_startproc
	subq	$88, %rsp
	.cfi_def_cfa_offset 96
	cmpl	$4, 96(%rsp)
	movq	104(%rsp), %rdx
	jne	.L830
	testq	%rdx, %rdx
	je	.L830
	movslq	8(%rdx), %rcx
	movl	12(%rdx), %esi
	movq	(%rdx), %rdi
	cmpl	%esi, %ecx
	jge	.L832
.L821:
	leal	1(%rcx), %eax
	vmovdqu	120(%rsp), %xmm0
	movl	%eax, 8(%rdx)
	movl	120(%rsp), %edx
	leaq	(%rcx,%rcx,2), %rax
	leaq	(%rdi,%rax,8), %rcx
	vmovdqa	%xmm0, 16(%rsp)
	movq	136(%rsp), %rax
	cmpl	$2, %edx
	je	.L822
	ja	.L823
	testl	%edx, %edx
	je	.L833
	vmovsd	24(%rsp), %xmm0
	vmovsd	%xmm0, 56(%rsp)
.L827:
	xorl	%eax, %eax
.L828:
	movl	%edx, 48(%rsp)
	vmovdqa	48(%rsp), %xmm0
	movq	%rax, 16(%rcx)
	vmovdqu	%xmm0, (%rcx)
.L830:
	addq	$88, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L833:
	.cfi_restore_state
	movq	24(%rsp), %rax
	movq	%rax, 56(%rsp)
	jmp	.L827
	.p2align 4,,10
	.p2align 3
.L832:
	addl	%esi, %esi
	movq	%rdx, (%rsp)
	movl	%esi, 12(%rdx)
	movslq	%esi, %rsi
	leaq	(%rsi,%rsi,2), %rsi
	salq	$3, %rsi
	call	realloc
	movq	(%rsp), %rdx
	movq	%rax, %rdi
	movq	%rax, (%rdx)
	movslq	8(%rdx), %rcx
	jmp	.L821
	.p2align 4,,10
	.p2align 3
.L823:
	cmpl	$3, %edx
	jne	.L826
	movq	24(%rsp), %rdi
	movl	%edx, 12(%rsp)
	movq	%rcx, (%rsp)
	call	strdup
	movq	(%rsp), %rcx
	movl	12(%rsp), %edx
	movq	%rax, 56(%rsp)
	jmp	.L827
	.p2align 4,,10
	.p2align 3
.L822:
	movl	24(%rsp), %eax
	movl	%eax, 56(%rsp)
	jmp	.L827
	.p2align 4,,10
	.p2align 3
.L826:
	vmovdqa	%xmm0, 48(%rsp)
	jmp	.L828
	.cfi_endproc
.LFE73:
	.size	aly_array_push, .-aly_array_push
	.p2align 4
	.globl	aly_object_new
	.type	aly_object_new, @function
aly_object_new:
.LFB74:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movl	$16, %esi
	movl	$1, %edi
	call	calloc
	movl	$40, %esi
	movl	$32, %edi
	movq	%rax, %rbx
	call	calloc
	movq	%rax, (%rbx)
	movabsq	$137438953472, %rax
	movq	%rax, 8(%rbx)
	movq	%rbx, %rax
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE74:
	.size	aly_object_new, .-aly_object_new
	.p2align 4
	.globl	aly_object_set
	.type	aly_object_set, @function
aly_object_set:
.LFB76:
	.cfi_startproc
	subq	$136, %rsp
	.cfi_def_cfa_offset 144
	movq	152(%rsp), %rax
	cmpl	$5, 144(%rsp)
	movq	%rax, 8(%rsp)
	jne	.L862
	testq	%rax, %rax
	je	.L862
	movq	%r15, 128(%rsp)
	movzbl	(%rdi), %edx
	testb	%dl, %dl
	.cfi_offset 15, -16
	je	.L857
	movq	%rdi, %rcx
	movl	$5381, %eax
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L839:
	movl	%eax, %esi
	addq	$1, %rcx
	sall	$5, %esi
	addl	%esi, %eax
	addl	%edx, %eax
	movzbl	(%rcx), %edx
	testb	%dl, %dl
	jne	.L839
.L838:
	movq	8(%rsp), %rcx
	movl	12(%rcx), %r15d
	testl	%r15d, %r15d
	jle	.L864
	xorl	%edx, %edx
	movq	%rbp, 96(%rsp)
	divl	%r15d
	movq	%r14, 120(%rsp)
	movq	%rdi, (%rsp)
	movq	%r12, 104(%rsp)
	.cfi_offset 6, -48
	.cfi_offset 14, -24
	.cfi_offset 12, -40
	movq	(%rcx), %r12
	movq	%rbx, 88(%rsp)
	movq	%r13, 112(%rsp)
	.cfi_offset 3, -56
	.cfi_offset 13, -32
	movl	%edx, %ebx
	leal	(%r15,%rdx), %r13d
	.p2align 4,,10
	.p2align 3
.L856:
	xorl	%edx, %edx
	movl	%ebx, %eax
	divl	%r15d
	leaq	(%rdx,%rdx,4), %rbp
	salq	$3, %rbp
	leaq	(%r12,%rbp), %r14
	movl	32(%r14), %eax
	testl	%eax, %eax
	jne	.L866
	movq	(%rsp), %rdi
	call	strdup
	movl	168(%rsp), %ebx
	vmovdqu	168(%rsp), %xmm0
	movq	%rax, (%r14)
	movq	184(%rsp), %rax
	cmpl	$2, %ebx
	vmovdqa	%xmm0, 16(%rsp)
	movq	%rax, 32(%rsp)
	je	.L849
	ja	.L850
	testl	%ebx, %ebx
	je	.L867
	vmovsd	24(%rsp), %xmm0
	vmovsd	%xmm0, 56(%rsp)
.L854:
	xorl	%eax, %eax
.L855:
	movq	%rax, 24(%r14)
	movl	%ebx, 48(%rsp)
	vmovdqa	48(%rsp), %xmm0
	movq	%rax, 64(%rsp)
	movq	8(%rsp), %rax
	movl	$1, 32(%r14)
	vmovdqu	%xmm0, 8(%r14)
	addl	$1, 8(%rax)
.L865:
	movq	88(%rsp), %rbx
	.cfi_restore 3
	movq	96(%rsp), %rbp
	.cfi_restore 6
	movq	104(%rsp), %r12
	.cfi_restore 12
	movq	112(%rsp), %r13
	.cfi_restore 13
	movq	120(%rsp), %r14
	.cfi_restore 14
	movq	128(%rsp), %r15
	.cfi_restore 15
.L862:
	addq	$136, %rsp
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L866:
	.cfi_def_cfa_offset 144
	.cfi_offset 3, -56
	.cfi_offset 6, -48
	.cfi_offset 12, -40
	.cfi_offset 13, -32
	.cfi_offset 14, -24
	.cfi_offset 15, -16
	movq	(%r14), %rdi
	movq	(%rsp), %rsi
	call	strcmp
	testl	%eax, %eax
	je	.L868
	addl	$1, %ebx
	cmpl	%r13d, %ebx
	jne	.L856
	jmp	.L865
	.p2align 4,,10
	.p2align 3
.L868:
	vmovdqu	8(%r14), %xmm0
	subq	$32, %rsp
	.cfi_def_cfa_offset 176
	vmovdqu	%xmm0, (%rsp)
	movq	24(%r14), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	movq	40(%rsp), %rax
	vmovdqu	200(%rsp), %xmm0
	movl	200(%rsp), %ebx
	addq	(%rax), %rbp
	movq	216(%rsp), %rax
	vmovdqa	%xmm0, 48(%rsp)
	movq	%rax, 64(%rsp)
	addq	$32, %rsp
	.cfi_def_cfa_offset 144
	cmpl	$2, %ebx
	je	.L842
	ja	.L843
	testl	%ebx, %ebx
	je	.L869
	vmovsd	24(%rsp), %xmm0
	vmovsd	%xmm0, 56(%rsp)
.L847:
	xorl	%eax, %eax
.L848:
	movl	%ebx, 48(%rsp)
	vmovdqa	48(%rsp), %xmm0
	movq	%rax, 24(%rbp)
	vmovdqu	%xmm0, 8(%rbp)
	jmp	.L865
.L849:
	movl	24(%rsp), %eax
	movl	%eax, 56(%rsp)
	jmp	.L854
.L867:
	movq	24(%rsp), %rax
	movq	%rax, 56(%rsp)
	jmp	.L854
.L843:
	cmpl	$3, %ebx
	jne	.L846
	movq	24(%rsp), %rdi
	call	strdup
	movq	%rax, 56(%rsp)
	jmp	.L847
	.p2align 4,,10
	.p2align 3
.L850:
	cmpl	$3, %ebx
	jne	.L853
	movq	24(%rsp), %rdi
	call	strdup
	movq	%rax, 56(%rsp)
	jmp	.L854
.L857:
	.cfi_restore 3
	.cfi_restore 6
	.cfi_restore 12
	.cfi_restore 13
	.cfi_restore 14
	movl	$5381, %eax
	jmp	.L838
.L853:
	.cfi_offset 3, -56
	.cfi_offset 6, -48
	.cfi_offset 12, -40
	.cfi_offset 13, -32
	.cfi_offset 14, -24
	vmovdqa	%xmm0, 48(%rsp)
	jmp	.L855
.L842:
	movl	24(%rsp), %eax
	movl	%eax, 56(%rsp)
	jmp	.L847
.L869:
	movq	24(%rsp), %rax
	movq	%rax, 56(%rsp)
	jmp	.L847
.L846:
	vmovdqa	%xmm0, 48(%rsp)
	jmp	.L848
.L864:
	.cfi_restore 3
	.cfi_restore 6
	.cfi_restore 12
	.cfi_restore 13
	.cfi_restore 14
	movq	128(%rsp), %r15
	.cfi_restore 15
	jmp	.L862
	.cfi_endproc
.LFE76:
	.size	aly_object_set, .-aly_object_set
	.p2align 4
	.globl	aly_object_get
	.type	aly_object_get, @function
aly_object_get:
.LFB77:
	.cfi_startproc
	subq	$104, %rsp
	.cfi_def_cfa_offset 112
	cmpl	$5, 112(%rsp)
	movq	%r12, 72(%rsp)
	.cfi_offset 12, -40
	movq	%rdi, %r12
	movq	120(%rsp), %rdi
	jne	.L871
	testq	%rdi, %rdi
	je	.L871
	movq	%rbp, 64(%rsp)
	movq	%rsi, %rcx
	.cfi_offset 6, -48
	movq	%rsi, %rbp
	movl	$5381, %eax
	movq	%r13, 80(%rsp)
	movzbl	(%rsi), %edx
	testb	%dl, %dl
	.cfi_offset 13, -32
	je	.L873
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L872:
	movl	%eax, %esi
	addq	$1, %rcx
	sall	$5, %esi
	addl	%esi, %eax
	addl	%edx, %eax
	movzbl	(%rcx), %edx
	testb	%dl, %dl
	jne	.L872
.L873:
	movl	12(%rdi), %r13d
	testl	%r13d, %r13d
	jle	.L889
	xorl	%edx, %edx
	movq	%rbx, 56(%rsp)
	divl	%r13d
	movq	%r14, 88(%rsp)
	.cfi_offset 3, -56
	.cfi_offset 14, -24
	movq	(%rdi), %r14
	movq	%r15, 96(%rsp)
	.cfi_offset 15, -16
	leal	0(%r13,%rdx), %eax
	movl	%edx, %ebx
	movl	%eax, 12(%rsp)
	jmp	.L882
	.p2align 4,,10
	.p2align 3
.L893:
	movq	(%r15), %rdi
	movq	%rbp, %rsi
	call	strcmp
	testl	%eax, %eax
	je	.L892
	addl	$1, %ebx
	cmpl	12(%rsp), %ebx
	je	.L890
.L882:
	xorl	%edx, %edx
	movl	%ebx, %eax
	divl	%r13d
	leaq	(%rdx,%rdx,4), %rax
	leaq	(%r14,%rax,8), %r15
	movl	32(%r15), %eax
	testl	%eax, %eax
	jne	.L893
.L890:
	movq	56(%rsp), %rbx
	.cfi_restore 3
	movq	64(%rsp), %rbp
	.cfi_restore 6
	movq	80(%rsp), %r13
	.cfi_restore 13
	movq	88(%rsp), %r14
	.cfi_restore 14
	movq	96(%rsp), %r15
	.cfi_restore 15
.L871:
	movl	$6, (%r12)
	movq	%r12, %rax
	movq	$0, 8(%r12)
	movq	$0, 16(%r12)
	movq	72(%rsp), %r12
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L892:
	.cfi_def_cfa_offset 112
	.cfi_offset 3, -56
	.cfi_offset 6, -48
	.cfi_offset 13, -32
	.cfi_offset 14, -24
	.cfi_offset 15, -16
	movl	8(%r15), %eax
	vmovdqu	8(%r15), %xmm0
	movq	24(%r15), %rcx
	cmpl	$2, %eax
	vmovdqa	%xmm0, 16(%rsp)
	movq	%rcx, 32(%rsp)
	je	.L876
	ja	.L877
	testl	%eax, %eax
	je	.L894
	vmovsd	24(%rsp), %xmm0
	movl	$1, (%r12)
	movq	$0, 16(%r12)
	vmovsd	%xmm0, 8(%r12)
.L891:
	movq	%r12, %rax
	movq	56(%rsp), %rbx
	.cfi_remember_state
	.cfi_restore 3
	movq	64(%rsp), %rbp
	.cfi_restore 6
	movq	80(%rsp), %r13
	.cfi_restore 13
	movq	88(%rsp), %r14
	.cfi_restore 14
	movq	96(%rsp), %r15
	.cfi_restore 15
	movq	72(%rsp), %r12
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	ret
.L894:
	.cfi_restore_state
	movq	24(%rsp), %rax
	movl	$0, (%r12)
	movq	$0, 16(%r12)
	movq	%rax, 8(%r12)
	jmp	.L891
	.p2align 4,,10
	.p2align 3
.L877:
	cmpl	$3, %eax
	jne	.L880
	movq	24(%rsp), %rdi
	call	strdup
	movl	$3, (%r12)
	movq	%rax, 8(%r12)
	movq	$0, 16(%r12)
	jmp	.L891
.L876:
	movl	24(%rsp), %eax
	movl	$2, (%r12)
	movq	$0, 16(%r12)
	movl	%eax, 8(%r12)
	jmp	.L891
.L880:
	movq	%rcx, 16(%r12)
	vmovdqu	%xmm0, (%r12)
	jmp	.L891
.L889:
	.cfi_restore 3
	.cfi_restore 14
	.cfi_restore 15
	movq	64(%rsp), %rbp
	.cfi_restore 6
	movq	80(%rsp), %r13
	.cfi_restore 13
	jmp	.L871
	.cfi_endproc
.LFE77:
	.size	aly_object_get, .-aly_object_get
	.p2align 4
	.globl	aly_len
	.type	aly_len, @function
aly_len:
.LFB78:
	.cfi_startproc
	subq	$8, %rsp
	.cfi_def_cfa_offset 16
	movl	16(%rsp), %eax
	movq	24(%rsp), %rdi
	cmpl	$4, %eax
	je	.L896
	cmpl	$5, %eax
	je	.L896
	cmpl	$3, %eax
	jne	.L898
	testq	%rdi, %rdi
	je	.L898
	call	strlen
	addq	$8, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L896:
	.cfi_restore_state
	testq	%rdi, %rdi
	je	.L898
	movl	8(%rdi), %eax
	addq	$8, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L898:
	.cfi_restore_state
	xorl	%eax, %eax
	addq	$8, %rsp
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE78:
	.size	aly_len, .-aly_len
	.p2align 4
	.globl	aly_type_of
	.type	aly_type_of, @function
aly_type_of:
.LFB79:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movl	16(%rsp), %eax
	movq	%rdi, %rbx
	movl	$.LC2, %edi
	cmpl	$7, %eax
	ja	.L910
	movq	CSWTCH.102(,%rax,8), %rdi
.L910:
	call	strdup
	movl	$3, (%rbx)
	movq	%rax, 8(%rbx)
	movq	%rbx, %rax
	movq	$0, 16(%rbx)
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE79:
	.size	aly_type_of, .-aly_type_of
	.p2align 4
	.globl	aly_pow
	.type	aly_pow, @function
aly_pow:
.LFB80:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movq	%rdi, %rbx
	vxorps	%xmm2, %xmm2, %xmm2
	subq	$16, %rsp
	.cfi_def_cfa_offset 32
	movl	56(%rsp), %eax
	movq	64(%rsp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm1
	je	.L917
	cmpl	$3, %eax
	je	.L916
	testl	%eax, %eax
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L917
	vcvtsi2sdq	%rdi, %xmm2, %xmm1
.L917:
	movl	32(%rsp), %eax
	movq	40(%rsp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm0
	je	.L920
	cmpl	$3, %eax
	je	.L919
	testl	%eax, %eax
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L920
	vcvtsi2sdq	%rdi, %xmm2, %xmm2
	vmovapd	%xmm2, %xmm0
.L920:
	call	pow
	movl	$1, (%rbx)
	movq	%rbx, %rax
	movq	$0, 16(%rbx)
	vmovsd	%xmm0, 8(%rbx)
	addq	$16, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 16
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L916:
	.cfi_restore_state
	xorl	%esi, %esi
	call	strtod
	vxorps	%xmm2, %xmm2, %xmm2
	vmovapd	%xmm0, %xmm1
	jmp	.L917
	.p2align 4,,10
	.p2align 3
.L919:
	xorl	%esi, %esi
	vmovsd	%xmm1, 8(%rsp)
	call	strtod
	vmovsd	8(%rsp), %xmm1
	jmp	.L920
	.cfi_endproc
.LFE80:
	.size	aly_pow, .-aly_pow
	.p2align 4
	.globl	aly_sqrt
	.type	aly_sqrt, @function
aly_sqrt:
.LFB81:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L925
	cmpl	$3, %edx
	je	.L926
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L927
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
.L928:
	vxorpd	%xmm1, %xmm1, %xmm1
	vucomisd	%xmm0, %xmm1
	ja	.L933
	vsqrtsd	%xmm0, %xmm0, %xmm0
.L927:
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	vmovsd	%xmm0, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L926:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rax
	jmp	.L928
	.p2align 4,,10
	.p2align 3
.L925:
	vmovq	%rdi, %xmm0
	jmp	.L928
.L933:
	movq	%rax, 8(%rsp)
	call	sqrt
	movq	8(%rsp), %rax
	jmp	.L927
	.cfi_endproc
.LFE81:
	.size	aly_sqrt, .-aly_sqrt
	.p2align 4
	.globl	aly_round
	.type	aly_round, @function
aly_round:
.LFB82:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L936
	cmpl	$3, %edx
	je	.L937
	xorl	%ecx, %ecx
	testl	%edx, %edx
	jne	.L938
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
.L938:
	movl	$0, (%rax)
	movq	%rcx, 8(%rax)
	movq	$0, 16(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L937:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	call	round
	movq	8(%rsp), %rax
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L938
	.p2align 4,,10
	.p2align 3
.L936:
	vmovq	%rdi, %xmm0
	movq	%rax, 8(%rsp)
	call	round
	movq	8(%rsp), %rax
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L938
	.cfi_endproc
.LFE82:
	.size	aly_round, .-aly_round
	.p2align 4
	.globl	aly_round_up
	.type	aly_round_up, @function
aly_round_up:
.LFB83:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L942
	cmpl	$3, %edx
	je	.L943
	xorl	%ecx, %ecx
	testl	%edx, %edx
	jne	.L944
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vroundsd	$10, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
.L944:
	movl	$0, (%rax)
	movq	%rcx, 8(%rax)
	movq	$0, 16(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L943:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rax
	vroundsd	$10, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L944
	.p2align 4,,10
	.p2align 3
.L942:
	vmovq	%rdi, %xmm0
	vroundsd	$10, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L944
	.cfi_endproc
.LFE83:
	.size	aly_round_up, .-aly_round_up
	.p2align 4
	.globl	aly_round_down
	.type	aly_round_down, @function
aly_round_down:
.LFB84:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L948
	cmpl	$3, %edx
	je	.L949
	xorl	%ecx, %ecx
	testl	%edx, %edx
	jne	.L950
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vroundsd	$9, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
.L950:
	movl	$0, (%rax)
	movq	%rcx, 8(%rax)
	movq	$0, 16(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L949:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rax
	vroundsd	$9, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L950
	.p2align 4,,10
	.p2align 3
.L948:
	vmovq	%rdi, %xmm0
	vroundsd	$9, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L950
	.cfi_endproc
.LFE84:
	.size	aly_round_down, .-aly_round_down
	.p2align 4
	.globl	aly_abs
	.type	aly_abs, @function
aly_abs:
.LFB85:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, %rcx
	movl	64(%rsp), %eax
	vmovsd	72(%rsp), %xmm0
	cmpl	$1, %eax
	je	.L961
	vmovdqu	64(%rsp), %xmm1
	cmpl	$2, %eax
	vmovdqa	%xmm1, 16(%rsp)
	je	.L956
	ja	.L962
	vmovq	%xmm0, %rdi
	movq	%rdi, %rax
	negq	%rax
	cmovs	%rdi, %rax
.L959:
	movq	%rax, 8(%rcx)
	movq	%rcx, %rax
	movl	$0, (%rcx)
	movq	$0, 16(%rcx)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L961:
	.cfi_restore_state
	vandpd	.LC21(%rip), %xmm0, %xmm0
	movl	$1, (%rdi)
	movq	%rcx, %rax
	movq	$0, 16(%rcx)
	vmovsd	%xmm0, 8(%rdi)
	addq	$56, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L962:
	.cfi_restore_state
	cmpl	$3, %eax
	jne	.L963
	movq	%rdi, 8(%rsp)
	movl	$10, %edx
	xorl	%esi, %esi
	vmovq	%xmm0, %rdi
	call	strtoll
	movq	8(%rsp), %rcx
	movq	%rax, %rdx
	negq	%rax
	cmovs	%rdx, %rax
	jmp	.L959
	.p2align 4,,10
	.p2align 3
.L956:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	jmp	.L959
	.p2align 4,,10
	.p2align 3
.L963:
	xorl	%eax, %eax
	jmp	.L959
	.cfi_endproc
.LFE85:
	.size	aly_abs, .-aly_abs
	.p2align 4
	.globl	aly_random
	.type	aly_random, @function
aly_random:
.LFB86:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movq	%rdi, %rbx
	call	rand
	vxorps	%xmm0, %xmm0, %xmm0
	movl	$1, (%rbx)
	vcvtsi2sdl	%eax, %xmm0, %xmm0
	movq	$0, 16(%rbx)
	movq	%rbx, %rax
	vdivsd	.LC22(%rip), %xmm0, %xmm0
	vmovsd	%xmm0, 8(%rbx)
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE86:
	.size	aly_random, .-aly_random
	.p2align 4
	.globl	aly_sin
	.type	aly_sin, @function
aly_sin:
.LFB87:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L967
	cmpl	$3, %edx
	je	.L968
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L969
	vxorps	%xmm0, %xmm0, %xmm0
	movq	%rax, 8(%rsp)
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	call	sin
	movq	8(%rsp), %rax
.L969:
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	vmovsd	%xmm0, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L968:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	call	sin
	movq	8(%rsp), %rax
	jmp	.L969
	.p2align 4,,10
	.p2align 3
.L967:
	vmovq	%rdi, %xmm0
	movq	%rax, 8(%rsp)
	call	sin
	movq	8(%rsp), %rax
	jmp	.L969
	.cfi_endproc
.LFE87:
	.size	aly_sin, .-aly_sin
	.p2align 4
	.globl	aly_cos
	.type	aly_cos, @function
aly_cos:
.LFB88:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L973
	cmpl	$3, %edx
	je	.L974
	testl	%edx, %edx
	vmovsd	.LC23(%rip), %xmm0
	jne	.L975
	vxorps	%xmm0, %xmm0, %xmm0
	movq	%rax, 8(%rsp)
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	call	cos
	movq	8(%rsp), %rax
.L975:
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	vmovsd	%xmm0, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L974:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	call	cos
	movq	8(%rsp), %rax
	jmp	.L975
	.p2align 4,,10
	.p2align 3
.L973:
	vmovq	%rdi, %xmm0
	movq	%rax, 8(%rsp)
	call	cos
	movq	8(%rsp), %rax
	jmp	.L975
	.cfi_endproc
.LFE88:
	.size	aly_cos, .-aly_cos
	.p2align 4
	.globl	aly_tan
	.type	aly_tan, @function
aly_tan:
.LFB89:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L979
	cmpl	$3, %edx
	je	.L980
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L981
	vxorps	%xmm0, %xmm0, %xmm0
	movq	%rax, 8(%rsp)
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	call	tan
	movq	8(%rsp), %rax
.L981:
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	vmovsd	%xmm0, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L980:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	call	tan
	movq	8(%rsp), %rax
	jmp	.L981
	.p2align 4,,10
	.p2align 3
.L979:
	vmovq	%rdi, %xmm0
	movq	%rax, 8(%rsp)
	call	tan
	movq	8(%rsp), %rax
	jmp	.L981
	.cfi_endproc
.LFE89:
	.size	aly_tan, .-aly_tan
	.p2align 4
	.globl	aly_log
	.type	aly_log, @function
aly_log:
.LFB90:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movl	16(%rsp), %eax
	movq	%rdi, %rbx
	movq	24(%rsp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm0
	je	.L987
	cmpl	$3, %eax
	je	.L986
	testl	%eax, %eax
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L987
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
.L987:
	call	log10
	movq	%rbx, %rax
	movl	$1, (%rbx)
	movq	$0, 16(%rbx)
	vmovsd	%xmm0, 8(%rbx)
	popq	%rbx
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L986:
	.cfi_restore_state
	xorl	%esi, %esi
	call	strtod
	jmp	.L987
	.cfi_endproc
.LFE90:
	.size	aly_log, .-aly_log
	.p2align 4
	.globl	aly_ln
	.type	aly_ln, @function
aly_ln:
.LFB91:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movl	16(%rsp), %eax
	movq	%rdi, %rbx
	movq	24(%rsp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm0
	je	.L993
	cmpl	$3, %eax
	je	.L992
	testl	%eax, %eax
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L993
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
.L993:
	call	log
	movq	%rbx, %rax
	movl	$1, (%rbx)
	movq	$0, 16(%rbx)
	vmovsd	%xmm0, 8(%rbx)
	popq	%rbx
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L992:
	.cfi_restore_state
	xorl	%esi, %esi
	call	strtod
	jmp	.L993
	.cfi_endproc
.LFE91:
	.size	aly_ln, .-aly_ln
	.p2align 4
	.globl	aly_min
	.type	aly_min, @function
aly_min:
.LFB92:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	vxorps	%xmm2, %xmm2, %xmm2
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	vmovq	%rdi, %xmm1
	je	.L999
	cmpl	$3, %edx
	je	.L998
	testl	%edx, %edx
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L999
	vcvtsi2sdq	%rdi, %xmm2, %xmm1
.L999:
	movl	56(%rsp), %edx
	movq	64(%rsp), %rdi
	cmpl	$1, %edx
	vmovq	%rdi, %xmm0
	je	.L1002
	cmpl	$3, %edx
	je	.L1001
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L1002
	vcvtsi2sdq	%rdi, %xmm2, %xmm2
	vmovapd	%xmm2, %xmm0
.L1002:
	vcomisd	%xmm1, %xmm0
	jbe	.L1009
	vmovdqu	32(%rsp), %xmm0
	movq	48(%rsp), %rdx
	movq	%rdx, 16(%rax)
	vmovdqu	%xmm0, (%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L1009:
	.cfi_restore_state
	vmovdqu	56(%rsp), %xmm0
	movq	72(%rsp), %rdx
	movq	%rdx, 16(%rax)
	vmovdqu	%xmm0, (%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L998:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, (%rsp)
	call	strtod
	movq	(%rsp), %rax
	vxorps	%xmm2, %xmm2, %xmm2
	vmovapd	%xmm0, %xmm1
	jmp	.L999
	.p2align 4,,10
	.p2align 3
.L1001:
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	vmovsd	%xmm1, (%rsp)
	call	strtod
	movq	8(%rsp), %rax
	vmovsd	(%rsp), %xmm1
	jmp	.L1002
	.cfi_endproc
.LFE92:
	.size	aly_min, .-aly_min
	.p2align 4
	.globl	aly_max
	.type	aly_max, @function
aly_max:
.LFB93:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	vxorps	%xmm2, %xmm2, %xmm2
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	vmovq	%rdi, %xmm1
	je	.L1014
	cmpl	$3, %edx
	je	.L1013
	testl	%edx, %edx
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L1014
	vcvtsi2sdq	%rdi, %xmm2, %xmm1
.L1014:
	movl	56(%rsp), %edx
	movq	64(%rsp), %rdi
	cmpl	$1, %edx
	vmovq	%rdi, %xmm0
	je	.L1017
	cmpl	$3, %edx
	je	.L1016
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L1017
	vcvtsi2sdq	%rdi, %xmm2, %xmm2
	vmovapd	%xmm2, %xmm0
.L1017:
	vcomisd	%xmm0, %xmm1
	jbe	.L1024
	vmovdqu	32(%rsp), %xmm0
	movq	48(%rsp), %rdx
	movq	%rdx, 16(%rax)
	vmovdqu	%xmm0, (%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L1024:
	.cfi_restore_state
	vmovdqu	56(%rsp), %xmm0
	movq	72(%rsp), %rdx
	movq	%rdx, 16(%rax)
	vmovdqu	%xmm0, (%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L1013:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, (%rsp)
	call	strtod
	movq	(%rsp), %rax
	vxorps	%xmm2, %xmm2, %xmm2
	vmovapd	%xmm0, %xmm1
	jmp	.L1014
	.p2align 4,,10
	.p2align 3
.L1016:
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	vmovsd	%xmm1, (%rsp)
	call	strtod
	movq	8(%rsp), %rax
	vmovsd	(%rsp), %xmm1
	jmp	.L1017
	.cfi_endproc
.LFE93:
	.size	aly_max, .-aly_max
	.p2align 4
	.globl	aly_str_upper
	.type	aly_str_upper, @function
aly_str_upper:
.LFB94:
	.cfi_startproc
	pushq	%r12
	.cfi_def_cfa_offset 16
	.cfi_offset 12, -16
	movq	%rdi, %r12
	pushq	%rbp
	.cfi_def_cfa_offset 24
	.cfi_offset 6, -24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset 3, -32
	subq	$32, %rsp
	.cfi_def_cfa_offset 64
	movq	%rsp, %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 96
	movq	112(%rsp), %rax
	vmovdqu	96(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	40(%rsp), %rbp
	addq	$32, %rsp
	.cfi_def_cfa_offset 64
	movsbq	0(%rbp), %rbx
	testb	%bl, %bl
	je	.L1027
	call	__ctype_toupper_loc
	movq	%rax, %rdx
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L1028:
	movq	(%rdx), %rax
	addq	$1, %rbp
	movl	(%rax,%rbx,4), %eax
	movb	%al, -1(%rbp)
	movsbq	0(%rbp), %rbx
	testb	%bl, %bl
	jne	.L1028
.L1027:
	movq	16(%rsp), %rax
	vmovdqa	(%rsp), %xmm0
	movq	%rax, 16(%r12)
	movq	%r12, %rax
	vmovdqu	%xmm0, (%r12)
	addq	$32, %rsp
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%rbp
	.cfi_def_cfa_offset 16
	popq	%r12
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE94:
	.size	aly_str_upper, .-aly_str_upper
	.p2align 4
	.globl	aly_str_lower
	.type	aly_str_lower, @function
aly_str_lower:
.LFB95:
	.cfi_startproc
	pushq	%r12
	.cfi_def_cfa_offset 16
	.cfi_offset 12, -16
	movq	%rdi, %r12
	pushq	%rbp
	.cfi_def_cfa_offset 24
	.cfi_offset 6, -24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset 3, -32
	subq	$32, %rsp
	.cfi_def_cfa_offset 64
	movq	%rsp, %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 96
	movq	112(%rsp), %rax
	vmovdqu	96(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	40(%rsp), %rbp
	addq	$32, %rsp
	.cfi_def_cfa_offset 64
	movsbq	0(%rbp), %rbx
	testb	%bl, %bl
	je	.L1035
	call	__ctype_tolower_loc
	movq	%rax, %rdx
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L1036:
	movq	(%rdx), %rax
	addq	$1, %rbp
	movl	(%rax,%rbx,4), %eax
	movb	%al, -1(%rbp)
	movsbq	0(%rbp), %rbx
	testb	%bl, %bl
	jne	.L1036
.L1035:
	movq	16(%rsp), %rax
	vmovdqa	(%rsp), %xmm0
	movq	%rax, 16(%r12)
	movq	%r12, %rax
	vmovdqu	%xmm0, (%r12)
	addq	$32, %rsp
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%rbp
	.cfi_def_cfa_offset 16
	popq	%r12
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE95:
	.size	aly_str_lower, .-aly_str_lower
	.p2align 4
	.globl	aly_str_trim
	.type	aly_str_trim, @function
aly_str_trim:
.LFB96:
	.cfi_startproc
	pushq	%r12
	.cfi_def_cfa_offset 16
	.cfi_offset 12, -16
	movq	%rdi, %r12
	pushq	%rbp
	.cfi_def_cfa_offset 24
	.cfi_offset 6, -24
	pushq	%rbx
	.cfi_def_cfa_offset 32
	.cfi_offset 3, -32
	subq	$32, %rsp
	.cfi_def_cfa_offset 64
	movq	%rsp, %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 96
	movq	112(%rsp), %rax
	vmovdqu	96(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	40(%rsp), %rbx
	addq	$32, %rsp
	.cfi_def_cfa_offset 64
	call	__ctype_b_loc
	movq	(%rax), %rbp
	movsbq	(%rbx), %rax
	testb	$32, 1(%rbp,%rax,2)
	je	.L1043
	.p2align 4
	.p2align 4,,10
	.p2align 3
.L1044:
	movsbq	1(%rbx), %rax
	addq	$1, %rbx
	testb	$32, 1(%rbp,%rax,2)
	jne	.L1044
.L1043:
	movq	%rbx, %rdi
	call	strlen
	leaq	-1(%rbx,%rax), %rax
	cmpq	%rax, %rbx
	jb	.L1045
	jmp	.L1046
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L1047:
	subq	$1, %rax
	cmpq	%rbx, %rax
	je	.L1046
.L1045:
	movsbq	(%rax), %rdx
	testb	$32, 1(%rbp,%rdx,2)
	jne	.L1047
.L1046:
	movb	$0, 1(%rax)
	movq	%rbx, %rdi
	call	strdup
	subq	$32, %rsp
	.cfi_def_cfa_offset 96
	movq	%rax, %rbx
	movq	48(%rsp), %rax
	vmovdqa	32(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	movl	$3, (%r12)
	movq	%r12, %rax
	movq	%rbx, 8(%r12)
	movq	$0, 16(%r12)
	addq	$64, %rsp
	.cfi_def_cfa_offset 32
	popq	%rbx
	.cfi_def_cfa_offset 24
	popq	%rbp
	.cfi_def_cfa_offset 16
	popq	%r12
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE96:
	.size	aly_str_trim, .-aly_str_trim
	.p2align 4
	.globl	aly_str_contains
	.type	aly_str_contains, @function
aly_str_contains:
.LFB97:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	subq	$64, %rsp
	.cfi_def_cfa_offset 80
	movq	%rsp, %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	128(%rsp), %rax
	vmovdqu	112(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	152(%rsp), %rax
	leaq	64(%rsp), %rdi
	vmovdqu	136(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	40(%rsp), %rdi
	movq	72(%rsp), %rsi
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	call	strstr
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	%rax, %rbx
	movq	48(%rsp), %rax
	vmovdqa	32(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	movq	80(%rsp), %rax
	vmovdqa	64(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	xorl	%eax, %eax
	testq	%rbx, %rbx
	setne	%al
	addq	$96, %rsp
	.cfi_def_cfa_offset 16
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE97:
	.size	aly_str_contains, .-aly_str_contains
	.section	.rodata.str1.1
.LC24:
	.string	"r"
.LC25:
	.string	"Error: Cannot open file\n"
	.text
	.p2align 4
	.globl	aly_fs_read
	.type	aly_fs_read, @function
aly_fs_read:
.LFB98:
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	%rbx, 40(%rsp)
	movq	%rbp, 48(%rsp)
	.cfi_offset 3, -40
	.cfi_offset 6, -32
	movq	%rdi, %rbp
	movq	%rsp, %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	128(%rsp), %rax
	vmovdqu	112(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	40(%rsp), %rdi
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	movl	$.LC24, %esi
	call	fopen
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	%rax, %rbx
	movq	48(%rsp), %rax
	vmovdqa	32(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	testq	%rbx, %rbx
	je	.L1061
	movl	$2, %edx
	xorl	%esi, %esi
	movq	%rbx, %rdi
	movq	%r12, 56(%rsp)
	movq	%r13, 64(%rsp)
	.cfi_offset 12, -24
	.cfi_offset 13, -16
	call	fseek
	movq	%rbx, %rdi
	call	ftell
	xorl	%edx, %edx
	xorl	%esi, %esi
	movq	%rbx, %rdi
	movq	%rax, %r13
	call	fseek
	leaq	1(%r13), %rdi
	call	malloc
	movq	%rbx, %rcx
	movq	%r13, %rdx
	movl	$1, %esi
	movq	%rax, %r12
	movq	%rax, %rdi
	call	fread
	movb	$0, (%r12,%r13)
	movq	%rbx, %rdi
	call	fclose
	movq	%r12, %rdi
	call	strdup
	movq	%r12, %rdi
	movq	%rax, %rbx
	call	free
	movq	56(%rsp), %r12
	.cfi_restore 12
	movq	64(%rsp), %r13
	.cfi_restore 13
.L1059:
	movq	%rbx, 8(%rbp)
	movq	%rbp, %rax
	movq	40(%rsp), %rbx
	movl	$3, 0(%rbp)
	movq	$0, 16(%rbp)
	movq	48(%rsp), %rbp
	addq	$72, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
.L1061:
	.cfi_restore_state
	movl	$24, %edx
	movl	$1, %esi
	movl	$.LC25, %edi
	movq	stderr(%rip), %rcx
	call	fwrite
	movl	$.LC20, %edi
	call	strdup
	movq	%rax, %rbx
	jmp	.L1059
	.cfi_endproc
.LFE98:
	.size	aly_fs_read, .-aly_fs_read
	.section	.rodata.str1.1
.LC26:
	.string	"w"
	.text
	.p2align 4
	.globl	aly_fs_write
	.type	aly_fs_write, @function
aly_fs_write:
.LFB99:
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	%rsp, %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	128(%rsp), %rax
	vmovdqu	112(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	152(%rsp), %rax
	leaq	64(%rsp), %rdi
	vmovdqu	136(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	40(%rsp), %rdi
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	movl	$.LC26, %esi
	call	fopen
	testq	%rax, %rax
	je	.L1063
	movq	40(%rsp), %rdi
	movq	%rbx, 64(%rsp)
	.cfi_offset 3, -16
	movq	%rax, %rsi
	movq	%rax, %rbx
	call	fputs
	movq	%rbx, %rdi
	call	fclose
	movq	64(%rsp), %rbx
	.cfi_restore 3
.L1063:
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	48(%rsp), %rax
	vmovdqa	32(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	vmovdqa	64(%rsp), %xmm0
	movq	80(%rsp), %rax
	vmovdqu	%xmm0, 112(%rsp)
	movq	%rax, 128(%rsp)
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	jmp	aly_free
	.cfi_endproc
.LFE99:
	.size	aly_fs_write, .-aly_fs_write
	.section	.rodata.str1.1
.LC27:
	.string	"a"
	.text
	.p2align 4
	.globl	aly_fs_append
	.type	aly_fs_append, @function
aly_fs_append:
.LFB100:
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	%rsp, %rdi
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	128(%rsp), %rax
	vmovdqu	112(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	152(%rsp), %rax
	leaq	64(%rsp), %rdi
	vmovdqu	136(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	40(%rsp), %rdi
	addq	$32, %rsp
	.cfi_def_cfa_offset 80
	movl	$.LC27, %esi
	call	fopen
	testq	%rax, %rax
	je	.L1069
	movq	40(%rsp), %rdi
	movq	%rbx, 64(%rsp)
	.cfi_offset 3, -16
	movq	%rax, %rsi
	movq	%rax, %rbx
	call	fputs
	movq	%rbx, %rdi
	call	fclose
	movq	64(%rsp), %rbx
	.cfi_restore 3
.L1069:
	subq	$32, %rsp
	.cfi_def_cfa_offset 112
	movq	48(%rsp), %rax
	vmovdqa	32(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_free
	vmovdqa	64(%rsp), %xmm0
	movq	80(%rsp), %rax
	vmovdqu	%xmm0, 112(%rsp)
	movq	%rax, 128(%rsp)
	addq	$104, %rsp
	.cfi_def_cfa_offset 8
	jmp	aly_free
	.cfi_endproc
.LFE100:
	.size	aly_fs_append, .-aly_fs_append
	.p2align 4
	.globl	aly_fs_exists
	.type	aly_fs_exists, @function
aly_fs_exists:
.LFB101:
	.cfi_startproc
	subq	$88, %rsp
	.cfi_def_cfa_offset 96
	movq	%rbx, 56(%rsp)
	leaq	16(%rsp), %rdi
	movq	%rbp, 64(%rsp)
	movq	%r12, 72(%rsp)
	subq	$32, %rsp
	.cfi_def_cfa_offset 128
	.cfi_offset 3, -40
	.cfi_offset 6, -32
	.cfi_offset 12, -24
	movq	144(%rsp), %rax
	vmovdqu	128(%rsp), %xmm0
	movq	%rax, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_to_str
	movq	56(%rsp), %rbx
	movl	$.LC24, %esi
	addq	$32, %rsp
	.cfi_def_cfa_offset 96
	movq	%rbx, %rdi
	call	fopen
	movq	32(%rsp), %r12
	movq	%rax, %rbp
	movl	16(%rsp), %eax
	cmpl	$4, %eax
	je	.L1075
	cmpl	$5, %eax
	je	.L1076
	cmpl	$3, %eax
	jne	.L1077
.L1099:
	movq	%rbx, %rdi
	call	free
.L1077:
	testq	%r12, %r12
	je	.L1084
	movq	%r12, %rdi
	call	free
.L1084:
	testq	%rbp, %rbp
	je	.L1086
	movq	%rbp, %rdi
	call	fclose
	movl	$1, %eax
.L1074:
	movq	56(%rsp), %rbx
	movq	64(%rsp), %rbp
	movq	72(%rsp), %r12
	addq	$88, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L1076:
	.cfi_restore_state
	testq	%rbx, %rbx
	je	.L1077
	movl	12(%rbx), %ecx
	movq	(%rbx), %rdi
	testl	%ecx, %ecx
	jle	.L1080
	movq	%r13, 80(%rsp)
	.cfi_offset 13, -16
	xorl	%r13d, %r13d
.L1083:
	leaq	0(%r13,%r13,4), %rax
	movl	32(%rdi,%rax,8), %edx
	testl	%edx, %edx
	jne	.L1100
	addq	$1, %r13
	cmpl	%r13d, %ecx
	jg	.L1083
.L1098:
	movq	80(%rsp), %r13
	.cfi_restore 13
.L1080:
	call	free
	jmp	.L1099
	.p2align 4,,10
	.p2align 3
.L1075:
	testq	%rbx, %rbx
	je	.L1077
	movl	8(%rbx), %esi
	testl	%esi, %esi
	jle	.L1078
	movq	%r13, 80(%rsp)
	.cfi_offset 13, -16
	xorl	%r13d, %r13d
.L1079:
	movq	(%rbx), %rcx
	leaq	0(%r13,%r13,2), %rax
	subq	$32, %rsp
	.cfi_def_cfa_offset 128
	addq	$1, %r13
	leaq	(%rcx,%rax,8), %rax
	vmovdqu	(%rax), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%rax), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	addq	$32, %rsp
	.cfi_def_cfa_offset 96
	cmpl	%r13d, 8(%rbx)
	jg	.L1079
	movq	80(%rsp), %r13
	.cfi_restore 13
.L1078:
	movq	(%rbx), %rdi
	call	free
	movq	%rbx, %rdi
	call	free
	jmp	.L1077
	.p2align 4,,10
	.p2align 3
.L1100:
	.cfi_offset 13, -16
	movq	(%rdi,%rax,8), %rdi
	movq	%rax, 8(%rsp)
	addq	$1, %r13
	call	free
	subq	$32, %rsp
	.cfi_def_cfa_offset 128
	movq	(%rbx), %rdx
	movq	40(%rsp), %rax
	vmovdqu	8(%rdx,%rax,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	24(%rdx,%rax,8), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	movl	12(%rbx), %ecx
	addq	$32, %rsp
	.cfi_def_cfa_offset 96
	movq	(%rbx), %rdi
	cmpl	%r13d, %ecx
	jg	.L1083
	jmp	.L1098
.L1086:
	.cfi_restore 13
	xorl	%eax, %eax
	jmp	.L1074
	.cfi_endproc
.LFE101:
	.size	aly_fs_exists, .-aly_fs_exists
	.p2align 4
	.globl	aly_ref
	.type	aly_ref, @function
aly_ref:
.LFB102:
	.cfi_startproc
	ret
	.cfi_endproc
.LFE102:
	.size	aly_ref, .-aly_ref
	.p2align 4
	.globl	aly_init
	.type	aly_init, @function
aly_init:
.LFB103:
	.cfi_startproc
	subq	$8, %rsp
	.cfi_def_cfa_offset 16
	movl	%edi, _aly_argc(%rip)
	xorl	%edi, %edi
	movq	%rsi, _aly_argv(%rip)
	call	time
	addq	$8, %rsp
	.cfi_def_cfa_offset 8
	movl	%eax, %edi
	jmp	srand
	.cfi_endproc
.LFE103:
	.size	aly_init, .-aly_init
	.p2align 4
	.globl	aly_cleanup
	.type	aly_cleanup, @function
aly_cleanup:
.LFB104:
	.cfi_startproc
	ret
	.cfi_endproc
.LFE104:
	.size	aly_cleanup, .-aly_cleanup
	.p2align 4
	.globl	aly_sys_args
	.type	aly_sys_args, @function
aly_sys_args:
.LFB105:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movl	$16, %esi
	movq	%r14, 32(%rsp)
	.cfi_offset 14, -16
	movl	_aly_argc(%rip), %r14d
	movq	%r13, 24(%rsp)
	.cfi_offset 13, -24
	movq	%rdi, %r13
	movl	$1, %edi
	movq	%r12, 16(%rsp)
	.cfi_offset 12, -32
	call	calloc
	testl	%r14d, %r14d
	movq	%rax, %r12
	jle	.L1111
	movq	%rbx, (%rsp)
	movq	%r14, %rdi
	movl	$24, %esi
	movq	%rbp, 8(%rsp)
	.cfi_offset 3, -48
	.cfi_offset 6, -40
	movl	%r14d, 12(%rax)
	call	calloc
	movq	_aly_argv(%rip), %rbp
	movl	$0, 8(%r12)
	movq	%rax, (%r12)
	movq	%rax, %rbx
	leaq	0(%rbp,%r14,8), %r14
	.p2align 4,,10
	.p2align 3
.L1108:
	movq	0(%rbp), %rdi
	addq	$8, %rbp
	addq	$24, %rbx
	call	strdup
	movl	$3, -24(%rbx)
	movq	%rax, -16(%rbx)
	movq	$0, -8(%rbx)
	addl	$1, 8(%r12)
	cmpq	%r14, %rbp
	jne	.L1108
	movq	(%rsp), %rbx
	.cfi_restore 3
	movq	8(%rsp), %rbp
	.cfi_restore 6
.L1107:
	movq	%r12, 8(%r13)
	movq	%r13, %rax
	movq	16(%rsp), %r12
	movl	$4, 0(%r13)
	movq	32(%rsp), %r14
	movq	$0, 16(%r13)
	movq	24(%rsp), %r13
	addq	$40, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L1111:
	.cfi_restore_state
	movl	$24, %esi
	movl	$8, %edi
	call	calloc
	movq	%rax, (%r12)
	movabsq	$34359738368, %rax
	movq	%rax, 8(%r12)
	jmp	.L1107
	.cfi_endproc
.LFE105:
	.size	aly_sys_args, .-aly_sys_args
	.section	.rodata.str1.1
.LC30:
	.string	"i"
.LC31:
	.string	"sum"
.LC32:
	.string	"<"
	.section	.rodata.str1.8,"aMS",@progbits,1
	.align 8
.LC33:
	.string	"Conditional loop sum (expected 10): "
	.align 8
.LC34:
	.string	"Conditional loop index (expected 5): "
	.section	.rodata.str1.1
.LC35:
	.string	"sum2"
.LC36:
	.string	"j"
	.section	.rodata.str1.8
	.align 8
.LC37:
	.string	"Three-part loop sum2 (expected 15): "
	.section	.rodata.str1.1
.LC38:
	.string	"k"
.LC39:
	.string	"even_count"
	.section	.rodata.str1.8
	.align 8
.LC40:
	.string	"Nested if in loop even_count (expected 3): "
	.text
	.p2align 4
	.globl	fn_0
	.type	fn_0, @function
fn_0:
.LFB110:
	.cfi_startproc
	leaq	8(%rsp), %r10
	.cfi_def_cfa 10, 0
	andq	$-32, %rsp
	pushq	-8(%r10)
	pushq	%rbp
	movq	%rsp, %rbp
	.cfi_escape 0x10,0x6,0x2,0x76,0
	pushq	%r15
	.cfi_escape 0x10,0xf,0x2,0x76,0x78
	xorl	%r15d, %r15d
	pushq	%r14
	pushq	%r13
	pushq	%r12
	pushq	%r10
	.cfi_escape 0xf,0x3,0x76,0x58,0x6
	.cfi_escape 0x10,0xe,0x2,0x76,0x70
	.cfi_escape 0x10,0xd,0x2,0x76,0x68
	.cfi_escape 0x10,0xc,0x2,0x76,0x60
	pushq	%rbx
	.cfi_escape 0x10,0x3,0x2,0x76,0x50
	xorl	%ebx, %ebx
	subq	$24752, %rsp
	movq	%rdi, -24728(%rbp)
	movl	$.LC30, %edi
	movl	$0, -24624(%rbp)
	movq	$0, -24616(%rbp)
	vmovdqa	-24624(%rbp), %xmm0
	movq	$0, -24608(%rbp)
	movq	$0, 40(%rsp)
	vmovdqu	%xmm0, 24(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	vmovdqa	-24624(%rbp), %xmm0
	movl	$.LC31, %edi
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp), %rax
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	addq	$48, %rsp
	.p2align 4,,10
	.p2align 3
.L1113:
	leaq	(%r15,%r15,2), %r10
	movq	aly_globals+8(%rip), %rdx
	movl	aly_globals(%rip), %esi
	leal	1(%rbx), %r12d
	leaq	-24624(%rbp,%r10,8), %r9
	movslq	%r12d, %r14
	movq	%r10, -24712(%rbp)
	movq	%r9, %rdi
	movq	%r9, -24704(%rbp)
	movl	%r12d, -24720(%rbp)
	call	aly_object_get.constprop.2.isra.0
	leaq	(%r14,%r14,2), %rax
	subq	$48, %rsp
	leaq	-24688(%rbp), %rdi
	leaq	0(,%rax,8), %r13
	leaq	-24624(%rbp,%r13), %rax
	movl	$0, (%rax)
	movq	$5, -24616(%rbp,%r13)
	vmovdqu	-24624(%rbp,%r13), %xmm0
	movq	$0, -24608(%rbp,%r13)
	movq	%rax, -24696(%rbp)
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp,%r13), %rsi
	movq	%rsi, 40(%rsp)
	movq	-24712(%rbp), %r10
	vmovdqu	-24624(%rbp,%r10,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	-24704(%rbp), %r9
	movq	16(%r9), %rsi
	movq	%rsi, 16(%rsp)
	movl	$.LC32, %esi
	call	aly_compare
	movq	-24712(%rbp), %r10
	movq	-24704(%rbp), %r9
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	vmovdqu	%xmm0, -24624(%rbp,%r10,8)
	movq	-24672(%rbp), %rsi
	vmovdqa	%xmm0, -24656(%rbp)
	movq	%rsi, 16(%r9)
	leaq	-24648(%rbp,%r13), %r9
	cmpl	$5, (%r9)
	movq	%rsi, -24640(%rbp)
	ja	.L1122
	movl	(%r9), %esi
	jmp	*.L1116(,%rsi,8)
	.section	.rodata
	.align 8
	.align 4
.L1116:
	.quad	.L1121
	.quad	.L1120
	.quad	.L1119
	.quad	.L1118
	.quad	.L1115
	.quad	.L1115
	.text
	.p2align 4,,10
	.p2align 3
.L1115:
	movq	-24648(%rbp), %rsi
	testq	%rsi, %rsi
	je	.L1122
	movl	8(%rsi), %esi
	testl	%esi, %esi
	jle	.L1122
.L1126:
	testl	%ebx, %ebx
	jle	.L1276
.L1129:
	leal	-1(%rbx), %eax
	movslq	%ebx, %r14
	movl	%r12d, %ebx
.L1130:
	vmovdqa	aly_globals(%rip), %xmm0
	subq	$32, %rsp
	cltq
	leaq	(%r14,%r14,2), %r14
	leaq	(%rax,%rax,2), %r13
	leaq	-24624(%rbp,%r14,8), %r12
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rsi
	leaq	-24624(%rbp,%r13,8), %r15
	movq	%r15, %rdi
	movq	%rsi, 16(%rsp)
	movl	$.LC31, %esi
	call	aly_object_get
	movl	aly_globals(%rip), %esi
	addq	$32, %rsp
	movq	%r12, %rdi
	movq	aly_globals+8(%rip), %rdx
	call	aly_object_get.constprop.2.isra.0
	subq	$48, %rsp
	leaq	-24688(%rbp), %rdi
	vmovdqu	-24624(%rbp,%r14,8), %xmm0
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r12), %rdx
	movq	%rdx, 40(%rsp)
	vmovdqu	-24624(%rbp,%r13,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%r15), %rdx
	movq	%rdx, 16(%rsp)
	call	aly_add
	vmovdqu	-24688(%rbp), %xmm0
	movl	$.LC31, %edi
	vmovdqu	%xmm0, -24624(%rbp,%r13,8)
	movq	-24672(%rbp), %rdx
	movq	%rdx, 16(%r15)
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r15), %rax
	movslq	%ebx, %r15
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	movq	aly_globals+8(%rip), %rdx
	addq	$48, %rsp
	movq	%r12, %rdi
	movl	aly_globals(%rip), %esi
	call	aly_object_get.constprop.2.isra.0
	movq	-24696(%rbp), %rax
	subq	$48, %rsp
	leaq	-24688(%rbp), %rdi
	movl	$0, (%rax)
	leaq	(%r15,%r15,2), %rax
	movq	$1, -24616(%rbp,%rax,8)
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	movq	$0, -24608(%rbp,%rax,8)
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp,%rax,8), %rax
	movq	%rax, 40(%rsp)
	vmovdqu	-24624(%rbp,%r14,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%r12), %rax
	movq	%rax, 16(%rsp)
	call	aly_add
	vmovdqu	-24688(%rbp), %xmm0
	movl	$.LC30, %edi
	vmovdqu	%xmm0, -24624(%rbp,%r14,8)
	movq	-24672(%rbp), %rax
	movq	%rax, 16(%r12)
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r12), %rax
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	addq	$48, %rsp
	jmp	.L1113
	.p2align 4,,10
	.p2align 3
.L1118:
	movq	-24648(%rbp), %rsi
	testq	%rsi, %rsi
	je	.L1122
	cmpb	$0, (%rsi)
	jne	.L1126
	.p2align 4,,10
	.p2align 3
.L1122:
	testl	%ebx, %ebx
	jle	.L1277
	movl	$.LC33, %edi
	leal	-1(%rbx), %r13d
	movl	%ebx, %r14d
	call	strdup
	movl	%r13d, %r10d
	subq	$32, %rsp
	leaq	(%r14,%r14,2), %r11
	movq	%rax, %rdx
	leaq	(%r10,%r10,2), %rax
	vmovdqa	aly_globals(%rip), %xmm0
	movq	%r10, -24736(%rbp)
	salq	$3, %rax
	movq	%r11, -24704(%rbp)
	movl	$.LC31, %esi
	leaq	-24624(%rbp,%r11,8), %rdi
	leaq	-24624(%rbp,%rax), %r9
	movq	%rax, -24712(%rbp)
	movl	$3, (%r9)
	movq	%r9, -24720(%rbp)
	movq	%rdx, -24616(%rbp,%rax)
	movq	$0, -24608(%rbp,%rax)
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rdx
	movq	%rdx, 16(%rsp)
	movq	%rdi, -24696(%rbp)
	call	aly_object_get
	movq	-24704(%rbp), %r11
	subq	$16, %rsp
	movq	-24712(%rbp), %rax
	vmovdqu	-24624(%rbp,%r11,8), %xmm0
	leaq	-24624(%rbp,%rax), %rdx
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24696(%rbp), %rdi
	movq	16(%rdi), %rsi
	leaq	-24688(%rbp), %rdi
	movq	%rsi, 40(%rsp)
	vmovdqu	-24624(%rbp,%rax), %xmm0
	movq	%rax, -24704(%rbp)
	vmovdqu	%xmm0, (%rsp)
	movq	16(%rdx), %rsi
	movq	%rdx, -24696(%rbp)
	movq	%rsi, 16(%rsp)
	call	aly_add
	movq	-24704(%rbp), %rax
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	movq	-24696(%rbp), %rdx
	movq	-24720(%rbp), %r9
	vmovdqu	%xmm0, -24624(%rbp,%rax)
	movq	-24672(%rbp), %rax
	movq	-24736(%rbp), %r10
	movq	%rax, 16(%rdx)
.L1131:
	cmpl	$3, (%r9)
	movq	%r9, -24704(%rbp)
	je	.L1278
	leaq	(%r10,%r10,2), %rax
	subq	$32, %rsp
	movq	%r10, -24696(%rbp)
	leaq	-24656(%rbp), %rdi
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	-24608(%rbp,%rax,8), %rax
	movq	%rax, 16(%rsp)
	call	aly_to_str
	movq	-24648(%rbp), %rdi
	addq	$32, %rsp
	call	puts
	vmovdqa	-24656(%rbp), %xmm0
	subq	$32, %rsp
	vmovdqu	%xmm0, (%rsp)
	movq	-24640(%rbp), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	movq	-24704(%rbp), %r9
	movq	-24696(%rbp), %r10
	addq	$32, %rsp
.L1134:
	leaq	(%r10,%r10,2), %r11
	movl	$.LC34, %edi
	movq	%r10, -24736(%rbp)
	movq	%r9, -24704(%rbp)
	movq	$0, -24616(%rbp,%r11,8)
	movq	%r11, -24696(%rbp)
	call	strdup
	movq	-24704(%rbp), %r9
	movq	-24696(%rbp), %r11
	movq	aly_globals+8(%rip), %rdx
	movl	aly_globals(%rip), %esi
	movl	$3, (%r9)
	movq	%rax, -24616(%rbp,%r11,8)
	leaq	(%r14,%r14,2), %rax
	leaq	-24624(%rbp,%rax,8), %rdi
	movq	%r9, -24720(%rbp)
	movq	$0, -24608(%rbp,%r11,8)
	movq	%r11, -24712(%rbp)
	movq	%rax, -24704(%rbp)
	movq	%rdi, -24696(%rbp)
	call	aly_object_get.constprop.2.isra.0
	movq	-24704(%rbp), %rax
	subq	$48, %rsp
	movq	-24712(%rbp), %r11
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	leaq	-24624(%rbp,%r11,8), %r14
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24696(%rbp), %rdi
	movq	16(%rdi), %rax
	leaq	-24688(%rbp), %rdi
	movq	%rax, 40(%rsp)
	vmovdqu	-24624(%rbp,%r11,8), %xmm0
	movq	%r11, -24696(%rbp)
	vmovdqu	%xmm0, (%rsp)
	movq	16(%r14), %rax
	movq	%rax, 16(%rsp)
	call	aly_add
	movq	-24696(%rbp), %r11
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	movq	-24720(%rbp), %r9
	movq	-24736(%rbp), %r10
	vmovdqu	%xmm0, -24624(%rbp,%r11,8)
	movq	-24672(%rbp), %rax
	movq	%rax, 16(%r14)
.L1135:
	cmpl	$3, (%r9)
	movq	%r9, -24704(%rbp)
	je	.L1279
	leaq	(%r10,%r10,2), %rax
	subq	$32, %rsp
	movq	%r10, -24696(%rbp)
	leaq	-24656(%rbp), %rdi
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	-24608(%rbp,%rax,8), %rax
	movq	%rax, 16(%rsp)
	call	aly_to_str
	movq	-24648(%rbp), %rdi
	addq	$32, %rsp
	call	puts
	vmovdqa	-24656(%rbp), %xmm0
	subq	$32, %rsp
	vmovdqu	%xmm0, (%rsp)
	movq	-24640(%rbp), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	movq	-24704(%rbp), %r9
	movq	-24696(%rbp), %r10
	addq	$32, %rsp
.L1138:
	leaq	(%r10,%r10,2), %rax
	movl	$6, (%r9)
	movq	$0, -24616(%rbp,%rax,8)
	movq	$0, -24608(%rbp,%rax,8)
	movslq	%r13d, %rax
	jmp	.L1139
	.p2align 4,,10
	.p2align 3
.L1120:
	xorl	%esi, %esi
	vxorpd	%xmm6, %xmm6, %xmm6
	movl	$1, %edi
	vucomisd	-24648(%rbp), %xmm6
	setp	%sil
	cmovne	%edi, %esi
.L1123:
	testl	%esi, %esi
	je	.L1122
	testl	%ebx, %ebx
	jg	.L1129
.L1276:
	leaq	-24600(%rbp,%r13), %rax
	movq	%rax, -24696(%rbp)
	movl	%ebx, %eax
	addl	$2, %ebx
	jmp	.L1130
	.p2align 4,,10
	.p2align 3
.L1121:
	xorl	%esi, %esi
	cmpq	$0, -24648(%rbp)
	setne	%sil
	jmp	.L1123
	.p2align 4,,10
	.p2align 3
.L1119:
	movl	-24648(%rbp), %esi
	jmp	.L1123
	.p2align 4,,10
	.p2align 3
.L1277:
	movl	$.LC33, %edi
	movq	%r9, -24704(%rbp)
	call	strdup
	movq	-24704(%rbp), %r9
	subq	$32, %rsp
	vmovdqa	aly_globals(%rip), %xmm0
	movq	%rax, %rsi
	leaq	(%r15,%r15,2), %rax
	leaq	(%r14,%r14,2), %r15
	movl	$3, (%r9)
	leaq	-24624(%rbp,%r15,8), %rdi
	movq	%r9, -24736(%rbp)
	movq	%rsi, -24616(%rbp,%rax,8)
	movq	$0, -24608(%rbp,%rax,8)
	movq	%rax, -24712(%rbp)
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rsi
	movq	%rsi, 16(%rsp)
	movl	$.LC31, %esi
	movq	%rdi, -24704(%rbp)
	call	aly_object_get
	subq	$16, %rsp
	movq	-24712(%rbp), %rax
	vmovdqu	-24624(%rbp,%r15,8), %xmm0
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24704(%rbp), %rdi
	leaq	-24624(%rbp,%rax,8), %rsi
	movq	16(%rdi), %rdi
	movq	%rdi, 40(%rsp)
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%rsi), %rdi
	movq	%rsi, -24704(%rbp)
	movq	%rdi, 16(%rsp)
	leaq	-24688(%rbp), %rdi
	call	aly_add
	movq	-24712(%rbp), %rax
	movq	-24704(%rbp), %rsi
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	testl	%r12d, %r12d
	movq	-24736(%rbp), %r9
	vmovdqu	%xmm0, -24624(%rbp,%rax,8)
	movq	-24672(%rbp), %rax
	movq	%rax, 16(%rsi)
	leal	2(%rbx), %eax
	jg	.L1280
	movq	-24696(%rbp), %rbx
	xorl	%edx, %edx
	testl	%eax, %eax
	movl	$.LC34, %edi
	cmovle	%eax, %edx
	movl	$6, (%rbx)
	movl	%edx, %r13d
	leal	1(%rdx), %ebx
	movq	$0, -24616(%rbp,%r15,8)
	movq	$0, -24608(%rbp,%r15,8)
	movslq	%ebx, %r15
	call	strdup
	leaq	(%r15,%r15,2), %r14
	movl	aly_globals(%rip), %esi
	movq	%rax, %rdx
	movslq	%r13d, %rax
	leaq	-24624(%rbp,%r14,8), %rdi
	addl	$2, %r13d
	leaq	(%rax,%rax,2), %r12
	movq	%rdi, -24696(%rbp)
	salq	$3, %r12
	leaq	-24624(%rbp,%r12), %r9
	movl	$3, (%r9)
	movq	%rdx, -24616(%rbp,%r12)
	movq	aly_globals+8(%rip), %rdx
	movq	%r9, -24704(%rbp)
	movq	$0, -24608(%rbp,%r12)
	call	aly_object_get.constprop.2.isra.0
	subq	$48, %rsp
	vmovdqu	-24624(%rbp,%r14,8), %xmm0
	leaq	-24624(%rbp,%r12), %rax
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24696(%rbp), %rdi
	movq	16(%rdi), %rdx
	leaq	-24688(%rbp), %rdi
	movq	%rdx, 40(%rsp)
	vmovdqu	-24624(%rbp,%r12), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%rax), %rdx
	movq	%rax, -24696(%rbp)
	movq	%rdx, 16(%rsp)
	call	aly_add
	movq	-24696(%rbp), %rax
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	cmpl	$1, %ebx
	movq	-24704(%rbp), %r9
	vmovdqu	%xmm0, -24624(%rbp,%r12)
	movq	-24672(%rbp), %rdx
	movq	%rdx, 16(%rax)
	je	.L1281
	movl	$6, -24600(%rbp,%r12)
	xorl	%eax, %eax
	testl	%r13d, %r13d
	movq	$0, -24616(%rbp,%r14,8)
	cmovg	%eax, %r13d
	movq	$0, -24608(%rbp,%r14,8)
	movslq	%r13d, %rax
	leal	1(%r13), %ebx
	leal	2(%r13), %r12d
	leaq	(%rax,%rax,2), %rdx
	movslq	%ebx, %r15
	leaq	-24624(%rbp,%rdx,8), %r9
.L1139:
	leaq	(%rax,%rax,2), %r13
	subq	$48, %rsp
	movl	$0, (%r9)
	movl	$.LC35, %edi
	leaq	-24624(%rbp,%r13,8), %r14
	movq	$0, -24616(%rbp,%r13,8)
	vmovdqu	-24624(%rbp,%r13,8), %xmm0
	movq	%r9, -24696(%rbp)
	movq	$0, -24608(%rbp,%r13,8)
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r14), %rax
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	movq	-24696(%rbp), %r9
	movl	$.LC36, %edi
	movl	$0, (%r9)
	movq	$0, -24616(%rbp,%r13,8)
	vmovdqu	-24624(%rbp,%r13,8), %xmm0
	movq	$0, -24608(%rbp,%r13,8)
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r14), %rax
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	movq	-24696(%rbp), %r9
	movl	$.LC36, %edi
	movl	$0, (%r9)
	movq	$1, -24616(%rbp,%r13,8)
	vmovdqu	-24624(%rbp,%r13,8), %xmm0
	movq	$0, -24608(%rbp,%r13,8)
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r14), %rax
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	addq	$48, %rsp
	.p2align 4,,10
	.p2align 3
.L1140:
	leaq	(%r15,%r15,2), %r11
	movq	aly_globals+8(%rip), %rdx
	movl	aly_globals(%rip), %esi
	leaq	-24624(%rbp,%r11,8), %r14
	movq	%r11, -24704(%rbp)
	movq	%r14, %rdi
	call	aly_object_get.constprop.1.isra.0
	movslq	%r12d, %r8
	subq	$48, %rsp
	movl	$.LC17, %esi
	leaq	(%r8,%r8,2), %rax
	movq	%r8, -24712(%rbp)
	leaq	-24688(%rbp), %rdi
	leaq	0(,%rax,8), %r13
	leaq	-24624(%rbp,%r13), %rax
	movl	$0, (%rax)
	movq	$5, -24616(%rbp,%r13)
	vmovdqu	-24624(%rbp,%r13), %xmm0
	movq	$0, -24608(%rbp,%r13)
	movq	%rax, -24696(%rbp)
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp,%r13), %rdx
	movq	%rdx, 40(%rsp)
	movq	-24704(%rbp), %r11
	vmovdqu	-24624(%rbp,%r11,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%r14), %rdx
	movq	%rdx, 16(%rsp)
	call	aly_compare
	movq	-24704(%rbp), %r11
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	movq	-24712(%rbp), %r8
	vmovdqu	%xmm0, -24624(%rbp,%r11,8)
	movq	-24672(%rbp), %rdx
	vmovdqa	%xmm0, -24656(%rbp)
	movq	%rdx, 16(%r14)
	movq	%rdx, -24640(%rbp)
	leaq	-24648(%rbp,%r13), %rdx
	cmpl	$5, (%rdx)
	ja	.L1153
	movl	(%rdx), %esi
	jmp	*.L1143(,%rsi,8)
	.section	.rodata
	.align 8
	.align 4
.L1143:
	.quad	.L1148
	.quad	.L1147
	.quad	.L1146
	.quad	.L1145
	.quad	.L1142
	.quad	.L1142
	.text
	.p2align 4,,10
	.p2align 3
.L1142:
	movq	-24648(%rbp), %rsi
	testq	%rsi, %rsi
	je	.L1153
	movl	8(%rsi), %ecx
	testl	%ecx, %ecx
	jle	.L1153
.L1152:
	testl	%ebx, %ebx
	jle	.L1282
.L1156:
	leal	-1(%rbx), %ecx
	movslq	%ebx, %r8
	movl	%r12d, %ebx
.L1157:
	vmovdqa	aly_globals(%rip), %xmm0
	movslq	%ecx, %rax
	subq	$32, %rsp
	movl	%ecx, -24704(%rbp)
	leaq	(%rax,%rax,2), %r15
	movq	%r8, -24712(%rbp)
	movl	$.LC35, %esi
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	leaq	-24624(%rbp,%r15,8), %r12
	movq	%r12, %rdi
	movq	%rax, 16(%rsp)
	call	aly_object_get
	movq	-24712(%rbp), %r8
	movl	aly_globals(%rip), %esi
	addq	$32, %rsp
	movq	aly_globals+8(%rip), %rdx
	leaq	(%r8,%r8,2), %r13
	leaq	-24624(%rbp,%r13,8), %r14
	movq	%r14, %rdi
	call	aly_object_get.constprop.1.isra.0
	subq	$48, %rsp
	leaq	-24688(%rbp), %rdi
	vmovdqu	-24624(%rbp,%r13,8), %xmm0
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r14), %rdx
	movq	%rdx, 40(%rsp)
	vmovdqu	-24624(%rbp,%r15,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%r12), %rdx
	movq	%rdx, 16(%rsp)
	call	aly_add
	vmovdqu	-24688(%rbp), %xmm0
	movl	$.LC35, %edi
	vmovdqu	%xmm0, -24624(%rbp,%r15,8)
	movslq	%ebx, %r15
	movq	-24672(%rbp), %rdx
	movq	%rdx, 16(%r12)
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r12), %rdx
	leaq	(%r15,%r15,2), %r12
	movq	%rdx, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rdx
	movq	%rdx, 16(%rsp)
	call	aly_object_set
	movl	aly_globals(%rip), %esi
	addq	$48, %rsp
	movq	%r14, %rdi
	movq	aly_globals+8(%rip), %rdx
	call	aly_object_get.constprop.1.isra.0
	movq	-24696(%rbp), %rax
	subq	$48, %rsp
	leaq	-24688(%rbp), %rdi
	movl	$0, (%rax)
	movq	$1, -24616(%rbp,%r12,8)
	vmovdqu	-24624(%rbp,%r12,8), %xmm0
	movq	$0, -24608(%rbp,%r12,8)
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp,%r12,8), %rdx
	movq	%rdx, 40(%rsp)
	vmovdqu	-24624(%rbp,%r13,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%r14), %rdx
	movq	%rdx, 16(%rsp)
	call	aly_add
	vmovdqu	-24688(%rbp), %xmm0
	movl	$.LC36, %edi
	vmovdqu	%xmm0, -24624(%rbp,%r13,8)
	movq	-24672(%rbp), %rdx
	movq	%rdx, 16(%r14)
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r14), %rax
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	movl	-24704(%rbp), %ecx
	addq	$48, %rsp
	leal	3(%rcx), %eax
	movq	-24696(%rbp), %rcx
	testl	%eax, %eax
	cmovle	%eax, %ebx
	movl	$6, (%rcx)
	movq	$0, -24616(%rbp,%r12,8)
	movq	$0, -24608(%rbp,%r12,8)
	movslq	%ebx, %rax
	leal	1(%rbx), %r12d
	cmovle	%rax, %r15
	jmp	.L1140
	.p2align 4,,10
	.p2align 3
.L1145:
	movq	-24648(%rbp), %rsi
	testq	%rsi, %rsi
	je	.L1153
	cmpb	$0, (%rsi)
	jne	.L1152
	.p2align 4,,10
	.p2align 3
.L1153:
	testl	%ebx, %ebx
	jle	.L1283
	leal	-1(%rbx), %r13d
	movl	$.LC37, %edi
	call	strdup
	leaq	0(%r13,%r13,2), %r12
	movl	%ebx, %ecx
	subq	$32, %rsp
	salq	$3, %r12
	vmovdqa	aly_globals(%rip), %xmm0
	leaq	(%rcx,%rcx,2), %rcx
	movq	%r13, %r15
	leaq	-24624(%rbp,%r12), %rdx
	movq	%rcx, -24696(%rbp)
	leaq	-24624(%rbp,%rcx,8), %rbx
	movl	$3, (%rdx)
	movq	%rbx, %rdi
	movq	%rdx, -24704(%rbp)
	movq	%rax, -24616(%rbp,%r12)
	movq	$0, -24608(%rbp,%r12)
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rsi
	movq	%rsi, 16(%rsp)
	movl	$.LC35, %esi
	call	aly_object_get
	movq	-24696(%rbp), %rcx
	subq	$16, %rsp
	leaq	-24688(%rbp), %rdi
	vmovdqu	-24624(%rbp,%rcx,8), %xmm0
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%rbx), %rax
	movq	%rax, 40(%rsp)
	vmovdqu	-24624(%rbp,%r12), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	-24608(%rbp,%r12), %rax
	movq	%rax, 16(%rsp)
	call	aly_add
	movq	-24704(%rbp), %rdx
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	vmovdqu	%xmm0, -24624(%rbp,%r12)
	movq	-24672(%rbp), %rax
	movq	%rax, -24608(%rbp,%r12)
.L1159:
	cmpl	$3, (%rdx)
	movq	%rdx, -24696(%rbp)
	je	.L1284
	leaq	0(%r13,%r13,2), %rax
	subq	$32, %rsp
	leaq	-24656(%rbp), %rdi
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	-24608(%rbp,%rax,8), %rax
	movq	%rax, 16(%rsp)
	call	aly_to_str
	movq	-24648(%rbp), %rdi
	addq	$32, %rsp
	call	puts
	vmovdqa	-24656(%rbp), %xmm0
	subq	$32, %rsp
	vmovdqu	%xmm0, (%rsp)
	movq	-24640(%rbp), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	movq	-24696(%rbp), %rdx
	addq	$32, %rsp
.L1162:
	leaq	0(%r13,%r13,2), %rax
	movl	$6, (%rdx)
	movslq	%r15d, %rbx
	movq	$0, -24616(%rbp,%rax,8)
	movq	$0, -24608(%rbp,%rax,8)
	jmp	.L1163
	.p2align 4,,10
	.p2align 3
.L1147:
	xorl	%esi, %esi
	vxorpd	%xmm7, %xmm7, %xmm7
	movl	$1, %edi
	vucomisd	-24648(%rbp), %xmm7
	setp	%sil
	cmovne	%edi, %esi
.L1149:
	testl	%esi, %esi
	je	.L1153
	testl	%ebx, %ebx
	jg	.L1156
.L1282:
	leaq	-24600(%rbp,%r13), %rax
	movl	%ebx, %ecx
	addl	$2, %ebx
	movq	%rax, -24696(%rbp)
	jmp	.L1157
	.p2align 4,,10
	.p2align 3
.L1148:
	xorl	%esi, %esi
	cmpq	$0, -24648(%rbp)
	setne	%sil
	jmp	.L1149
	.p2align 4,,10
	.p2align 3
.L1146:
	movl	-24648(%rbp), %esi
	jmp	.L1149
	.p2align 4,,10
	.p2align 3
.L1283:
	movl	$.LC37, %edi
	movq	%r8, -24720(%rbp)
	leaq	(%r15,%r15,2), %r14
	movq	%rdx, -24704(%rbp)
	call	strdup
	movq	-24704(%rbp), %rdx
	movq	-24720(%rbp), %r8
	subq	$32, %rsp
	vmovdqa	aly_globals(%rip), %xmm0
	movl	$3, (%rdx)
	leaq	(%r8,%r8,2), %r15
	movq	%rdx, -24712(%rbp)
	leaq	-24624(%rbp,%r15,8), %rdi
	movq	%rax, -24616(%rbp,%r14,8)
	movq	$0, -24608(%rbp,%r14,8)
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rsi
	movq	%rsi, 16(%rsp)
	movl	$.LC35, %esi
	movq	%rdi, -24704(%rbp)
	call	aly_object_get
	subq	$16, %rsp
	vmovdqu	-24624(%rbp,%r15,8), %xmm0
	leaq	-24624(%rbp,%r14,8), %rsi
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24704(%rbp), %rdi
	movq	16(%rdi), %rdi
	movq	%rdi, 40(%rsp)
	vmovdqu	-24624(%rbp,%r14,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%rsi), %rdi
	movq	%rsi, -24704(%rbp)
	movq	%rdi, 16(%rsp)
	leaq	-24688(%rbp), %rdi
	call	aly_add
	movq	-24704(%rbp), %rsi
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	testl	%r12d, %r12d
	movq	-24712(%rbp), %rdx
	vmovdqu	%xmm0, -24624(%rbp,%r14,8)
	movq	-24672(%rbp), %rdi
	movq	%rdi, 16(%rsi)
	jg	.L1285
	movq	-24696(%rbp), %rax
	movl	$6, (%rax)
	xorl	%eax, %eax
	movq	$0, -24616(%rbp,%r15,8)
	movq	$0, -24608(%rbp,%r15,8)
	leal	2(%rbx), %r15d
	testl	%r15d, %r15d
	cmovg	%eax, %r15d
	movslq	%r15d, %rbx
	leaq	(%rbx,%rbx,2), %rax
	leaq	-24624(%rbp,%rax,8), %rdx
.L1163:
	leaq	(%rbx,%rbx,2), %r12
	subq	$48, %rsp
	movl	$0, (%rdx)
	movl	$.LC38, %edi
	movq	$0, -24616(%rbp,%r12,8)
	vmovdqu	-24624(%rbp,%r12,8), %xmm0
	movq	%rdx, -24696(%rbp)
	movq	$0, -24608(%rbp,%r12,8)
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp,%r12,8), %rax
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	movq	-24696(%rbp), %rdx
	movl	$.LC39, %edi
	movl	$0, (%rdx)
	movq	$0, -24616(%rbp,%r12,8)
	vmovdqu	-24624(%rbp,%r12,8), %xmm0
	movq	$0, -24608(%rbp,%r12,8)
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp,%r12,8), %rax
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	addq	$48, %rsp
	.p2align 4,,10
	.p2align 3
.L1164:
	leaq	(%rbx,%rbx,2), %r11
	movq	aly_globals+8(%rip), %rdx
	movl	aly_globals(%rip), %esi
	leal	1(%r15), %r13d
	leaq	-24624(%rbp,%r11,8), %r14
	movslq	%r13d, %r12
	movq	%r11, -24704(%rbp)
	movq	%r14, %rdi
	call	aly_object_get.constprop.0.isra.0
	leaq	(%r12,%r12,2), %rax
	subq	$48, %rsp
	movl	$.LC32, %esi
	salq	$3, %rax
	leaq	-24688(%rbp), %rdi
	leaq	-24624(%rbp,%rax), %rcx
	movl	$0, (%rcx)
	movq	$5, -24616(%rbp,%rax)
	vmovdqu	-24624(%rbp,%rax), %xmm0
	movq	$0, -24608(%rbp,%rax)
	movq	%rcx, -24696(%rbp)
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp,%rax), %rdx
	movq	%rax, -24712(%rbp)
	movq	%rdx, 40(%rsp)
	movq	-24704(%rbp), %r11
	vmovdqu	-24624(%rbp,%r11,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%r14), %rdx
	movq	%rdx, 16(%rsp)
	call	aly_compare
	movq	-24704(%rbp), %r11
	movq	-24712(%rbp), %rax
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	vmovdqu	%xmm0, -24624(%rbp,%r11,8)
	movq	-24672(%rbp), %rdx
	vmovdqa	%xmm0, -24656(%rbp)
	movq	%rdx, 16(%r14)
	leaq	-24648(%rbp,%rax), %r14
	cmpl	$5, (%r14)
	movq	%rdx, -24640(%rbp)
	ja	.L1177
	movl	(%r14), %edx
	jmp	*.L1167(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L1167:
	.quad	.L1172
	.quad	.L1171
	.quad	.L1170
	.quad	.L1169
	.quad	.L1166
	.quad	.L1166
	.text
	.p2align 4,,10
	.p2align 3
.L1166:
	movq	-24648(%rbp), %rdx
	testq	%rdx, %rdx
	je	.L1177
	movl	8(%rdx), %edx
	testl	%edx, %edx
	jle	.L1177
	.p2align 4,,10
	.p2align 3
.L1176:
	testl	%r15d, %r15d
	leal	-1(%r15), %edx
	movq	-24696(%rbp), %rcx
	leaq	-24672(%rbp,%rax), %rax
	cmovg	%r15d, %r13d
	cmovg	%edx, %r15d
	movl	aly_globals(%rip), %esi
	movq	aly_globals+8(%rip), %rdx
	cmovg	%r14, %rcx
	cmovg	%rax, %r14
	movslq	%r15d, %rax
	cmovg	%rax, %rbx
	movslq	%r13d, %rax
	movq	%rcx, -24696(%rbp)
	cmovg	%rax, %r12
	leaq	(%rbx,%rbx,2), %rax
	leaq	-24624(%rbp,%rax,8), %rdi
	movq	%rax, -24704(%rbp)
	call	aly_object_get.constprop.0.isra.0
	movq	-24696(%rbp), %rcx
	leaq	(%r12,%r12,2), %rdx
	movq	-24704(%rbp), %rax
	movl	$0, (%rcx)
	movq	$2, -24616(%rbp,%rdx,8)
	movq	-24616(%rbp,%rax,8), %rax
	movq	$0, -24608(%rbp,%rdx,8)
	movl	(%rcx), %edi
	movl	(%r14), %edx
	movl	%edi, %esi
	orl	%edx, %esi
	jne	.L1181
	movq	%rax, %rdi
	shrq	$63, %rdi
	leaq	(%rax,%rdi), %rdx
	andl	$1, %edx
	subq	%rdi, %rdx
.L1182:
	leaq	(%rbx,%rbx,2), %rax
	movq	-24696(%rbp), %rcx
	movl	%esi, (%r14)
	subq	$48, %rsp
	movq	%rdx, -24616(%rbp,%rax,8)
	leaq	(%r12,%r12,2), %rdx
	leaq	-24624(%rbp,%rax,8), %r11
	movl	$.LC15, %esi
	movq	$0, -24608(%rbp,%rax,8)
	leaq	-24688(%rbp), %rdi
	movl	$0, (%rcx)
	movq	$0, -24616(%rbp,%rdx,8)
	vmovdqu	-24624(%rbp,%rdx,8), %xmm0
	movq	$0, -24608(%rbp,%rdx,8)
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp,%rdx,8), %rdx
	movq	%rdx, 40(%rsp)
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	movq	%rax, -24712(%rbp)
	vmovdqu	%xmm0, (%rsp)
	movq	16(%r11), %rdx
	movq	%r11, -24704(%rbp)
	movq	%rdx, 16(%rsp)
	call	aly_compare
	movq	-24712(%rbp), %rax
	movq	-24704(%rbp), %r11
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	vmovdqu	%xmm0, -24624(%rbp,%rax,8)
	movq	-24672(%rbp), %rax
	vmovdqa	%xmm0, -24656(%rbp)
	movq	%rax, 16(%r11)
	cmpl	$5, (%r14)
	movq	%rax, -24640(%rbp)
	ja	.L1202
	movl	(%r14), %eax
	jmp	*.L1192(,%rax,8)
	.section	.rodata
	.align 8
	.align 4
.L1192:
	.quad	.L1197
	.quad	.L1196
	.quad	.L1195
	.quad	.L1194
	.quad	.L1191
	.quad	.L1191
	.text
	.p2align 4,,10
	.p2align 3
.L1191:
	movq	-24648(%rbp), %rax
	testq	%rax, %rax
	je	.L1202
	movl	8(%rax), %eax
	testl	%eax, %eax
	jle	.L1202
.L1201:
	testl	%r15d, %r15d
	jle	.L1286
.L1205:
	leal	-1(%r15), %eax
	movslq	%r15d, %r12
	movl	%r13d, %r15d
.L1206:
	vmovdqa	aly_globals(%rip), %xmm0
	subq	$32, %rsp
	cltq
	movl	$.LC39, %esi
	leaq	(%rax,%rax,2), %rbx
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	leaq	-24624(%rbp,%rbx,8), %r13
	movq	%r13, %rdi
	movq	%rax, 16(%rsp)
	call	aly_object_get
	leaq	(%r12,%r12,2), %rax
	subq	$16, %rsp
	movl	$0, (%r14)
	movq	$1, -24616(%rbp,%rax,8)
	leaq	-24688(%rbp), %rdi
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	movq	$0, -24608(%rbp,%rax,8)
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp,%rax,8), %rax
	movq	%rax, 40(%rsp)
	vmovdqu	-24624(%rbp,%rbx,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%r13), %rax
	movq	%rax, 16(%rsp)
	call	aly_add
	vmovdqu	-24688(%rbp), %xmm0
	movl	$.LC39, %edi
	vmovdqu	%xmm0, -24624(%rbp,%rbx,8)
	movq	-24672(%rbp), %rax
	movslq	%r15d, %rbx
	movq	%rax, 16(%r13)
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r13), %rax
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	addq	$48, %rsp
.L1204:
	leaq	(%r12,%r12,2), %r12
	movq	aly_globals+8(%rip), %rdx
	movl	aly_globals(%rip), %esi
	leaq	-24624(%rbp,%r12,8), %r13
	movq	%r13, %rdi
	call	aly_object_get.constprop.0.isra.0
	movq	-24696(%rbp), %rax
	subq	$48, %rsp
	leaq	-24688(%rbp), %rdi
	movl	$0, (%rax)
	leaq	(%rbx,%rbx,2), %rax
	movq	$1, -24616(%rbp,%rax,8)
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	movq	$0, -24608(%rbp,%rax,8)
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24608(%rbp,%rax,8), %rax
	movq	%rax, 40(%rsp)
	vmovdqu	-24624(%rbp,%r12,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%r13), %rax
	movq	%rax, 16(%rsp)
	call	aly_add
	vmovdqu	-24688(%rbp), %xmm0
	movl	$.LC38, %edi
	vmovdqu	%xmm0, -24624(%rbp,%r12,8)
	movq	-24672(%rbp), %rax
	movq	%rax, 16(%r13)
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r13), %rax
	movq	%rax, 40(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	addq	$48, %rsp
	jmp	.L1164
	.p2align 4,,10
	.p2align 3
.L1169:
	movq	-24648(%rbp), %rdx
	testq	%rdx, %rdx
	je	.L1177
	cmpb	$0, (%rdx)
	jne	.L1176
	.p2align 4,,10
	.p2align 3
.L1177:
	testl	%r15d, %r15d
	jle	.L1287
	leal	-1(%r15), %r12d
	movl	$.LC40, %edi
	call	strdup
	leaq	(%r12,%r12,2), %r13
	movl	%r15d, %r8d
	subq	$32, %rsp
	salq	$3, %r13
	vmovdqa	aly_globals(%rip), %xmm0
	movl	$.LC39, %esi
	leaq	-24624(%rbp,%r13), %r14
	movl	$3, (%r14)
	movq	%rax, -24616(%rbp,%r13)
	leaq	(%r8,%r8,2), %rax
	movq	%rax, -24696(%rbp)
	leaq	-24624(%rbp,%rax,8), %r15
	movq	$0, -24608(%rbp,%r13)
	movq	%r15, %rdi
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rdx
	movq	%rdx, 16(%rsp)
	call	aly_object_get
	movq	-24696(%rbp), %rax
	subq	$16, %rsp
	leaq	-24688(%rbp), %rdi
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	vmovdqu	%xmm0, 24(%rsp)
	movq	16(%r15), %rax
	movq	%rax, 40(%rsp)
	vmovdqu	-24624(%rbp,%r13), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	-24608(%rbp,%r13), %rax
	movq	%rax, 16(%rsp)
	call	aly_add
	vmovdqu	-24688(%rbp), %xmm0
	addq	$48, %rsp
	vmovdqu	%xmm0, -24624(%rbp,%r13)
	movq	-24672(%rbp), %rax
	movq	%rax, -24608(%rbp,%r13)
.L1207:
	cmpl	$3, (%r14)
	je	.L1288
	leaq	(%r12,%r12,2), %rax
	subq	$32, %rsp
	leaq	-24656(%rbp), %rdi
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	-24608(%rbp,%rax,8), %rax
	movq	%rax, 16(%rsp)
	call	aly_to_str
	movq	-24648(%rbp), %rdi
	addq	$32, %rsp
	call	puts
	vmovdqa	-24656(%rbp), %xmm0
	subq	$32, %rsp
	vmovdqu	%xmm0, (%rsp)
	movq	-24640(%rbp), %rax
	movq	%rax, 16(%rsp)
	call	aly_free
	addq	$32, %rsp
.L1210:
	leaq	(%r12,%r12,2), %rax
	movl	$6, (%r14)
	movq	$0, -24616(%rbp,%rax,8)
	movq	$0, -24608(%rbp,%rax,8)
.L1211:
	leaq	(%r12,%r12,2), %rax
	movl	$6, (%r14)
	movq	$0, -24616(%rbp,%rax,8)
	movq	$0, -24608(%rbp,%rax,8)
.L1213:
	leaq	(%r12,%r12,2), %rax
	movq	-24728(%rbp), %rbx
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	movq	-24608(%rbp,%rax,8), %rax
	vmovdqu	%xmm0, (%rbx)
	movq	%rax, 16(%rbx)
	movq	%rbx, %rax
	jmp	.L1112
	.p2align 4,,10
	.p2align 3
.L1171:
	xorl	%edx, %edx
	vxorpd	%xmm1, %xmm1, %xmm1
	movl	$1, %ecx
	vucomisd	-24648(%rbp), %xmm1
	setp	%dl
	cmovne	%ecx, %edx
.L1173:
	testl	%edx, %edx
	jne	.L1176
	jmp	.L1177
	.p2align 4,,10
	.p2align 3
.L1170:
	movl	-24648(%rbp), %edx
	jmp	.L1173
	.p2align 4,,10
	.p2align 3
.L1172:
	xorl	%edx, %edx
	cmpq	$0, -24648(%rbp)
	setne	%dl
	jmp	.L1173
	.p2align 4,,10
	.p2align 3
.L1194:
	movq	-24648(%rbp), %rax
	testq	%rax, %rax
	je	.L1202
	cmpb	$0, (%rax)
	jne	.L1201
	.p2align 4,,10
	.p2align 3
.L1202:
	testl	%r15d, %r15d
	jle	.L1289
	leal	-1(%r15), %r13d
	movq	%r14, -24696(%rbp)
	movslq	%r13d, %r12
	jmp	.L1204
	.p2align 4,,10
	.p2align 3
.L1195:
	movl	-24648(%rbp), %eax
.L1198:
	testl	%eax, %eax
	je	.L1202
	testl	%r15d, %r15d
	jg	.L1205
.L1286:
	leal	2(%r15), %edx
	movq	-24696(%rbp), %r14
	movslq	%edx, %rax
	leaq	(%rax,%rax,2), %rax
	leaq	-24624(%rbp,%rax,8), %rax
	movq	%rax, -24696(%rbp)
	movl	%r15d, %eax
	movl	%edx, %r15d
	jmp	.L1206
	.p2align 4,,10
	.p2align 3
.L1196:
	xorl	%eax, %eax
	vxorpd	%xmm6, %xmm6, %xmm6
	movl	$1, %ecx
	vucomisd	-24648(%rbp), %xmm6
	setp	%al
	cmovne	%ecx, %eax
	jmp	.L1198
	.p2align 4,,10
	.p2align 3
.L1197:
	xorl	%eax, %eax
	cmpq	$0, -24648(%rbp)
	setne	%al
	jmp	.L1198
	.p2align 4,,10
	.p2align 3
.L1181:
	cmpl	$1, %edi
	je	.L1216
	cmpl	$3, %edi
	je	.L1184
	vmovsd	.LC28(%rip), %xmm3
	testl	%edi, %edi
	vmovsd	%xmm3, -24712(%rbp)
	je	.L1183
	movq	$0x000000000, -24712(%rbp)
.L1183:
	cmpl	$1, %edx
	je	.L1185
	cmpl	$3, %edx
	je	.L1186
	movq	$0x000000000, -24704(%rbp)
	testl	%edx, %edx
	jne	.L1187
	vxorpd	%xmm5, %xmm5, %xmm5
	vcvtsi2sdq	%rax, %xmm5, %xmm0
	vmovlpd	%xmm0, -24704(%rbp)
.L1187:
	fldl	-24712(%rbp)
	fldl	-24704(%rbp)
.L1188:
	fprem
	fnstsw	%ax
	sahf
	jp	.L1188
	fstp	%st(1)
	fstpl	-24720(%rbp)
	vmovsd	-24720(%rbp), %xmm2
	vucomisd	%xmm2, %xmm2
	jp	.L1290
.L1189:
	vmovq	%xmm2, %rdx
	movl	$1, %esi
	jmp	.L1182
	.p2align 4,,10
	.p2align 3
.L1289:
	movslq	%r13d, %rbx
	movslq	%r15d, %r12
	movq	%rbx, %r15
	jmp	.L1204
	.p2align 4,,10
	.p2align 3
.L1186:
	xorl	%esi, %esi
	movq	%rax, %rdi
	call	strtod
	vmovsd	%xmm0, -24704(%rbp)
	jmp	.L1187
	.p2align 4,,10
	.p2align 3
.L1185:
	movq	%rax, -24704(%rbp)
	jmp	.L1187
	.p2align 4,,10
	.p2align 3
.L1184:
	xorl	%esi, %esi
	movl	$2, %edi
	movl	%edx, -24720(%rbp)
	movq	%rax, -24704(%rbp)
	call	strtod
	movq	-24704(%rbp), %rax
	movl	-24720(%rbp), %edx
	vmovsd	%xmm0, -24712(%rbp)
	jmp	.L1183
	.p2align 4,,10
	.p2align 3
.L1216:
	vmovsd	.LC29(%rip), %xmm4
	vmovsd	%xmm4, -24712(%rbp)
	jmp	.L1183
	.p2align 4,,10
	.p2align 3
.L1287:
	movl	$.LC40, %edi
	leaq	(%rbx,%rbx,2), %rbx
	call	strdup
	subq	$32, %rsp
	movl	$.LC39, %esi
	vmovdqa	aly_globals(%rip), %xmm0
	movl	$3, (%r14)
	movq	%rax, -24616(%rbp,%rbx,8)
	leaq	(%r12,%r12,2), %rax
	movq	%rax, -24712(%rbp)
	leaq	-24624(%rbp,%rax,8), %rcx
	movq	$0, -24608(%rbp,%rbx,8)
	movq	%rcx, %rdi
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rdx
	movq	%rdx, 16(%rsp)
	movq	%rcx, -24704(%rbp)
	call	aly_object_get
	movq	-24712(%rbp), %rax
	subq	$16, %rsp
	leaq	-24624(%rbp,%rbx,8), %rdx
	leaq	-24688(%rbp), %rdi
	vmovdqu	-24624(%rbp,%rax,8), %xmm0
	vmovdqu	%xmm0, 24(%rsp)
	movq	-24704(%rbp), %rcx
	movq	16(%rcx), %rcx
	movq	%rcx, 40(%rsp)
	vmovdqu	-24624(%rbp,%rbx,8), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	16(%rdx), %rcx
	movq	%rdx, -24704(%rbp)
	movq	%rcx, 16(%rsp)
	call	aly_add
	movq	-24704(%rbp), %rdx
	addq	$48, %rsp
	vmovdqu	-24688(%rbp), %xmm0
	testl	%r13d, %r13d
	movq	-24712(%rbp), %rax
	vmovdqu	%xmm0, -24624(%rbp,%rbx,8)
	movq	-24672(%rbp), %rcx
	movq	%rcx, 16(%rdx)
	jg	.L1291
	movq	-24696(%rbp), %rbx
	cmpl	$-1, %r15d
	movl	$6, (%rbx)
	movq	$0, -24616(%rbp,%rax,8)
	movq	$0, -24608(%rbp,%rax,8)
	je	.L1292
	movl	%r15d, %r8d
	addl	$2, %r8d
	movslq	%r8d, %r8
	leaq	(%r8,%r8,2), %rax
	movl	$6, -24624(%rbp,%rax,8)
	movq	$0, -24616(%rbp,%rax,8)
	movq	$0, -24608(%rbp,%rax,8)
	je	.L1293
	movq	-24728(%rbp), %rax
	movl	$6, (%rax)
	movq	$0, 8(%rax)
	movq	$0, 16(%rax)
.L1112:
	leaq	-48(%rbp), %rsp
	popq	%rbx
	popq	%r10
	.cfi_remember_state
	.cfi_def_cfa 10, 0
	popq	%r12
	popq	%r13
	popq	%r14
	popq	%r15
	popq	%rbp
	leaq	-8(%r10), %rsp
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L1292:
	.cfi_restore_state
	leaq	-24624(%rbp), %r14
	xorl	%r12d, %r12d
	jmp	.L1211
	.p2align 4,,10
	.p2align 3
.L1288:
	leaq	(%r12,%r12,2), %rax
	movq	-24616(%rbp,%rax,8), %rdi
	call	puts
	jmp	.L1210
	.p2align 4,,10
	.p2align 3
.L1284:
	leaq	0(%r13,%r13,2), %rax
	movq	-24616(%rbp,%rax,8), %rdi
	call	puts
	movq	-24696(%rbp), %rdx
	jmp	.L1162
	.p2align 4,,10
	.p2align 3
.L1278:
	leaq	(%r10,%r10,2), %rax
	movq	%r10, -24696(%rbp)
	movq	-24616(%rbp,%rax,8), %rdi
	call	puts
	movq	-24696(%rbp), %r10
	movq	-24704(%rbp), %r9
	jmp	.L1134
	.p2align 4,,10
	.p2align 3
.L1279:
	leaq	(%r10,%r10,2), %rax
	movq	%r10, -24696(%rbp)
	movq	-24616(%rbp,%rax,8), %rdi
	call	puts
	movq	-24696(%rbp), %r10
	movq	-24704(%rbp), %r9
	jmp	.L1138
	.p2align 4,,10
	.p2align 3
.L1293:
	xorl	%r12d, %r12d
	jmp	.L1213
.L1290:
	vmovsd	-24712(%rbp), %xmm1
	vmovsd	-24704(%rbp), %xmm0
	call	fmod
	vmovsd	-24720(%rbp), %xmm2
	jmp	.L1189
.L1285:
	movl	%ebx, %r13d
	movq	%r13, %r15
	jmp	.L1159
.L1291:
	movl	%r15d, %r12d
	jmp	.L1207
.L1281:
	xorl	%r13d, %r13d
	movl	$2, %r12d
	xorl	%r10d, %r10d
	jmp	.L1135
.L1280:
	movslq	-24720(%rbp), %r15
	movl	%ebx, %r10d
	movl	%r12d, %ebx
	addl	$1, %r12d
	movq	%r10, %r13
	movl	%r15d, %r14d
	jmp	.L1131
	.cfi_endproc
.LFE110:
	.size	fn_0, .-fn_0
	.section	.rodata.str1.1
.LC41:
	.string	"print"
.LC42:
	.string	"input"
.LC43:
	.string	"tomb"
.LC44:
	.string	"len"
	.section	.text.startup,"ax",@progbits
	.p2align 4
	.globl	main
	.type	main, @function
main:
.LFB111:
	.cfi_startproc
	leaq	8(%rsp), %r10
	.cfi_def_cfa 10, 0
	andq	$-32, %rsp
	pushq	-8(%r10)
	pushq	%rbp
	movq	%rsp, %rbp
	.cfi_escape 0x10,0x6,0x2,0x76,0
	pushq	%r10
	.cfi_escape 0xf,0x3,0x76,0x78,0x6
	pushq	%rbx
	subq	$192, %rsp
	.cfi_escape 0x10,0x3,0x2,0x76,0x70
	movl	%edi, _aly_argc(%rip)
	xorl	%edi, %edi
	movq	%rsi, _aly_argv(%rip)
	call	time
	movl	%eax, %edi
	call	srand
	movl	$16, %esi
	movl	$1, %edi
	movl	$5, aly_globals(%rip)
	call	calloc
	movl	$40, %esi
	movl	$32, %edi
	movq	%rax, %rbx
	call	calloc
	subq	$48, %rsp
	movl	$.LC41, %edi
	movl	$7, -144(%rbp)
	movq	$native_print, -136(%rbp)
	vmovdqa	-144(%rbp), %xmm0
	movq	%rax, (%rbx)
	movabsq	$137438953472, %rax
	movq	%rax, 8(%rbx)
	movq	%rbx, aly_globals+8(%rip)
	movq	$0, aly_globals+16(%rip)
	movq	$0, 40(%rsp)
	vmovdqu	%xmm0, 24(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	movq	$0, 16(%rsp)
	vmovdqu	%xmm0, (%rsp)
	call	aly_object_set
	movq	$native_input, -104(%rbp)
	movl	$.LC42, %edi
	movl	$7, -112(%rbp)
	vmovdqa	-112(%rbp), %xmm0
	movq	$0, 40(%rsp)
	vmovdqu	%xmm0, 24(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	movq	$native_tomb, -72(%rbp)
	movl	$.LC43, %edi
	movl	$7, -80(%rbp)
	vmovdqa	-80(%rbp), %xmm0
	movq	$0, 40(%rsp)
	vmovdqu	%xmm0, 24(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	movq	$native_len, -40(%rbp)
	movl	$.LC44, %edi
	movl	$7, -48(%rbp)
	vmovdqa	-48(%rbp), %xmm0
	movq	$0, 40(%rsp)
	vmovdqu	%xmm0, 24(%rsp)
	vmovdqa	aly_globals(%rip), %xmm0
	vmovdqu	%xmm0, (%rsp)
	movq	aly_globals+16(%rip), %rax
	movq	%rax, 16(%rsp)
	call	aly_object_set
	leaq	-208(%rbp), %rdi
	addq	$48, %rsp
	xorl	%esi, %esi
	leaq	-176(%rbp), %rdx
	call	fn_0
	leaq	-16(%rbp), %rsp
	xorl	%eax, %eax
	popq	%rbx
	popq	%r10
	.cfi_def_cfa 10, 0
	popq	%rbp
	leaq	-8(%r10), %rsp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE111:
	.size	main, .-main
	.section	.rodata.str1.1
.LC45:
	.string	"int"
.LC46:
	.string	"float"
.LC47:
	.string	"bool"
.LC48:
	.string	"string"
.LC49:
	.string	"array"
.LC50:
	.string	"object"
.LC51:
	.string	"function"
	.section	.rodata
	.align 32
	.type	CSWTCH.102, @object
	.size	CSWTCH.102, 64
CSWTCH.102:
	.quad	.LC45
	.quad	.LC46
	.quad	.LC47
	.quad	.LC48
	.quad	.LC49
	.quad	.LC50
	.quad	.LC7
	.quad	.LC51
	.globl	aly_globals
	.bss
	.align 16
	.type	aly_globals, @object
	.size	aly_globals, 24
aly_globals:
	.zero	24
	.local	_aly_argv
	.comm	_aly_argv,8,8
	.local	_aly_argc
	.comm	_aly_argc,4,4
	.section	.rodata.cst16,"aM",@progbits,16
	.align 16
.LC13:
	.long	0
	.long	-2147483648
	.long	0
	.long	0
	.section	.rodata.cst8,"aM",@progbits,8
	.align 8
.LC14:
	.long	0
	.long	1079574528
	.section	.rodata.cst16
	.align 16
.LC21:
	.long	-1
	.long	2147483647
	.long	0
	.long	0
	.section	.rodata.cst8
	.align 8
.LC22:
	.long	-4194304
	.long	1105199103
	.align 8
.LC23:
	.long	0
	.long	1072693248
	.align 8
.LC28:
	.long	0
	.long	1073741824
	.align 8
.LC29:
	.long	2
	.long	0
	.ident	"GCC: (GNU) 16.1.1 20260515 (Red Hat 16.1.1-2)"
	.section	.note.GNU-stack,"",@progbits
