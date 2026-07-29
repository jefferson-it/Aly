	.file	"tomb_aly_tmp.c"
	.text
	.p2align 4
	.globl	native_tomb
	.type	native_tomb, @function
native_tomb:
.LFB110:
	.cfi_startproc
	testl	%esi, %esi
	movq	%rdi, %rax
	jg	.L16
	movl	$6, (%rdi)
	movq	$0, 8(%rdi)
	movq	$0, 16(%rdi)
	movl	$1, 24(%rdi)
	ret
	.p2align 4,,10
	.p2align 3
.L16:
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	vmovdqu	(%rdx), %ymm0
	cmpl	$8, (%rdx)
	movq	8(%rdx), %rcx
	vmovdqu	%ymm0, -32(%rsp)
	jne	.L3
	testq	%rcx, %rcx
	jne	.L17
.L3:
	vmovdqu	-32(%rsp), %ymm0
	vmovdqu	%ymm0, (%rax)
	vzeroupper
	popq	%rbp
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L17:
	.cfi_restore_state
	vmovdqu	-32(%rsp), %ymm0
	movl	$0, 24(%rcx)
	vmovdqu	%ymm0, (%rax)
	vzeroupper
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE110:
	.size	native_tomb, .-native_tomb
	.p2align 4
	.globl	native_len
	.type	native_len, @function
native_len:
.LFB111:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	testl	%esi, %esi
	movq	%rdi, %rcx
	jg	.L19
.L23:
	xorl	%eax, %eax
.L20:
	movq	%rax, 8(%rcx)
	movq	%rcx, %rax
	movl	$0, (%rcx)
	movq	$0, 16(%rcx)
	movl	$1, 24(%rcx)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L19:
	.cfi_restore_state
	movl	(%rdx), %eax
	movq	8(%rdx), %rdi
	cmpl	$4, %eax
	je	.L22
	cmpl	$5, %eax
	je	.L22
	cmpl	$3, %eax
	jne	.L23
	testq	%rdi, %rdi
	je	.L23
	movq	%rcx, 8(%rsp)
	call	strlen
	movq	8(%rsp), %rcx
	cltq
	jmp	.L20
	.p2align 4,,10
	.p2align 3
.L22:
	testq	%rdi, %rdi
	je	.L23
	movslq	8(%rdi), %rax
	jmp	.L20
	.cfi_endproc
.LFE111:
	.size	native_len, .-native_len
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
	movl	$1, 24(%rdi)
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
	movl	$1, 24(%rdi)
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
	movl	$1, 24(%rdi)
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
	movl	$1, 24(%rdi)
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
	movl	$1, 24(%rdi)
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
	movl	$1, 24(%rbx)
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
	movl	$1, 24(%rbx)
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE30:
	.size	aly_string_alloc, .-aly_string_alloc
	.p2align 4
	.globl	aly_ref_val
	.type	aly_ref_val, @function
aly_ref_val:
.LFB31:
	.cfi_startproc
	vmovq	%rsi, %xmm0
	movl	$8, (%rdi)
	movq	%rdi, %rax
	movl	$1, 24(%rdi)
	vmovdqu	%xmm0, 8(%rdi)
	ret
	.cfi_endproc
.LFE31:
	.size	aly_ref_val, .-aly_ref_val
	.p2align 4
	.globl	aly_free
	.type	aly_free, @function
aly_free:
.LFB32:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	movq	%rbx, -24(%rbp)
	movq	%r13, -8(%rbp)
	.cfi_offset 3, -40
	.cfi_offset 13, -24
	movq	32(%rbp), %rbx
	cmpl	$4, %eax
	movq	24(%rbp), %r13
	je	.L47
	cmpl	$5, %eax
	je	.L48
	cmpl	$3, %eax
	jne	.L49
.L67:
	movq	%r13, %rdi
	call	free
.L49:
	testq	%rbx, %rbx
	je	.L64
	movq	-8(%rbp), %r13
	movq	%rbx, %rdi
	movq	-24(%rbp), %rbx
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	jmp	free
	.p2align 4,,10
	.p2align 3
.L48:
	.cfi_restore_state
	testq	%r13, %r13
	je	.L49
	movl	12(%r13), %esi
	movq	0(%r13), %rdi
	testl	%esi, %esi
	jle	.L52
	movq	%r12, -16(%rbp)
	.cfi_offset 12, -32
	xorl	%r12d, %r12d
.L55:
	leaq	(%r12,%r12,2), %rdx
	salq	$4, %rdx
	leaq	(%rdi,%rdx), %rcx
	movl	40(%rcx), %eax
	testl	%eax, %eax
	jne	.L68
	addq	$1, %r12
	cmpl	%r12d, %esi
	jg	.L55
.L66:
	movq	-16(%rbp), %r12
	.cfi_restore 12
.L52:
	call	free
	jmp	.L67
	.p2align 4,,10
	.p2align 3
.L64:
	movq	-24(%rbp), %rbx
	movq	-8(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L47:
	.cfi_restore_state
	testq	%r13, %r13
	je	.L49
	movl	8(%r13), %edx
	testl	%edx, %edx
	jle	.L50
	movq	%r12, -16(%rbp)
	.cfi_offset 12, -32
	xorl	%r12d, %r12d
.L51:
	movq	%r12, %rax
	subq	$32, %rsp
	salq	$5, %rax
	addq	0(%r13), %rax
	vmovdqu	(%rax), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	addq	$1, %r12
	addq	$32, %rsp
	cmpl	%r12d, 8(%r13)
	jg	.L51
	movq	-16(%rbp), %r12
	.cfi_restore 12
.L50:
	movq	0(%r13), %rdi
	call	free
	movq	%r13, %rdi
	call	free
	jmp	.L49
	.p2align 4,,10
	.p2align 3
.L68:
	.cfi_offset 12, -32
	movq	(%rcx), %rdi
	movq	%rdx, -40(%rbp)
	call	free
	movq	0(%r13), %rcx
	movq	-40(%rbp), %rdx
	subq	$32, %rsp
	vmovdqu	8(%rcx,%rdx), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movl	12(%r13), %esi
	addq	$1, %r12
	addq	$32, %rsp
	movq	0(%r13), %rdi
	cmpl	%r12d, %esi
	jg	.L55
	jmp	.L66
	.cfi_endproc
.LFE32:
	.size	aly_free, .-aly_free
	.p2align 4
	.globl	aly_is_truthy
	.type	aly_is_truthy, @function
aly_is_truthy:
.LFB34:
	.cfi_startproc
	cmpl	$5, 8(%rsp)
	ja	.L70
	movl	8(%rsp), %eax
	jmp	*.L72(,%rax,8)
	.section	.rodata
	.align 8
	.align 4
.L72:
	.quad	.L77
	.quad	.L76
	.quad	.L75
	.quad	.L74
	.quad	.L71
	.quad	.L71
	.text
	.p2align 4,,10
	.p2align 3
.L71:
	movq	16(%rsp), %rax
	testq	%rax, %rax
	je	.L70
	movl	8(%rax), %eax
	testl	%eax, %eax
	setg	%al
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L74:
	movq	16(%rsp), %rax
	testq	%rax, %rax
	je	.L70
	cmpb	$0, (%rax)
	setne	%al
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L77:
	xorl	%eax, %eax
	cmpq	$0, 16(%rsp)
	setne	%al
	ret
	.p2align 4,,10
	.p2align 3
.L76:
	vxorpd	%xmm0, %xmm0, %xmm0
	xorl	%eax, %eax
	movl	$1, %edx
	vucomisd	16(%rsp), %xmm0
	setp	%al
	cmovne	%edx, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L75:
	movl	16(%rsp), %eax
	ret
	.p2align 4,,10
	.p2align 3
.L70:
	xorl	%eax, %eax
	ret
	.cfi_endproc
.LFE34:
	.size	aly_is_truthy, .-aly_is_truthy
	.section	.rodata.str1.1,"aMS",@progbits,1
.LC1:
	.string	"unknown"
	.text
	.p2align 4
	.globl	aly_type_name
	.type	aly_type_name, @function
aly_type_name:
.LFB35:
	.cfi_startproc
	movl	8(%rsp), %eax
	movl	$.LC1, %edx
	cmpl	$8, %eax
	ja	.L90
	movq	CSWTCH.89(,%rax,8), %rdx
.L90:
	movq	%rdx, %rax
	ret
	.cfi_endproc
.LFE35:
	.size	aly_type_name, .-aly_type_name
	.p2align 4
	.globl	aly_to_float
	.type	aly_to_float, @function
aly_to_float:
.LFB37:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rax
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	16(%rbp), %edx
	cmpl	$1, %edx
	je	.L94
	cmpl	$3, %edx
	movq	24(%rbp), %rdi
	je	.L95
	testl	%edx, %edx
	movl	$1, (%rax)
	jne	.L96
	vxorps	%xmm0, %xmm0, %xmm0
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vmovlpd	%xmm0, 8(%rax)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L95:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rax
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	vmovsd	%xmm0, 8(%rax)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L96:
	.cfi_restore_state
	movq	$0x000000000, 8(%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L94:
	.cfi_restore_state
	vmovdqu	16(%rbp), %ymm0
	vmovdqu	%ymm0, (%rdi)
	vzeroupper
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE37:
	.size	aly_to_float, .-aly_to_float
	.section	.rodata.str1.1
.LC2:
	.string	"true"
.LC3:
	.string	"false"
.LC4:
	.string	"%lld"
.LC5:
	.string	"%g"
.LC6:
	.string	"None"
.LC7:
	.string	"[]"
.LC8:
	.string	"{}"
	.text
	.p2align 4
	.globl	aly_to_str
	.type	aly_to_str, @function
aly_to_str:
.LFB38:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$576, %rsp
	movq	%r10, -24(%rbp)
	cmpl	$6, 16(%rbp)
	.cfi_offset 10, -40
	leaq	16(%rbp), %r10
	movq	%rbx, -32(%rbp)
	.cfi_offset 3, -48
	movq	%rdi, %rbx
	ja	.L100
	movl	(%r10), %eax
	jmp	*.L102(,%rax,8)
	.section	.rodata
	.align 8
	.align 4
.L102:
	.quad	.L108
	.quad	.L107
	.quad	.L106
	.quad	.L105
	.quad	.L104
	.quad	.L103
	.quad	.L100
	.text
	.p2align 4,,10
	.p2align 3
.L100:
	movl	$.LC6, %edi
	call	strdup
.L109:
	movq	%rax, 8(%rbx)
	movq	%rbx, %rax
	movl	$3, (%rbx)
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	movq	-32(%rbp), %rbx
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L103:
	.cfi_restore_state
	movq	8(%r10), %r8
	testq	%r8, %r8
	je	.L117
	movl	8(%r8), %r9d
	testl	%r9d, %r9d
	je	.L117
	movl	$1024, %esi
	movl	$1, %edi
	movq	%r14, -16(%rbp)
	movq	%r8, -552(%rbp)
	.cfi_offset 14, -32
	call	calloc
	movq	-552(%rbp), %r8
	movw	$123, (%rax)
	movq	%rax, %r14
	movl	12(%r8), %eax
	testl	%eax, %eax
	jle	.L119
	movq	%r15, -8(%rbp)
	xorl	%ecx, %ecx
	movl	$1, %esi
	.cfi_offset 15, -24
.L122:
	leaq	(%rcx,%rcx,2), %rdx
	salq	$4, %rdx
	addq	(%r8), %rdx
	movl	40(%rdx), %edi
	testl	%edi, %edi
	je	.L120
	testl	%esi, %esi
	je	.L132
.L121:
	movq	%r14, %rdi
	movq	%rcx, -568(%rbp)
	movq	%r8, -560(%rbp)
	movq	%rdx, -552(%rbp)
	call	strlen
	movq	-552(%rbp), %rdx
	leaq	(%r14,%rax), %rdi
	movq	(%rdx), %rsi
	call	stpcpy
	movq	-552(%rbp), %rdx
	subq	$32, %rsp
	leaq	-544(%rbp), %rdi
	movw	$8250, (%rax)
	movq	%rax, %r15
	vmovdqu	8(%rdx), %ymm0
	movb	$0, 2(%rax)
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	addq	$32, %rsp
	leaq	2(%r15), %rdi
	movq	-536(%rbp), %rsi
	call	strcpy
	vmovdqu	-544(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	addq	$32, %rsp
	xorl	%esi, %esi
	movq	-560(%rbp), %r8
	movq	-568(%rbp), %rcx
	movl	12(%r8), %eax
.L120:
	addq	$1, %rcx
	cmpl	%ecx, %eax
	jg	.L122
	movq	-8(%rbp), %r15
	.cfi_restore 15
.L119:
	movq	%r14, %rdi
	call	strlen
	movq	%r14, %rdi
	movw	$125, (%r14,%rax)
	call	strdup
	movq	%r14, %rdi
	movq	%rax, -552(%rbp)
	call	free
	movq	-552(%rbp), %rax
	movq	-16(%rbp), %r14
	.cfi_restore 14
	jmp	.L109
	.p2align 4,,10
	.p2align 3
.L108:
	movq	8(%r10), %rcx
	movl	$.LC4, %edx
	movl	$512, %esi
	xorl	%eax, %eax
	leaq	-544(%rbp), %rdi
	call	snprintf
	leaq	-544(%rbp), %rdi
	call	strdup
	jmp	.L109
	.p2align 4,,10
	.p2align 3
.L107:
	vmovsd	8(%r10), %xmm0
	movl	$.LC5, %edx
	movl	$512, %esi
	leaq	-544(%rbp), %rdi
	movl	$1, %eax
	call	snprintf
	leaq	-544(%rbp), %rdi
	call	strdup
	jmp	.L109
	.p2align 4,,10
	.p2align 3
.L106:
	movl	8(%r10), %r11d
	movl	$.LC3, %edi
	movl	$.LC2, %eax
	testl	%r11d, %r11d
	cmovne	%rax, %rdi
	call	strdup
	jmp	.L109
	.p2align 4,,10
	.p2align 3
.L105:
	movq	8(%r10), %rdi
	call	strdup
	jmp	.L109
	.p2align 4,,10
	.p2align 3
.L104:
	movq	8(%r10), %rax
	movq	%r15, -8(%rbp)
	testq	%rax, %rax
	.cfi_offset 15, -24
	movq	%rax, %r15
	je	.L111
	movl	8(%rax), %edx
	testl	%edx, %edx
	je	.L111
	movl	$1024, %esi
	movl	$1, %edi
	movq	%r14, -16(%rbp)
	.cfi_offset 14, -32
	movl	%edx, -552(%rbp)
	call	calloc
	movl	-552(%rbp), %r10d
	movw	$91, (%rax)
	movq	%rax, %r14
	testl	%r10d, %r10d
	jle	.L113
	xorl	%edx, %edx
.L116:
	movq	%rdx, %rax
	movq	%rdx, -552(%rbp)
	subq	$32, %rsp
	leaq	-544(%rbp), %rdi
	salq	$5, %rax
	addq	(%r15), %rax
	vmovdqu	(%rax), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	addq	$32, %rsp
	cmpq	$0, -552(%rbp)
	jne	.L133
	movq	-536(%rbp), %rsi
	movq	%r14, %rdi
	call	strcat
	vmovdqu	-544(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	addq	$32, %rsp
	cmpl	$1, 8(%r15)
	jle	.L113
	movl	$1, %edx
	jmp	.L116
	.p2align 4,,10
	.p2align 3
.L133:
	movq	%r14, %rdi
	call	strlen
	movq	-536(%rbp), %rsi
	movw	$8236, (%r14,%rax)
	leaq	2(%r14,%rax), %rdi
	movb	$0, 2(%r14,%rax)
	call	strcpy
	vmovdqu	-544(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movq	-552(%rbp), %rdx
	addq	$32, %rsp
	addq	$1, %rdx
	cmpl	%edx, 8(%r15)
	jg	.L116
.L113:
	movq	%r14, %rdi
	call	strlen
	movq	%r14, %rdi
	movw	$93, (%r14,%rax)
	call	strdup
	movq	%r14, %rdi
	movq	%rax, -552(%rbp)
	call	free
	movq	-552(%rbp), %rax
	movq	-16(%rbp), %r14
	.cfi_restore 14
	movq	-8(%rbp), %r15
	.cfi_restore 15
	jmp	.L109
	.p2align 4,,10
	.p2align 3
.L111:
	.cfi_offset 15, -24
	movl	$.LC7, %edi
	call	strdup
	movq	-8(%rbp), %r15
	.cfi_restore 15
	jmp	.L109
	.p2align 4,,10
	.p2align 3
.L117:
	movl	$.LC8, %edi
	call	strdup
	jmp	.L109
	.p2align 4,,10
	.p2align 3
.L132:
	.cfi_offset 14, -32
	.cfi_offset 15, -24
	movq	%r14, %rdi
	movq	%rcx, -568(%rbp)
	movq	%r8, -560(%rbp)
	movq	%rdx, -552(%rbp)
	call	strlen
	movq	-568(%rbp), %rcx
	movq	-560(%rbp), %r8
	movw	$8236, (%r14,%rax)
	movq	-552(%rbp), %rdx
	movb	$0, 2(%r14,%rax)
	jmp	.L121
	.cfi_endproc
.LFE38:
	.size	aly_to_str, .-aly_to_str
	.p2align 4
	.globl	native_print
	.type	native_print, @function
native_print:
.LFB108:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	pushq	%r10
	subq	$56, %rsp
	.cfi_offset 10, -24
	testl	%esi, %esi
	jg	.L138
.L135:
	movl	$6, (%rcx)
	movq	%rcx, %rax
	movq	$0, 8(%rcx)
	movq	$0, 16(%rcx)
	movl	$1, 24(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L138:
	.cfi_restore_state
	cmpl	$3, (%rdx)
	movq	%rdi, -56(%rbp)
	je	.L139
	vmovdqu	(%rdx), %ymm0
	subq	$32, %rsp
	leaq	-48(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-40(%rbp), %rdi
	addq	$32, %rsp
	call	puts
	vmovdqu	-48(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movq	-56(%rbp), %rcx
	addq	$32, %rsp
	jmp	.L135
	.p2align 4,,10
	.p2align 3
.L139:
	movq	8(%rdx), %rdi
	call	puts
	movq	-56(%rbp), %rcx
	jmp	.L135
	.cfi_endproc
.LFE108:
	.size	native_print, .-native_print
	.p2align 4
	.globl	aly_band
	.type	aly_band, @function
aly_band:
.LFB39:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	vmovdqu	16(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L141
	ja	.L142
	testl	%eax, %eax
	movq	24(%rbp), %r8
	je	.L146
	vcvttsd2siq	24(%rsp), %r8
.L146:
	movl	48(%rbp), %eax
	vmovdqu	48(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L147
	ja	.L148
	testl	%eax, %eax
	je	.L157
	vcvttsd2siq	24(%rsp), %rax
	andq	%rax, %r8
	vzeroupper
.L152:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r8, 8(%rcx)
	movq	$0, 16(%rcx)
	movl	$1, 24(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L142:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L146
	movq	%rdi, 8(%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	vzeroupper
	call	strtoll
	movq	8(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L146
	.p2align 4,,10
	.p2align 3
.L148:
	cmpl	$3, %eax
	jne	.L158
	movq	%rcx, (%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	movq	%r8, 8(%rsp)
	vzeroupper
	call	strtoll
	movq	8(%rsp), %r8
	movq	(%rsp), %rcx
	andq	%rax, %r8
	jmp	.L152
	.p2align 4,,10
	.p2align 3
.L147:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	andq	%rax, %r8
	vzeroupper
	jmp	.L152
	.p2align 4,,10
	.p2align 3
.L141:
	movl	24(%rsp), %esi
	xorl	%r8d, %r8d
	testl	%esi, %esi
	setne	%r8b
	jmp	.L146
	.p2align 4,,10
	.p2align 3
.L157:
	andq	56(%rbp), %r8
	vzeroupper
	jmp	.L152
	.p2align 4,,10
	.p2align 3
.L158:
	xorl	%r8d, %r8d
	vzeroupper
	jmp	.L152
	.cfi_endproc
.LFE39:
	.size	aly_band, .-aly_band
	.p2align 4
	.globl	aly_bor
	.type	aly_bor, @function
aly_bor:
.LFB40:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	vmovdqu	16(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L160
	ja	.L161
	testl	%eax, %eax
	movq	24(%rbp), %r8
	je	.L165
	vcvttsd2siq	24(%rsp), %r8
.L165:
	movl	48(%rbp), %eax
	vmovdqu	48(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L166
	ja	.L167
	testl	%eax, %eax
	je	.L177
	vcvttsd2siq	24(%rsp), %rax
	orq	%rax, %r8
	vzeroupper
.L171:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r8, 8(%rcx)
	movq	$0, 16(%rcx)
	movl	$1, 24(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L161:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L165
	movq	%rdi, 8(%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	vzeroupper
	call	strtoll
	movq	8(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L165
	.p2align 4,,10
	.p2align 3
.L167:
	cmpl	$3, %eax
	jne	.L176
	movq	%rcx, (%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	movq	%r8, 8(%rsp)
	vzeroupper
	call	strtoll
	movq	8(%rsp), %r8
	movq	(%rsp), %rcx
	orq	%rax, %r8
	jmp	.L171
	.p2align 4,,10
	.p2align 3
.L177:
	orq	56(%rbp), %r8
.L176:
	vzeroupper
	jmp	.L171
	.p2align 4,,10
	.p2align 3
.L166:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	orq	%rax, %r8
	vzeroupper
	jmp	.L171
	.p2align 4,,10
	.p2align 3
.L160:
	movl	24(%rsp), %esi
	xorl	%r8d, %r8d
	testl	%esi, %esi
	setne	%r8b
	jmp	.L165
	.cfi_endproc
.LFE40:
	.size	aly_bor, .-aly_bor
	.p2align 4
	.globl	aly_bxor
	.type	aly_bxor, @function
aly_bxor:
.LFB41:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	vmovdqu	16(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L179
	ja	.L180
	testl	%eax, %eax
	movq	24(%rbp), %r8
	je	.L184
	vcvttsd2siq	24(%rsp), %r8
.L184:
	movl	48(%rbp), %eax
	vmovdqu	48(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L185
	ja	.L186
	testl	%eax, %eax
	je	.L196
	vcvttsd2siq	24(%rsp), %rax
	xorq	%rax, %r8
	vzeroupper
.L190:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r8, 8(%rcx)
	movq	$0, 16(%rcx)
	movl	$1, 24(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L180:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L184
	movq	%rdi, 8(%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	vzeroupper
	call	strtoll
	movq	8(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L184
	.p2align 4,,10
	.p2align 3
.L186:
	cmpl	$3, %eax
	jne	.L195
	movq	%rcx, (%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	movq	%r8, 8(%rsp)
	vzeroupper
	call	strtoll
	movq	8(%rsp), %r8
	movq	(%rsp), %rcx
	xorq	%rax, %r8
	jmp	.L190
	.p2align 4,,10
	.p2align 3
.L196:
	xorq	56(%rbp), %r8
.L195:
	vzeroupper
	jmp	.L190
	.p2align 4,,10
	.p2align 3
.L185:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	xorq	%rax, %r8
	vzeroupper
	jmp	.L190
	.p2align 4,,10
	.p2align 3
.L179:
	movl	24(%rsp), %esi
	xorl	%r8d, %r8d
	testl	%esi, %esi
	setne	%r8b
	jmp	.L184
	.cfi_endproc
.LFE41:
	.size	aly_bxor, .-aly_bxor
	.p2align 4
	.globl	aly_bnot
	.type	aly_bnot, @function
aly_bnot:
.LFB42:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	vmovdqu	16(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L198
	ja	.L199
	testl	%eax, %eax
	je	.L206
	vcvttsd2siq	24(%rsp), %rax
	notq	%rax
	vzeroupper
.L203:
	movq	%rax, 8(%rcx)
	movq	%rcx, %rax
	movl	$0, (%rcx)
	movq	$0, 16(%rcx)
	movl	$1, 24(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L199:
	.cfi_restore_state
	cmpl	$3, %eax
	jne	.L207
	movq	%rdi, 8(%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	vzeroupper
	call	strtoll
	movq	8(%rsp), %rcx
	notq	%rax
	jmp	.L203
	.p2align 4,,10
	.p2align 3
.L198:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	notq	%rax
	vzeroupper
	jmp	.L203
	.p2align 4,,10
	.p2align 3
.L206:
	movq	24(%rbp), %rax
	notq	%rax
	vzeroupper
	jmp	.L203
	.p2align 4,,10
	.p2align 3
.L207:
	movq	$-1, %rax
	vzeroupper
	jmp	.L203
	.cfi_endproc
.LFE42:
	.size	aly_bnot, .-aly_bnot
	.p2align 4
	.globl	aly_shl
	.type	aly_shl, @function
aly_shl:
.LFB43:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	vmovdqu	16(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L209
	ja	.L210
	testl	%eax, %eax
	movq	24(%rbp), %r8
	je	.L214
	vcvttsd2siq	24(%rsp), %r8
.L214:
	movl	48(%rbp), %eax
	vmovdqu	48(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L215
	ja	.L216
	testl	%eax, %eax
	je	.L226
	vcvttsd2siq	24(%rsp), %rax
	shlx	%rax, %r8, %r8
	vzeroupper
.L220:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r8, 8(%rcx)
	movq	$0, 16(%rcx)
	movl	$1, 24(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L210:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L214
	movq	%rdi, 8(%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	vzeroupper
	call	strtoll
	movq	8(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L214
	.p2align 4,,10
	.p2align 3
.L216:
	cmpl	$3, %eax
	jne	.L225
	movq	%rcx, (%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	movq	%r8, 8(%rsp)
	vzeroupper
	call	strtoll
	movq	8(%rsp), %r8
	movq	(%rsp), %rcx
	shlx	%rax, %r8, %r8
	jmp	.L220
	.p2align 4,,10
	.p2align 3
.L226:
	movq	56(%rbp), %rax
	shlx	%rax, %r8, %r8
.L225:
	vzeroupper
	jmp	.L220
	.p2align 4,,10
	.p2align 3
.L215:
	movl	24(%rsp), %eax
	testl	%eax, %eax
	setne	%al
	shlx	%rax, %r8, %r8
	vzeroupper
	jmp	.L220
	.p2align 4,,10
	.p2align 3
.L209:
	movl	24(%rsp), %edx
	xorl	%r8d, %r8d
	testl	%edx, %edx
	setne	%r8b
	jmp	.L214
	.cfi_endproc
.LFE43:
	.size	aly_shl, .-aly_shl
	.p2align 4
	.globl	aly_shr
	.type	aly_shr, @function
aly_shr:
.LFB44:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	vmovdqu	16(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L228
	ja	.L229
	testl	%eax, %eax
	movq	24(%rbp), %r8
	je	.L233
	vcvttsd2siq	24(%rsp), %r8
.L233:
	movl	48(%rbp), %eax
	vmovdqu	48(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L234
	ja	.L235
	testl	%eax, %eax
	je	.L245
	vcvttsd2siq	24(%rsp), %rax
	sarx	%rax, %r8, %r8
	vzeroupper
.L239:
	movl	$0, (%rcx)
	movq	%rcx, %rax
	movq	%r8, 8(%rcx)
	movq	$0, 16(%rcx)
	movl	$1, 24(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L229:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L233
	movq	%rdi, 8(%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	vzeroupper
	call	strtoll
	movq	8(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L233
	.p2align 4,,10
	.p2align 3
.L235:
	cmpl	$3, %eax
	jne	.L244
	movq	%rcx, (%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	movq	%r8, 8(%rsp)
	vzeroupper
	call	strtoll
	movq	8(%rsp), %r8
	movq	(%rsp), %rcx
	sarx	%rax, %r8, %r8
	jmp	.L239
	.p2align 4,,10
	.p2align 3
.L245:
	movq	56(%rbp), %rax
	sarx	%rax, %r8, %r8
.L244:
	vzeroupper
	jmp	.L239
	.p2align 4,,10
	.p2align 3
.L234:
	movl	24(%rsp), %eax
	testl	%eax, %eax
	setne	%al
	sarx	%rax, %r8, %r8
	vzeroupper
	jmp	.L239
	.p2align 4,,10
	.p2align 3
.L228:
	movl	24(%rsp), %edx
	xorl	%r8d, %r8d
	testl	%edx, %edx
	setne	%r8b
	jmp	.L233
	.cfi_endproc
.LFE44:
	.size	aly_shr, .-aly_shr
	.section	.rodata.str1.1
.LC9:
	.string	"%s%s"
	.text
	.p2align 4
	.globl	aly_add
	.type	aly_add, @function
aly_add:
.LFB45:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %r9
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$112, %rsp
	movq	%r10, -24(%rbp)
	movl	48(%rbp), %eax
	.cfi_offset 10, -40
	leaq	16(%rbp), %r10
	movl	(%r10), %edx
	cmpl	$3, %eax
	je	.L262
	cmpl	$3, %edx
	je	.L262
	cmpl	$1, %edx
	vxorps	%xmm0, %xmm0, %xmm0
	je	.L250
	cmpl	$1, %eax
	je	.L267
	vmovdqu	(%r10), %ymm0
	cmpl	$2, %edx
	vmovdqu	%ymm0, -64(%rbp)
	je	.L256
	movl	$0, %edx
	ja	.L257
	movq	8(%r10), %rdx
.L257:
	vmovdqu	32(%r10), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, -64(%rbp)
	je	.L258
	ja	.L259
	addq	40(%r10), %rdx
.L259:
	movl	$0, (%r9)
	movq	%r9, %rax
	movq	%rdx, 8(%r9)
	movq	$0, 16(%r9)
	movl	$1, 24(%r9)
	vzeroupper
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L267:
	.cfi_restore_state
	testl	%edx, %edx
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L252
	vcvtsi2sdq	8(%r10), %xmm0, %xmm0
	vmovapd	%xmm0, %xmm1
.L252:
	vmovsd	40(%r10), %xmm0
	jmp	.L255
	.p2align 4,,10
	.p2align 3
.L262:
	movq	%r9, -112(%rbp)
	subq	$32, %rsp
	vmovdqu	(%r10), %ymm0
	leaq	-96(%rbp), %rdi
	movq	%rbx, -32(%rbp)
	movq	%r14, -16(%rbp)
	movq	%r15, -8(%rbp)
	.cfi_offset 3, -48
	.cfi_offset 14, -32
	.cfi_offset 15, -24
	movq	%r10, -104(%rbp)
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-104(%rbp), %r10
	movq	-88(%rbp), %r14
	leaq	-64(%rbp), %rdi
	vmovdqu	32(%r10), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-56(%rbp), %r15
	addq	$32, %rsp
	movq	%r14, %rdi
	call	strlen
	movq	%r15, %rdi
	movq	%rax, %rbx
	call	strlen
	leal	1(%rbx,%rax), %esi
	movslq	%esi, %rsi
	movq	%rsi, %rdi
	movq	%rsi, -104(%rbp)
	call	malloc
	movq	-104(%rbp), %rsi
	movq	%r15, %r8
	movq	%r14, %rcx
	movl	$.LC9, %edx
	movq	%rax, %rdi
	movq	%rax, %rbx
	xorl	%eax, %eax
	call	snprintf
	vmovdqu	-96(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	vmovdqu	-64(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	addq	$32, %rsp
	movq	%rbx, %rdi
	call	strdup
	movq	%rbx, %rdi
	movq	%rax, -104(%rbp)
	call	free
	movq	-112(%rbp), %r9
	movq	-104(%rbp), %rax
	movq	-32(%rbp), %rbx
	.cfi_restore 3
	movq	-16(%rbp), %r14
	.cfi_restore 14
	movq	%rax, 8(%r9)
	movq	-8(%rbp), %r15
	.cfi_restore 15
	movq	%r9, %rax
	movl	$3, (%r9)
	movq	$0, 16(%r9)
	movl	$1, 24(%r9)
	leave
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L250:
	.cfi_def_cfa 6, 16
	cmpl	$3, %eax
	vmovsd	8(%r10), %xmm1
	ja	.L253
	testl	%eax, %eax
	jne	.L268
	vcvtsi2sdq	40(%r10), %xmm0, %xmm0
.L255:
	vaddsd	%xmm1, %xmm0, %xmm0
	movl	$1, (%r9)
	movq	%r9, %rax
	movq	$0, 16(%r9)
	movl	$1, 24(%r9)
	vmovsd	%xmm0, 8(%r9)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L268:
	.cfi_restore_state
	cmpl	$1, %eax
	je	.L252
.L253:
	vxorpd	%xmm0, %xmm0, %xmm0
	jmp	.L255
	.p2align 4,,10
	.p2align 3
.L258:
	cmpl	$1, -56(%rbp)
	sbbq	$-1, %rdx
	jmp	.L259
	.p2align 4,,10
	.p2align 3
.L256:
	movl	-56(%rbp), %ecx
	xorl	%edx, %edx
	testl	%ecx, %ecx
	setne	%dl
	jmp	.L257
	.cfi_endproc
.LFE45:
	.size	aly_add, .-aly_add
	.p2align 4
	.globl	aly_sub
	.type	aly_sub, @function
aly_sub:
.LFB46:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	vxorps	%xmm0, %xmm0, %xmm0
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	movl	48(%rbp), %r8d
	cmpl	$1, %eax
	je	.L270
	cmpl	$1, %r8d
	je	.L294
	vmovdqu	16(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L280
	movq	24(%rbp), %r9
	ja	.L295
.L283:
	vmovdqu	48(%rbp), %ymm0
	cmpl	$2, %r8d
	vmovdqu	%ymm0, 16(%rsp)
	je	.L284
	ja	.L296
	subq	56(%rbp), %r9
.L293:
	vzeroupper
.L287:
	movl	$0, (%rcx)
	movq	%r9, 8(%rcx)
	jmp	.L279
	.p2align 4,,10
	.p2align 3
.L270:
	cmpl	$3, %r8d
	vmovsd	24(%rbp), %xmm1
	movq	56(%rbp), %rdi
	je	.L274
	ja	.L276
	testl	%r8d, %r8d
	jne	.L297
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vsubsd	%xmm0, %xmm1, %xmm1
.L276:
	movl	$1, (%rcx)
	vmovsd	%xmm1, 8(%rcx)
.L279:
	movq	$0, 16(%rcx)
	movq	%rcx, %rax
	movl	$1, 24(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L294:
	.cfi_restore_state
	cmpl	$3, %eax
	movq	24(%rbp), %rdi
	je	.L272
	testl	%eax, %eax
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L273
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vmovapd	%xmm0, %xmm1
.L273:
	vsubsd	56(%rbp), %xmm1, %xmm1
	jmp	.L276
	.p2align 4,,10
	.p2align 3
.L280:
	movl	24(%rsp), %eax
	xorl	%r9d, %r9d
	testl	%eax, %eax
	setne	%r9b
	jmp	.L283
	.p2align 4,,10
	.p2align 3
.L295:
	xorl	%r9d, %r9d
	cmpl	$3, %eax
	jne	.L283
	movl	%r8d, 8(%rsp)
	xorl	%esi, %esi
	movl	$10, %edx
	movq	%rdi, (%rsp)
	movq	24(%rsp), %rdi
	vzeroupper
	call	strtoll
	movq	(%rsp), %rcx
	movl	8(%rsp), %r8d
	movq	%rax, %r9
	jmp	.L283
	.p2align 4,,10
	.p2align 3
.L296:
	cmpl	$3, %r8d
	jne	.L293
	movq	%rcx, (%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	movq	%r9, 8(%rsp)
	vzeroupper
	call	strtoll
	movq	8(%rsp), %r9
	movq	(%rsp), %rcx
	subq	%rax, %r9
	jmp	.L287
	.p2align 4,,10
	.p2align 3
.L284:
	cmpl	$1, 24(%rsp)
	adcq	$-1, %r9
	vzeroupper
	jmp	.L287
	.p2align 4,,10
	.p2align 3
.L272:
	xorl	%esi, %esi
	movq	%rcx, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rcx
	vmovapd	%xmm0, %xmm1
	jmp	.L273
	.p2align 4,,10
	.p2align 3
.L274:
	xorl	%esi, %esi
	movq	%rcx, (%rsp)
	vmovsd	%xmm1, 8(%rsp)
	call	strtod
	vmovsd	8(%rsp), %xmm1
	movq	(%rsp), %rcx
	vsubsd	%xmm0, %xmm1, %xmm1
	jmp	.L276
	.p2align 4,,10
	.p2align 3
.L297:
	cmpl	$1, %r8d
	jne	.L276
	jmp	.L273
	.cfi_endproc
.LFE46:
	.size	aly_sub, .-aly_sub
	.p2align 4
	.globl	aly_mul
	.type	aly_mul, @function
aly_mul:
.LFB47:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$96, %rsp
	movq	%r10, -32(%rbp)
	movl	16(%rbp), %eax
	.cfi_offset 10, -48
	leaq	16(%rbp), %r10
	movq	%rbx, -40(%rbp)
	movl	32(%r10), %ecx
	.cfi_offset 3, -56
	movq	%rdi, %rbx
	cmpl	$3, %eax
	movq	%r12, -24(%rbp)
	.cfi_offset 12, -40
	movq	40(%r10), %r12
	je	.L336
	cmpl	$1, %eax
	vxorps	%xmm0, %xmm0, %xmm0
	je	.L307
	cmpl	$1, %ecx
	je	.L308
	vmovdqu	(%r10), %ymm0
	cmpl	$1, %eax
	vmovdqu	%ymm0, -80(%rbp)
	jbe	.L337
	xorl	%r8d, %r8d
	cmpl	$2, %eax
	jne	.L318
	movl	-72(%rbp), %edx
	xorl	%r8d, %r8d
	testl	%edx, %edx
	setne	%r8b
.L318:
	vmovdqu	32(%r10), %ymm0
	cmpl	$2, %ecx
	vmovdqu	%ymm0, -80(%rbp)
	je	.L302
	imulq	%r8, %r12
	cmpl	$2, %ecx
	ja	.L303
	vzeroupper
.L321:
	movq	%r12, 8(%rbx)
	movq	%rbx, %rax
	movq	-24(%rbp), %r12
	movl	$0, (%rbx)
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	movq	-40(%rbp), %rbx
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L336:
	.cfi_restore_state
	testl	%ecx, %ecx
	je	.L300
	cmpl	$1, %ecx
	je	.L301
	vmovdqu	(%r10), %ymm0
	movq	%r10, -88(%rbp)
	xorl	%esi, %esi
	movl	$10, %edx
	movl	%ecx, -92(%rbp)
	vmovdqu	%ymm0, -80(%rbp)
	movq	-72(%rbp), %rdi
	vzeroupper
	call	strtoll
	movq	-88(%rbp), %r10
	movl	-92(%rbp), %ecx
	movq	%rax, %r8
	vmovdqu	32(%r10), %ymm0
	cmpl	$2, %ecx
	vmovdqu	%ymm0, -80(%rbp)
	je	.L302
.L303:
	cmpl	$3, %ecx
	jne	.L338
	movq	%r8, -88(%rbp)
	movq	-72(%rbp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	vzeroupper
	call	strtoll
	imulq	-88(%rbp), %rax
	movq	%rax, %r12
	jmp	.L321
	.p2align 4,,10
	.p2align 3
.L307:
	cmpl	$3, %ecx
	vmovsd	8(%r10), %xmm1
	je	.L312
	ja	.L316
	testl	%ecx, %ecx
	jne	.L339
	vcvtsi2sdq	%r12, %xmm0, %xmm0
.L314:
	vmulsd	%xmm1, %xmm0, %xmm0
	movl	$1, (%rbx)
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	vmovsd	%xmm0, 8(%rbx)
.L298:
	movq	%rbx, %rax
	movq	-24(%rbp), %r12
	movq	-40(%rbp), %rbx
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L308:
	.cfi_restore_state
	testl	%eax, %eax
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L311
	vcvtsi2sdq	8(%r10), %xmm0, %xmm0
	vmovapd	%xmm0, %xmm1
.L311:
	vmovq	%r12, %xmm0
	jmp	.L314
	.p2align 4,,10
	.p2align 3
.L337:
	movq	8(%r10), %r8
	jmp	.L318
	.p2align 4,,10
	.p2align 3
.L312:
	xorl	%esi, %esi
	movq	%r12, %rdi
	vmovsd	%xmm1, -88(%rbp)
	call	strtod
	vmovsd	-88(%rbp), %xmm1
	jmp	.L314
	.p2align 4,,10
	.p2align 3
.L301:
	movq	8(%r10), %rdi
	xorl	%esi, %esi
	call	strtod
	vmovapd	%xmm0, %xmm1
	jmp	.L311
	.p2align 4,,10
	.p2align 3
.L302:
	movl	-72(%rbp), %eax
	xorl	%r12d, %r12d
	testl	%eax, %eax
	cmovne	%r8, %r12
	vzeroupper
	jmp	.L321
	.p2align 4,,10
	.p2align 3
.L339:
	cmpl	$1, %ecx
	je	.L311
.L316:
	vxorpd	%xmm0, %xmm0, %xmm0
	jmp	.L314
	.p2align 4,,10
	.p2align 3
.L300:
	vmovdqu	(%r10), %ymm0
	movq	%r14, -16(%rbp)
	subq	$32, %rsp
	leaq	-80(%rbp), %rdi
	movq	%r15, -8(%rbp)
	.cfi_offset 14, -32
	.cfi_offset 15, -24
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-72(%rbp), %rdi
	addq	$32, %rsp
	movq	-72(%rbp), %r15
	call	strlen
	movl	%eax, %edi
	imull	%r12d, %edi
	addl	$1, %edi
	movslq	%edi, %rdi
	call	malloc
	testl	%r12d, %r12d
	movb	$0, (%rax)
	movq	%rax, %r14
	jle	.L304
	xorl	%edx, %edx
	.p2align 4,,10
	.p2align 3
.L305:
	movq	%r15, %rsi
	movq	%r14, %rdi
	movl	%edx, -88(%rbp)
	call	strcat
	movl	-88(%rbp), %edx
	addl	$1, %edx
	cmpl	%edx, %r12d
	jne	.L305
.L304:
	vmovdqu	-80(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movq	%r14, %rdi
	addq	$32, %rsp
	call	strdup
	movq	%r14, %rdi
	movq	%rax, %r12
	call	free
	movl	$3, (%rbx)
	movq	-16(%rbp), %r14
	.cfi_restore 14
	movq	%r12, 8(%rbx)
	movq	-8(%rbp), %r15
	.cfi_restore 15
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	jmp	.L298
	.p2align 4,,10
	.p2align 3
.L338:
	xorl	%r12d, %r12d
	vzeroupper
	jmp	.L321
	.cfi_endproc
.LFE47:
	.size	aly_mul, .-aly_mul
	.section	.rodata.str1.1
.LC10:
	.string	"Error: Division by zero\n"
	.text
	.p2align 4
	.globl	aly_div
	.type	aly_div, @function
aly_div:
.LFB48:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	vxorps	%xmm2, %xmm2, %xmm2
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	vmovq	%rdi, %xmm1
	je	.L343
	cmpl	$3, %edx
	je	.L342
	testl	%edx, %edx
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L343
	vcvtsi2sdq	%rdi, %xmm2, %xmm1
.L343:
	movl	64(%rsp), %edx
	movq	72(%rsp), %rdi
	cmpl	$1, %edx
	je	.L344
.L353:
	cmpl	$3, %edx
	je	.L345
	testl	%edx, %edx
	je	.L352
.L346:
	movl	$.LC10, %edi
	movl	$24, %edx
	movl	$1, %esi
	movq	stderr(%rip), %rcx
	call	fwrite
	movl	$1, %edi
	call	exit
	.p2align 4,,10
	.p2align 3
.L342:
	xorl	%esi, %esi
	movq	%rax, (%rsp)
	call	strtod
	movl	64(%rsp), %edx
	movq	(%rsp), %rax
	vxorps	%xmm2, %xmm2, %xmm2
	movq	72(%rsp), %rdi
	vmovapd	%xmm0, %xmm1
	cmpl	$1, %edx
	jne	.L353
.L344:
	vmovq	%rdi, %xmm0
	jmp	.L347
	.p2align 4,,10
	.p2align 3
.L345:
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	vmovsd	%xmm1, (%rsp)
	call	strtod
	movq	8(%rsp), %rax
	vmovsd	(%rsp), %xmm1
.L347:
	vxorpd	%xmm2, %xmm2, %xmm2
	vucomisd	%xmm2, %xmm0
	jp	.L348
	je	.L346
.L348:
	vdivsd	%xmm0, %xmm1, %xmm1
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	vmovsd	%xmm1, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L352:
	.cfi_restore_state
	vcvtsi2sdq	%rdi, %xmm2, %xmm2
	vmovapd	%xmm2, %xmm0
	jmp	.L347
	.cfi_endproc
.LFE48:
	.size	aly_div, .-aly_div
	.section	.rodata.str1.1
.LC11:
	.string	"Error: Modulo by zero\n"
	.text
	.p2align 4
	.globl	aly_mod
	.type	aly_mod, @function
aly_mod:
.LFB49:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movq	%rdi, %rcx
	movl	48(%rsp), %edx
	movl	80(%rsp), %esi
	movq	56(%rsp), %rax
	movq	88(%rsp), %rdi
	movl	%edx, %r8d
	orl	%esi, %r8d
	jne	.L355
	testq	%rdi, %rdi
	je	.L370
	cqto
	movl	$0, (%rcx)
	idivq	%rdi
	movq	%rdx, 8(%rcx)
.L357:
	movq	$0, 16(%rcx)
	movq	%rcx, %rax
	movl	$1, 24(%rcx)
	addq	$40, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L355:
	.cfi_restore_state
	cmpl	$1, %esi
	vxorps	%xmm1, %xmm1, %xmm1
	je	.L358
	cmpl	$3, %esi
	je	.L359
	movq	$0x000000000, 8(%rsp)
	testl	%esi, %esi
	je	.L371
.L360:
	cmpl	$1, %edx
	je	.L361
.L374:
	cmpl	$3, %edx
	je	.L362
	testl	%edx, %edx
	movq	$0x000000000, (%rsp)
	je	.L372
.L363:
	fldl	8(%rsp)
	fldl	(%rsp)
.L364:
	fprem
	fnstsw	%ax
	sahf
	jp	.L364
	fstp	%st(1)
	fstpl	16(%rsp)
	vmovsd	16(%rsp), %xmm2
	vucomisd	%xmm2, %xmm2
	jp	.L373
.L365:
	movl	$1, (%rcx)
	vmovsd	%xmm2, 8(%rcx)
	jmp	.L357
	.p2align 4,,10
	.p2align 3
.L372:
	vcvtsi2sdq	%rax, %xmm1, %xmm1
	vmovlpd	%xmm1, (%rsp)
	jmp	.L363
	.p2align 4,,10
	.p2align 3
.L371:
	cmpl	$1, %edx
	vcvtsi2sdq	%rdi, %xmm1, %xmm0
	vmovlpd	%xmm0, 8(%rsp)
	jne	.L374
.L361:
	movq	%rax, (%rsp)
	jmp	.L363
	.p2align 4,,10
	.p2align 3
.L358:
	movq	%rdi, 8(%rsp)
	jmp	.L360
	.p2align 4,,10
	.p2align 3
.L359:
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
	jmp	.L360
	.p2align 4,,10
	.p2align 3
.L362:
	xorl	%esi, %esi
	movq	%rax, %rdi
	movq	%rcx, 16(%rsp)
	call	strtod
	movq	16(%rsp), %rcx
	vmovsd	%xmm0, (%rsp)
	jmp	.L363
.L370:
	movl	$.LC11, %edi
	movl	$22, %edx
	movl	$1, %esi
	movq	stderr(%rip), %rcx
	call	fwrite
	movl	$1, %edi
	call	exit
.L373:
	vmovsd	8(%rsp), %xmm1
	vmovsd	(%rsp), %xmm0
	movq	%rcx, 24(%rsp)
	call	fmod
	movq	24(%rsp), %rcx
	vmovsd	16(%rsp), %xmm2
	jmp	.L365
	.cfi_endproc
.LFE49:
	.size	aly_mod, .-aly_mod
	.p2align 4
	.globl	aly_neg
	.type	aly_neg, @function
aly_neg:
.LFB50:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	vmovsd	24(%rbp), %xmm0
	cmpl	$1, %eax
	je	.L383
	vmovdqu	16(%rbp), %ymm1
	cmpl	$2, %eax
	vmovdqu	%ymm1, 16(%rsp)
	je	.L378
	ja	.L384
	vmovq	%xmm0, %rax
	negq	%rax
	vzeroupper
.L381:
	movl	$0, (%rcx)
	movq	%rax, 8(%rcx)
	jmp	.L377
	.p2align 4,,10
	.p2align 3
.L383:
	vxorpd	.LC13(%rip), %xmm0, %xmm0
	movl	$1, (%rdi)
	vmovsd	%xmm0, 8(%rdi)
.L377:
	movq	$0, 16(%rcx)
	movq	%rcx, %rax
	movl	$1, 24(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L384:
	.cfi_restore_state
	cmpl	$3, %eax
	jne	.L385
	movq	%rdi, 8(%rsp)
	xorl	%esi, %esi
	vmovq	%xmm0, %rdi
	movl	$10, %edx
	vzeroupper
	call	strtoll
	movq	8(%rsp), %rcx
	negq	%rax
	jmp	.L381
	.p2align 4,,10
	.p2align 3
.L378:
	movl	24(%rsp), %eax
	negl	%eax
	sbbq	%rax, %rax
	vzeroupper
	jmp	.L381
	.p2align 4,,10
	.p2align 3
.L385:
	xorl	%eax, %eax
	vzeroupper
	jmp	.L381
	.cfi_endproc
.LFE50:
	.size	aly_neg, .-aly_neg
	.p2align 4
	.globl	aly_not
	.type	aly_not, @function
aly_not:
.LFB51:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rax
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	vmovdqu	16(%rbp), %ymm0
	cmpl	$5, 16(%rbp)
	vmovdqu	%ymm0, -32(%rsp)
	ja	.L387
	movl	16(%rbp), %edx
	jmp	*.L389(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L389:
	.quad	.L394
	.quad	.L393
	.quad	.L392
	.quad	.L391
	.quad	.L390
	.quad	.L390
	.text
	.p2align 4,,10
	.p2align 3
.L390:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L387
	movl	8(%rdx), %edx
	testl	%edx, %edx
	setg	%dl
	movzbl	%dl, %edx
.L395:
	testl	%edx, %edx
	movl	$2, (%rax)
	sete	%dl
	movq	$0, 16(%rax)
	movzbl	%dl, %edx
	movl	$1, 24(%rax)
	movl	%edx, 8(%rax)
	vzeroupper
	popq	%rbp
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L394:
	.cfi_restore_state
	xorl	%edx, %edx
	cmpq	$0, -24(%rsp)
	setne	%dl
	jmp	.L395
	.p2align 4,,10
	.p2align 3
.L393:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-24(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L395
	.p2align 4,,10
	.p2align 3
.L392:
	movl	-24(%rsp), %edx
	jmp	.L395
	.p2align 4,,10
	.p2align 3
.L391:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L387
	cmpb	$0, (%rdx)
	setne	%dl
	movzbl	%dl, %edx
	jmp	.L395
	.p2align 4,,10
	.p2align 3
.L387:
	xorl	%edx, %edx
	jmp	.L395
	.cfi_endproc
.LFE51:
	.size	aly_not, .-aly_not
	.p2align 4
	.globl	aly_and
	.type	aly_and, @function
aly_and:
.LFB52:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rax
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	vmovdqu	16(%rbp), %ymm0
	cmpl	$5, 16(%rbp)
	vmovdqu	%ymm0, -32(%rsp)
	ja	.L414
	movl	16(%rbp), %edx
	jmp	*.L408(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L408:
	.quad	.L413
	.quad	.L412
	.quad	.L411
	.quad	.L410
	.quad	.L407
	.quad	.L407
	.text
	.p2align 4,,10
	.p2align 3
.L407:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	jne	.L457
.L414:
	xorl	%edx, %edx
.L419:
	movl	$2, (%rax)
	movl	%edx, 8(%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	vzeroupper
	popq	%rbp
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L457:
	.cfi_restore_state
	movl	8(%rdx), %ecx
	testl	%ecx, %ecx
	jle	.L414
.L418:
	vmovdqu	48(%rbp), %ymm0
	cmpl	$5, 48(%rbp)
	vmovdqu	%ymm0, -32(%rsp)
	ja	.L414
	movl	48(%rbp), %edx
	jmp	*.L422(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L422:
	.quad	.L427
	.quad	.L426
	.quad	.L425
	.quad	.L424
	.quad	.L421
	.quad	.L421
	.text
	.p2align 4,,10
	.p2align 3
.L421:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L414
	movl	8(%rdx), %edx
	testl	%edx, %edx
	jle	.L414
.L430:
	movl	$1, %edx
	jmp	.L419
	.p2align 4,,10
	.p2align 3
.L413:
	xorl	%edx, %edx
	cmpq	$0, -24(%rsp)
	setne	%dl
.L415:
	testl	%edx, %edx
	jne	.L418
	jmp	.L419
	.p2align 4,,10
	.p2align 3
.L412:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-24(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L415
	.p2align 4,,10
	.p2align 3
.L411:
	movl	-24(%rsp), %edx
	jmp	.L415
	.p2align 4,,10
	.p2align 3
.L410:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L414
	cmpb	$0, (%rdx)
	jne	.L418
	jmp	.L414
	.p2align 4,,10
	.p2align 3
.L424:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L414
	cmpb	$0, (%rdx)
	jne	.L430
	jmp	.L414
	.p2align 4,,10
	.p2align 3
.L425:
	movl	-24(%rsp), %edx
.L428:
	testl	%edx, %edx
	setne	%dl
	movzbl	%dl, %edx
	jmp	.L419
	.p2align 4,,10
	.p2align 3
.L426:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-24(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L428
	.p2align 4,,10
	.p2align 3
.L427:
	xorl	%edx, %edx
	cmpq	$0, -24(%rsp)
	setne	%dl
	jmp	.L428
	.cfi_endproc
.LFE52:
	.size	aly_and, .-aly_and
	.p2align 4
	.globl	aly_or
	.type	aly_or, @function
aly_or:
.LFB53:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rax
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	vmovdqu	16(%rbp), %ymm0
	cmpl	$5, 16(%rbp)
	vmovdqu	%ymm0, -32(%rsp)
	ja	.L467
	movl	16(%rbp), %edx
	jmp	*.L461(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L461:
	.quad	.L466
	.quad	.L465
	.quad	.L464
	.quad	.L463
	.quad	.L460
	.quad	.L460
	.text
	.p2align 4,,10
	.p2align 3
.L466:
	xorl	%edx, %edx
	cmpq	$0, -24(%rsp)
	setne	%dl
.L468:
	testl	%edx, %edx
	jne	.L471
.L467:
	vmovdqu	48(%rbp), %ymm0
	cmpl	$5, 48(%rbp)
	vmovdqu	%ymm0, -32(%rsp)
	ja	.L484
	movl	48(%rbp), %edx
	jmp	*.L474(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L474:
	.quad	.L479
	.quad	.L478
	.quad	.L477
	.quad	.L476
	.quad	.L473
	.quad	.L473
	.text
	.p2align 4,,10
	.p2align 3
.L460:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L467
	movl	8(%rdx), %ecx
	testl	%ecx, %ecx
	jle	.L467
.L471:
	movl	$1, %edx
	jmp	.L483
	.p2align 4,,10
	.p2align 3
.L473:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L484
	movl	8(%rdx), %edx
	testl	%edx, %edx
	jg	.L471
.L484:
	xorl	%edx, %edx
.L483:
	movl	$2, (%rax)
	movl	%edx, 8(%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	vzeroupper
	popq	%rbp
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L465:
	.cfi_restore_state
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-24(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L468
	.p2align 4,,10
	.p2align 3
.L464:
	movl	-24(%rsp), %edx
	jmp	.L468
	.p2align 4,,10
	.p2align 3
.L463:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L467
	cmpb	$0, (%rdx)
	jne	.L471
	jmp	.L467
	.p2align 4,,10
	.p2align 3
.L479:
	xorl	%edx, %edx
	cmpq	$0, -24(%rsp)
	setne	%dl
.L480:
	testl	%edx, %edx
	setne	%dl
	movzbl	%dl, %edx
	jmp	.L483
	.p2align 4,,10
	.p2align 3
.L476:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L484
	cmpb	$0, (%rdx)
	jne	.L471
	jmp	.L484
	.p2align 4,,10
	.p2align 3
.L478:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-24(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L480
	.p2align 4,,10
	.p2align 3
.L477:
	movl	-24(%rsp), %edx
	jmp	.L480
	.cfi_endproc
.LFE53:
	.size	aly_or, .-aly_or
	.p2align 4
	.globl	aly_xor
	.type	aly_xor, @function
aly_xor:
.LFB54:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rax
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	vmovdqu	16(%rbp), %ymm0
	cmpl	$5, 16(%rbp)
	vmovdqu	%ymm0, -32(%rsp)
	ja	.L510
	movl	16(%rbp), %edx
	jmp	*.L512(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L512:
	.quad	.L517
	.quad	.L516
	.quad	.L515
	.quad	.L514
	.quad	.L513
	.quad	.L513
	.text
	.p2align 4,,10
	.p2align 3
.L513:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L510
	movl	8(%rdx), %esi
	xorl	%edx, %edx
	testl	%esi, %esi
	setg	%dl
.L518:
	vmovdqu	48(%rbp), %ymm0
	cmpl	$5, 48(%rbp)
	vmovdqu	%ymm0, -32(%rsp)
	ja	.L528
	movl	48(%rbp), %ecx
	jmp	*.L522(,%rcx,8)
	.section	.rodata
	.align 8
	.align 4
.L522:
	.quad	.L527
	.quad	.L526
	.quad	.L525
	.quad	.L524
	.quad	.L521
	.quad	.L521
	.text
	.p2align 4,,10
	.p2align 3
.L521:
	movq	-24(%rsp), %rcx
	testq	%rcx, %rcx
	je	.L528
	movl	8(%rcx), %ecx
	testl	%ecx, %ecx
	setg	%cl
	movzbl	%cl, %ecx
	xorl	%ecx, %edx
.L528:
	movl	$2, (%rax)
	movl	%edx, 8(%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	vzeroupper
	popq	%rbp
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L517:
	.cfi_restore_state
	xorl	%edx, %edx
	cmpq	$0, -24(%rsp)
	setne	%dl
	jmp	.L518
	.p2align 4,,10
	.p2align 3
.L516:
	xorl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %ecx
	vucomisd	-24(%rsp), %xmm0
	setp	%dl
	cmovne	%ecx, %edx
	jmp	.L518
	.p2align 4,,10
	.p2align 3
.L515:
	movl	-24(%rsp), %edx
	jmp	.L518
	.p2align 4,,10
	.p2align 3
.L514:
	movq	-24(%rsp), %rdx
	testq	%rdx, %rdx
	je	.L510
	cmpb	$0, (%rdx)
	setne	%dl
	movzbl	%dl, %edx
	jmp	.L518
	.p2align 4,,10
	.p2align 3
.L524:
	movq	-24(%rsp), %rcx
	testq	%rcx, %rcx
	je	.L528
	cmpb	$0, (%rcx)
	setne	%cl
	movzbl	%cl, %ecx
	xorl	%ecx, %edx
	jmp	.L528
	.p2align 4,,10
	.p2align 3
.L525:
	xorl	-24(%rsp), %edx
	jmp	.L528
	.p2align 4,,10
	.p2align 3
.L526:
	xorl	%ecx, %ecx
	vxorpd	%xmm0, %xmm0, %xmm0
	movl	$1, %esi
	vucomisd	-24(%rsp), %xmm0
	setp	%cl
	cmovne	%esi, %ecx
	xorl	%ecx, %edx
	jmp	.L528
	.p2align 4,,10
	.p2align 3
.L527:
	xorl	%ecx, %ecx
	cmpq	$0, -24(%rsp)
	setne	%cl
	xorl	%ecx, %edx
	jmp	.L528
	.p2align 4,,10
	.p2align 3
.L510:
	xorl	%edx, %edx
	jmp	.L518
	.cfi_endproc
.LFE54:
	.size	aly_xor, .-aly_xor
	.p2align 4
	.globl	aly_percent
	.type	aly_percent, @function
aly_percent:
.LFB55:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	vmovsd	24(%rbp), %xmm0
	cmpl	$1, %eax
	je	.L554
	vmovdqu	16(%rbp), %ymm2
	cmpl	$2, %eax
	vxorps	%xmm1, %xmm1, %xmm1
	vmovdqu	%ymm2, 16(%rsp)
	je	.L549
	ja	.L555
	vmovq	%xmm0, %rax
	vcvtsi2sdq	%rax, %xmm1, %xmm1
	vdivsd	.LC14(%rip), %xmm1, %xmm1
	vzeroupper
	jmp	.L548
	.p2align 4,,10
	.p2align 3
.L554:
	vdivsd	.LC14(%rip), %xmm0, %xmm1
.L548:
	movl	$1, (%rcx)
	movq	%rcx, %rax
	movq	$0, 16(%rcx)
	movl	$1, 24(%rcx)
	vmovsd	%xmm1, 8(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L555:
	.cfi_restore_state
	cmpl	$3, %eax
	jne	.L556
	movq	%rdi, 8(%rsp)
	xorl	%esi, %esi
	vmovq	%xmm0, %rdi
	movl	$10, %edx
	vzeroupper
	call	strtoll
	vxorps	%xmm1, %xmm1, %xmm1
	movq	8(%rsp), %rcx
	vcvtsi2sdq	%rax, %xmm1, %xmm1
	vdivsd	.LC14(%rip), %xmm1, %xmm1
	jmp	.L548
	.p2align 4,,10
	.p2align 3
.L549:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	vcvtsi2sdq	%rax, %xmm1, %xmm1
	vdivsd	.LC14(%rip), %xmm1, %xmm1
	vzeroupper
	jmp	.L548
	.p2align 4,,10
	.p2align 3
.L556:
	vxorpd	%xmm1, %xmm1, %xmm1
	vzeroupper
	jmp	.L548
	.cfi_endproc
.LFE55:
	.size	aly_percent, .-aly_percent
	.p2align 4
	.globl	aly_compare_int
	.type	aly_compare_int, @function
aly_compare_int:
.LFB56:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	vmovdqu	16(%rbp), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L558
	ja	.L559
	testl	%eax, %eax
	movq	24(%rbp), %r8
	je	.L563
	vcvttsd2siq	24(%rsp), %r8
.L563:
	movl	48(%rbp), %edx
	vmovdqu	48(%rbp), %ymm0
	cmpl	$2, %edx
	vmovdqu	%ymm0, 16(%rsp)
	je	.L564
	ja	.L565
	testl	%edx, %edx
	movq	56(%rbp), %rax
	je	.L588
	vcvttsd2siq	24(%rsp), %rax
	vzeroupper
.L569:
	movzbl	(%rcx), %edx
	cmpl	$61, %edx
	jne	.L571
	cmpb	$61, 1(%rcx)
	jne	.L571
	cmpb	$0, 2(%rcx)
	jne	.L571
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	cmpq	%rax, %r8
	sete	%al
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L559:
	.cfi_restore_state
	xorl	%r8d, %r8d
	cmpl	$3, %eax
	jne	.L563
	movq	%rdi, 8(%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	vzeroupper
	call	strtoll
	movq	8(%rsp), %rcx
	movq	%rax, %r8
	jmp	.L563
	.p2align 4,,10
	.p2align 3
.L571:
	cmpl	$33, %edx
	jne	.L574
	cmpb	$61, 1(%rcx)
	je	.L589
.L574:
	movzbl	(%rcx), %esi
	cmpl	$60, %esi
	jne	.L576
	cmpb	$0, 1(%rcx)
	jne	.L576
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	cmpq	%rax, %r8
	setl	%al
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L576:
	.cfi_restore_state
	cmpl	$60, %edx
	je	.L590
.L578:
	cmpl	$62, %esi
	jne	.L580
	cmpb	$0, 1(%rcx)
	jne	.L580
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	cmpq	%rax, %r8
	setg	%al
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L591:
	.cfi_restore_state
	xorl	%eax, %eax
.L588:
	vzeroupper
	jmp	.L569
	.p2align 4,,10
	.p2align 3
.L565:
	cmpl	$3, %edx
	jne	.L591
	movq	%rcx, (%rsp)
	movq	24(%rsp), %rdi
	xorl	%esi, %esi
	movl	$10, %edx
	movq	%r8, 8(%rsp)
	vzeroupper
	call	strtoll
	movq	(%rsp), %rcx
	movq	8(%rsp), %r8
	jmp	.L569
	.p2align 4,,10
	.p2align 3
.L558:
	movl	24(%rsp), %esi
	xorl	%r8d, %r8d
	testl	%esi, %esi
	setne	%r8b
	jmp	.L563
	.p2align 4,,10
	.p2align 3
.L564:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	vzeroupper
	jmp	.L569
	.p2align 4,,10
	.p2align 3
.L589:
	cmpb	$0, 2(%rcx)
	jne	.L574
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	cmpq	%rax, %r8
	setne	%al
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L590:
	.cfi_restore_state
	cmpb	$61, 1(%rcx)
	jne	.L578
	cmpb	$0, 2(%rcx)
	jne	.L578
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	cmpq	%rax, %r8
	setle	%al
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L580:
	.cfi_restore_state
	cmpl	$62, %edx
	jne	.L582
	cmpb	$61, 1(%rcx)
	jne	.L582
	cmpb	$0, 2(%rcx)
	jne	.L582
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	cmpq	%rax, %r8
	setge	%al
	movzbl	%al, %eax
	ret
	.p2align 4,,10
	.p2align 3
.L582:
	.cfi_restore_state
	leave
	.cfi_def_cfa 7, 8
	xorl	%eax, %eax
	ret
	.cfi_endproc
.LFE56:
	.size	aly_compare_int, .-aly_compare_int
	.p2align 4
	.globl	aly_compare
	.type	aly_compare, @function
aly_compare:
.LFB57:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsi, %rdx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$112, %rsp
	movq	%r10, -16(%rbp)
	movl	48(%rbp), %eax
	.cfi_offset 10, -32
	leaq	16(%rbp), %r10
	movq	%rbx, -24(%rbp)
	movl	(%r10), %ecx
	.cfi_offset 3, -40
	movq	%rdi, %rbx
	cmpl	$3, %eax
	je	.L629
	cmpl	$3, %ecx
	je	.L629
	cmpl	$1, %ecx
	vxorps	%xmm0, %xmm0, %xmm0
	je	.L608
	cmpl	$1, %eax
	je	.L635
	vmovdqu	32(%r10), %ymm0
	subq	$64, %rsp
	movq	%rsi, %rdi
	vmovdqu	%ymm0, 32(%rsp)
	vmovdqu	(%r10), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_compare_int
	movl	$2, (%rbx)
	addq	$64, %rsp
	movl	%eax, 8(%rbx)
	movq	%rbx, %rax
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	movq	-24(%rbp), %rbx
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L635:
	.cfi_restore_state
	testl	%ecx, %ecx
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L610
	vcvtsi2sdq	8(%r10), %xmm0, %xmm0
	vmovapd	%xmm0, %xmm1
.L610:
	vmovsd	40(%r10), %xmm0
	jmp	.L613
	.p2align 4,,10
	.p2align 3
.L629:
	subq	$32, %rsp
	movq	%rdx, -112(%rbp)
	vmovdqu	(%r10), %ymm0
	leaq	-96(%rbp), %rdi
	movq	%r10, -104(%rbp)
	movq	%r14, -8(%rbp)
	.cfi_offset 14, -24
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-104(%rbp), %r10
	leaq	-64(%rbp), %rdi
	vmovdqu	32(%r10), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-56(%rbp), %rsi
	movq	-88(%rbp), %rdi
	addq	$32, %rsp
	call	strcmp
	movq	-112(%rbp), %rdx
	movl	%eax, %ecx
	movzbl	(%rdx), %eax
	cmpl	$61, %eax
	jne	.L596
	cmpb	$61, 1(%rdx)
	jne	.L596
	cmpb	$0, 2(%rdx)
	jne	.L596
	xorl	%r14d, %r14d
	testl	%ecx, %ecx
	sete	%r14b
.L597:
	vmovdqu	-96(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	vmovdqu	-64(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movl	%r14d, 8(%rbx)
	movq	%rbx, %rax
	movl	$2, (%rbx)
	movq	-8(%rbp), %r14
	.cfi_restore 14
	addq	$32, %rsp
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	movq	-24(%rbp), %rbx
	leave
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L608:
	.cfi_def_cfa 6, 16
	cmpl	$3, %eax
	vmovsd	8(%r10), %xmm1
	ja	.L611
	testl	%eax, %eax
	jne	.L636
	vcvtsi2sdq	40(%r10), %xmm0, %xmm0
.L613:
	movzbl	(%rdx), %eax
	cmpl	$61, %eax
	jne	.L615
	cmpb	$61, 1(%rdx)
	jne	.L615
	movzbl	2(%rdx), %ecx
	testl	%ecx, %ecx
	jne	.L615
	xorl	%eax, %eax
	vucomisd	%xmm1, %xmm0
	setnp	%al
	cmovne	%ecx, %eax
.L616:
	movl	%eax, 8(%rbx)
	movq	%rbx, %rax
	movl	$2, (%rbx)
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	movq	-24(%rbp), %rbx
	leave
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L596:
	.cfi_def_cfa 6, 16
	.cfi_offset 14, -24
	cmpl	$33, %eax
	jne	.L599
	cmpb	$61, 1(%rdx)
	jne	.L599
	cmpb	$0, 2(%rdx)
	jne	.L599
	xorl	%r14d, %r14d
	testl	%ecx, %ecx
	setne	%r14b
	jmp	.L597
	.p2align 4,,10
	.p2align 3
.L636:
	.cfi_restore 14
	cmpl	$1, %eax
	je	.L610
.L611:
	vxorpd	%xmm0, %xmm0, %xmm0
	jmp	.L613
	.p2align 4,,10
	.p2align 3
.L599:
	.cfi_offset 14, -24
	movzbl	(%rdx), %esi
	cmpl	$60, %esi
	je	.L637
.L601:
	cmpl	$60, %eax
	jne	.L603
	cmpb	$61, 1(%rdx)
	jne	.L603
	cmpb	$0, 2(%rdx)
	jne	.L603
	xorl	%r14d, %r14d
	testl	%ecx, %ecx
	setle	%r14b
	jmp	.L597
	.p2align 4,,10
	.p2align 3
.L615:
	.cfi_restore 14
	cmpl	$33, %eax
	jne	.L618
	cmpb	$61, 1(%rdx)
	jne	.L618
	cmpb	$0, 2(%rdx)
	jne	.L618
	xorl	%eax, %eax
	vucomisd	%xmm1, %xmm0
	movl	$1, %edx
	setp	%al
	cmovne	%edx, %eax
	jmp	.L616
	.p2align 4,,10
	.p2align 3
.L618:
	movzbl	(%rdx), %esi
	cmpl	$60, %esi
	je	.L638
.L620:
	cmpl	$60, %eax
	jne	.L622
	cmpb	$61, 1(%rdx)
	jne	.L622
	cmpb	$0, 2(%rdx)
	jne	.L622
	xorl	%eax, %eax
	vcomisd	%xmm1, %xmm0
	setnb	%al
	jmp	.L616
	.p2align 4,,10
	.p2align 3
.L637:
	.cfi_offset 14, -24
	cmpb	$0, 1(%rdx)
	jne	.L601
.L634:
	shrl	$31, %ecx
	movl	%ecx, %r14d
	jmp	.L597
	.p2align 4,,10
	.p2align 3
.L638:
	.cfi_restore 14
	cmpb	$0, 1(%rdx)
	jne	.L620
	xorl	%eax, %eax
	vcomisd	%xmm1, %xmm0
	seta	%al
	jmp	.L616
	.p2align 4,,10
	.p2align 3
.L603:
	.cfi_offset 14, -24
	cmpl	$62, %esi
	jne	.L605
	cmpb	$0, 1(%rdx)
	jne	.L605
	xorl	%r14d, %r14d
	testl	%ecx, %ecx
	setg	%r14b
	jmp	.L597
	.p2align 4,,10
	.p2align 3
.L622:
	.cfi_restore 14
	cmpl	$62, %esi
	jne	.L624
	cmpb	$0, 1(%rdx)
	jne	.L624
	xorl	%eax, %eax
	vcomisd	%xmm0, %xmm1
	seta	%al
	jmp	.L616
.L605:
	.cfi_offset 14, -24
	cmpl	$62, %eax
	jne	.L626
	cmpb	$61, 1(%rdx)
	jne	.L626
	cmpb	$0, 2(%rdx)
	jne	.L626
	notl	%ecx
	jmp	.L634
.L624:
	.cfi_restore 14
	cmpl	$62, %eax
	jne	.L628
	cmpb	$61, 1(%rdx)
	jne	.L628
	cmpb	$0, 2(%rdx)
	jne	.L628
	xorl	%eax, %eax
	vcomisd	%xmm0, %xmm1
	setnb	%al
	jmp	.L616
.L626:
	.cfi_offset 14, -24
	xorl	%r14d, %r14d
	jmp	.L597
.L628:
	.cfi_restore 14
	xorl	%eax, %eax
	jmp	.L616
	.cfi_endproc
.LFE57:
	.size	aly_compare, .-aly_compare
	.section	.rodata.str1.1
.LC15:
	.string	"=="
	.text
	.p2align 4
	.globl	aly_eq
	.type	aly_eq, @function
aly_eq:
.LFB58:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	pushq	%r13
	.cfi_offset 13, -24
	leaq	16(%rbp), %r13
	subq	$136, %rsp
	movl	32(%r13), %eax
	movl	0(%r13), %edx
	vmovdqu	0(%r13), %ymm0
	vmovdqu	32(%r13), %ymm1
	cmpl	$3, %eax
	vmovdqu	%ymm0, -144(%rbp)
	vmovdqu	%ymm1, -112(%rbp)
	je	.L650
	cmpl	$3, %edx
	je	.L650
	cmpl	$1, %edx
	vxorps	%xmm1, %xmm1, %xmm1
	je	.L643
	cmpl	$1, %eax
	je	.L655
	vmovdqu	%ymm0, 0(%r13)
	movl	$.LC15, %edi
	vzeroupper
	movq	-8(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	jmp	aly_compare_int
	.p2align 4,,10
	.p2align 3
.L655:
	.cfi_restore_state
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L645
	vcvtsi2sdq	-136(%rbp), %xmm1, %xmm1
	vmovapd	%xmm1, %xmm0
.L645:
	vmovsd	-104(%rbp), %xmm1
	jmp	.L648
	.p2align 4,,10
	.p2align 3
.L650:
	vmovdqu	-144(%rbp), %ymm0
	subq	$32, %rsp
	leaq	-80(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	vmovdqu	-112(%rbp), %ymm0
	leaq	-48(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-40(%rbp), %rsi
	movq	-72(%rbp), %rdi
	addq	$32, %rsp
	xorl	%r13d, %r13d
	call	strcmp
	vmovdqu	-80(%rbp), %ymm0
	testl	%eax, %eax
	sete	%r13b
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	vmovdqu	-48(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movl	%r13d, %eax
	addq	$32, %rsp
	movq	-8(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L643:
	.cfi_restore_state
	cmpl	$3, %eax
	vmovsd	-136(%rbp), %xmm0
	ja	.L646
	testl	%eax, %eax
	jne	.L656
	vcvtsi2sdq	-104(%rbp), %xmm1, %xmm1
.L648:
	xorl	%r13d, %r13d
	vucomisd	%xmm1, %xmm0
	movl	$0, %eax
	setnp	%r13b
	vzeroupper
	cmovne	%eax, %r13d
	movl	%r13d, %eax
	movq	-8(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L656:
	.cfi_restore_state
	cmpl	$1, %eax
	je	.L645
.L646:
	vxorpd	%xmm1, %xmm1, %xmm1
	jmp	.L648
	.cfi_endproc
.LFE58:
	.size	aly_eq, .-aly_eq
	.section	.rodata.str1.1
.LC16:
	.string	">="
	.text
	.p2align 4
	.globl	aly_gte
	.type	aly_gte, @function
aly_gte:
.LFB59:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	pushq	%r13
	.cfi_offset 13, -24
	leaq	16(%rbp), %r13
	subq	$136, %rsp
	movl	32(%r13), %eax
	movl	0(%r13), %edx
	vmovdqu	0(%r13), %ymm0
	vmovdqu	32(%r13), %ymm1
	cmpl	$3, %eax
	vmovdqu	%ymm0, -144(%rbp)
	vmovdqu	%ymm1, -112(%rbp)
	je	.L668
	cmpl	$3, %edx
	je	.L668
	cmpl	$1, %edx
	vxorps	%xmm1, %xmm1, %xmm1
	je	.L661
	cmpl	$1, %eax
	je	.L673
	vmovdqu	%ymm0, 0(%r13)
	movl	$.LC16, %edi
	vzeroupper
	movq	-8(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	jmp	aly_compare_int
	.p2align 4,,10
	.p2align 3
.L673:
	.cfi_restore_state
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L663
	vcvtsi2sdq	-136(%rbp), %xmm1, %xmm1
	vmovapd	%xmm1, %xmm0
.L663:
	vmovsd	-104(%rbp), %xmm1
	jmp	.L666
	.p2align 4,,10
	.p2align 3
.L668:
	vmovdqu	-144(%rbp), %ymm0
	subq	$32, %rsp
	leaq	-80(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	vmovdqu	-112(%rbp), %ymm0
	leaq	-48(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-40(%rbp), %rsi
	movq	-72(%rbp), %rdi
	addq	$32, %rsp
	call	strcmp
	vmovdqu	-80(%rbp), %ymm0
	subq	$32, %rsp
	notl	%eax
	movl	%eax, %r13d
	vmovdqu	%ymm0, (%rsp)
	shrl	$31, %r13d
	vzeroupper
	call	aly_free
	vmovdqu	-48(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movl	%r13d, %eax
	addq	$32, %rsp
	movq	-8(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L661:
	.cfi_restore_state
	cmpl	$3, %eax
	vmovsd	-136(%rbp), %xmm0
	ja	.L664
	testl	%eax, %eax
	jne	.L674
	vcvtsi2sdq	-104(%rbp), %xmm1, %xmm1
.L666:
	xorl	%r13d, %r13d
	vcomisd	%xmm1, %xmm0
	setnb	%r13b
	vzeroupper
	movl	%r13d, %eax
	movq	-8(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L674:
	.cfi_restore_state
	cmpl	$1, %eax
	je	.L663
.L664:
	vxorpd	%xmm1, %xmm1, %xmm1
	jmp	.L666
	.cfi_endproc
.LFE59:
	.size	aly_gte, .-aly_gte
	.section	.rodata.str1.1
.LC17:
	.string	"<="
	.text
	.p2align 4
	.globl	aly_lte
	.type	aly_lte, @function
aly_lte:
.LFB60:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	pushq	%r13
	.cfi_offset 13, -24
	leaq	16(%rbp), %r13
	subq	$136, %rsp
	movl	32(%r13), %eax
	movl	0(%r13), %edx
	vmovdqu	0(%r13), %ymm0
	vmovdqu	32(%r13), %ymm1
	cmpl	$3, %eax
	vmovdqu	%ymm0, -144(%rbp)
	vmovdqu	%ymm1, -112(%rbp)
	je	.L686
	cmpl	$3, %edx
	je	.L686
	cmpl	$1, %edx
	vxorps	%xmm1, %xmm1, %xmm1
	je	.L679
	cmpl	$1, %eax
	je	.L691
	vmovdqu	%ymm0, 0(%r13)
	movl	$.LC17, %edi
	vzeroupper
	movq	-8(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	jmp	aly_compare_int
	.p2align 4,,10
	.p2align 3
.L691:
	.cfi_restore_state
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L681
	vcvtsi2sdq	-136(%rbp), %xmm1, %xmm1
	vmovapd	%xmm1, %xmm0
.L681:
	vmovsd	-104(%rbp), %xmm1
	jmp	.L684
	.p2align 4,,10
	.p2align 3
.L686:
	vmovdqu	-144(%rbp), %ymm0
	subq	$32, %rsp
	leaq	-80(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	vmovdqu	-112(%rbp), %ymm0
	leaq	-48(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-40(%rbp), %rsi
	movq	-72(%rbp), %rdi
	addq	$32, %rsp
	xorl	%r13d, %r13d
	call	strcmp
	vmovdqu	-80(%rbp), %ymm0
	testl	%eax, %eax
	setle	%r13b
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	vmovdqu	-48(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movl	%r13d, %eax
	addq	$32, %rsp
	movq	-8(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L679:
	.cfi_restore_state
	cmpl	$3, %eax
	vmovsd	-136(%rbp), %xmm0
	ja	.L682
	testl	%eax, %eax
	jne	.L692
	vcvtsi2sdq	-104(%rbp), %xmm1, %xmm1
.L684:
	xorl	%r13d, %r13d
	vcomisd	%xmm0, %xmm1
	setnb	%r13b
	vzeroupper
	movl	%r13d, %eax
	movq	-8(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L692:
	.cfi_restore_state
	cmpl	$1, %eax
	je	.L681
.L682:
	vxorpd	%xmm1, %xmm1, %xmm1
	jmp	.L684
	.cfi_endproc
.LFE60:
	.size	aly_lte, .-aly_lte
	.p2align 4
	.globl	aly_print
	.type	aly_print, @function
aly_print:
.LFB61:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$64, %rsp
	movq	%r13, -8(%rbp)
	cmpl	$3, 16(%rbp)
	.cfi_offset 13, -24
	leaq	16(%rbp), %r13
	je	.L714
	vmovdqu	0(%r13), %ymm0
	movq	%rbx, -16(%rbp)
	.cfi_offset 3, -32
	subq	$32, %rsp
	leaq	-48(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-40(%rbp), %r13
	addq	$32, %rsp
	movq	%r13, %rdi
	call	puts
	movl	-48(%rbp), %eax
	movq	-32(%rbp), %rbx
	cmpl	$4, %eax
	je	.L695
	cmpl	$5, %eax
	je	.L696
	cmpl	$3, %eax
	je	.L713
.L698:
	testq	%rbx, %rbx
	je	.L711
.L716:
	movq	-8(%rbp), %r13
	movq	%rbx, %rdi
	movq	-16(%rbp), %rbx
	.cfi_remember_state
	.cfi_restore 3
	leave
	.cfi_def_cfa 7, 8
	jmp	free
	.p2align 4,,10
	.p2align 3
.L696:
	.cfi_restore_state
	movl	12(%r13), %esi
	xorl	%eax, %eax
	movq	0(%r13), %rdi
	testl	%esi, %esi
	jle	.L700
.L699:
	leaq	(%rax,%rax,2), %rdx
	salq	$4, %rdx
	leaq	(%rdi,%rdx), %rcx
	movl	40(%rcx), %r8d
	testl	%r8d, %r8d
	jne	.L715
	addq	$1, %rax
	cmpl	%eax, %esi
	jg	.L699
.L700:
	call	free
.L713:
	movq	%r13, %rdi
	call	free
	testq	%rbx, %rbx
	jne	.L716
.L711:
	movq	-16(%rbp), %rbx
	.cfi_restore 3
	movq	-8(%rbp), %r13
	leave
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L714:
	.cfi_def_cfa 6, 16
	movq	8(%r13), %rdi
	movq	-8(%rbp), %r13
	leave
	.cfi_def_cfa 7, 8
	jmp	puts
	.p2align 4,,10
	.p2align 3
.L695:
	.cfi_def_cfa 6, 16
	.cfi_offset 3, -32
	movl	8(%r13), %r9d
	xorl	%eax, %eax
	testl	%r9d, %r9d
	jle	.L702
.L701:
	movq	%rax, %rdx
	movq	%rax, -56(%rbp)
	subq	$32, %rsp
	salq	$5, %rdx
	addq	0(%r13), %rdx
	vmovdqu	(%rdx), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movq	-56(%rbp), %rax
	addq	$32, %rsp
	addq	$1, %rax
	cmpl	%eax, 8(%r13)
	jg	.L701
.L702:
	movq	0(%r13), %rdi
	call	free
	movq	%r13, %rdi
	call	free
	jmp	.L698
	.p2align 4,,10
	.p2align 3
.L715:
	movq	(%rcx), %rdi
	movq	%rax, -56(%rbp)
	movq	%rdx, -64(%rbp)
	call	free
	movq	0(%r13), %rcx
	movq	-64(%rbp), %rdx
	subq	$32, %rsp
	vmovdqu	8(%rcx,%rdx), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movq	-56(%rbp), %rax
	movl	12(%r13), %esi
	addq	$32, %rsp
	movq	0(%r13), %rdi
	addq	$1, %rax
	cmpl	%eax, %esi
	jg	.L699
	jmp	.L700
	.cfi_endproc
.LFE61:
	.size	aly_print, .-aly_print
	.section	.rodata.str1.1
.LC18:
	.string	"%s"
	.text
	.p2align 4
	.globl	aly_print_raw
	.type	aly_print_raw, @function
aly_print_raw:
.LFB62:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$64, %rsp
	movq	%r13, -8(%rbp)
	cmpl	$3, 16(%rbp)
	.cfi_offset 13, -24
	leaq	16(%rbp), %r13
	je	.L739
	vmovdqu	0(%r13), %ymm0
	movq	%rbx, -16(%rbp)
	.cfi_offset 3, -32
	subq	$32, %rsp
	leaq	-48(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-40(%rbp), %r13
	xorl	%eax, %eax
	addq	$32, %rsp
	movl	$.LC18, %edi
	movq	%r13, %rsi
	call	printf
	movl	-48(%rbp), %eax
	movq	-32(%rbp), %rbx
	cmpl	$4, %eax
	je	.L719
	cmpl	$5, %eax
	je	.L720
	cmpl	$3, %eax
	je	.L738
.L721:
	testq	%rbx, %rbx
	je	.L736
.L741:
	movq	-8(%rbp), %r13
	movq	%rbx, %rdi
	movq	-16(%rbp), %rbx
	.cfi_remember_state
	.cfi_restore 3
	leave
	.cfi_def_cfa 7, 8
	jmp	free
	.p2align 4,,10
	.p2align 3
.L720:
	.cfi_restore_state
	testq	%r13, %r13
	je	.L721
	movl	12(%r13), %esi
	movq	0(%r13), %rdi
	testl	%esi, %esi
	jle	.L724
	xorl	%eax, %eax
.L727:
	leaq	(%rax,%rax,2), %rdx
	salq	$4, %rdx
	leaq	(%rdi,%rdx), %rcx
	movl	40(%rcx), %r8d
	testl	%r8d, %r8d
	jne	.L740
	addq	$1, %rax
	cmpl	%eax, %esi
	jg	.L727
.L724:
	call	free
.L738:
	movq	%r13, %rdi
	call	free
	testq	%rbx, %rbx
	jne	.L741
.L736:
	movq	-16(%rbp), %rbx
	.cfi_restore 3
	movq	-8(%rbp), %r13
	leave
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L739:
	.cfi_def_cfa 6, 16
	movq	8(%r13), %rsi
	movq	-8(%rbp), %r13
	movl	$.LC18, %edi
	xorl	%eax, %eax
	leave
	.cfi_def_cfa 7, 8
	jmp	printf
	.p2align 4,,10
	.p2align 3
.L719:
	.cfi_def_cfa 6, 16
	.cfi_offset 3, -32
	testq	%r13, %r13
	je	.L721
	movl	8(%r13), %r9d
	testl	%r9d, %r9d
	jle	.L722
	xorl	%eax, %eax
.L723:
	movq	%rax, %rdx
	movq	%rax, -56(%rbp)
	subq	$32, %rsp
	salq	$5, %rdx
	addq	0(%r13), %rdx
	vmovdqu	(%rdx), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movq	-56(%rbp), %rax
	addq	$32, %rsp
	addq	$1, %rax
	cmpl	%eax, 8(%r13)
	jg	.L723
.L722:
	movq	0(%r13), %rdi
	call	free
	movq	%r13, %rdi
	call	free
	jmp	.L721
	.p2align 4,,10
	.p2align 3
.L740:
	movq	(%rcx), %rdi
	movq	%rax, -56(%rbp)
	movq	%rdx, -64(%rbp)
	call	free
	movq	0(%r13), %rcx
	movq	-64(%rbp), %rdx
	subq	$32, %rsp
	vmovdqu	8(%rcx,%rdx), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movq	-56(%rbp), %rax
	movl	12(%r13), %esi
	addq	$32, %rsp
	movq	0(%r13), %rdi
	addq	$1, %rax
	cmpl	%eax, %esi
	jg	.L727
	jmp	.L724
	.cfi_endproc
.LFE62:
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
.LFB63:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movq	%rdi, %rbx
	subq	$4096, %rsp
	.cfi_def_cfa_offset 4112
	testq	%rsi, %rsi
	je	.L743
	movl	$.LC18, %edi
	xorl	%eax, %eax
	call	printf
.L743:
	movq	stdin(%rip), %rdx
	movl	$4096, %esi
	movq	%rsp, %rdi
	call	fgets
	testq	%rax, %rax
	je	.L744
	movq	%rsp, %rdi
	movl	$.LC19, %esi
	call	strcspn
	movq	%rsp, %rdi
	movb	$0, (%rsp,%rax)
	call	strdup
.L745:
	movq	%rax, 8(%rbx)
	movq	%rbx, %rax
	movl	$3, (%rbx)
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	addq	$4096, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 16
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L744:
	.cfi_restore_state
	movl	$.LC20, %edi
	call	strdup
	jmp	.L745
	.cfi_endproc
.LFE63:
	.size	aly_input, .-aly_input
	.p2align 4
	.globl	native_input
	.type	native_input, @function
native_input:
.LFB109:
	.cfi_startproc
	testl	%esi, %esi
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movq	%rdi, %rbx
	jg	.L754
	movl	$.LC20, %esi
	call	aly_input
	movq	%rbx, %rax
	popq	%rbx
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L754:
	.cfi_restore_state
	movq	8(%rdx), %rsi
	call	aly_input
	movq	%rbx, %rax
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE109:
	.size	native_input, .-native_input
	.p2align 4
	.globl	aly_tomb
	.type	aly_tomb, @function
aly_tomb:
.LFB64:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rax
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	cmpl	$8, 16(%rbp)
	movq	24(%rbp), %rdx
	jne	.L756
	testq	%rdx, %rdx
	jne	.L764
.L756:
	vmovdqu	16(%rbp), %ymm0
	vmovdqu	%ymm0, (%rax)
	vzeroupper
	popq	%rbp
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L764:
	.cfi_restore_state
	vmovdqu	16(%rbp), %ymm0
	movl	$0, 24(%rdx)
	vmovdqu	%ymm0, (%rax)
	vzeroupper
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE64:
	.size	aly_tomb, .-aly_tomb
	.p2align 4
	.globl	aly_array_new
	.type	aly_array_new, @function
aly_array_new:
.LFB65:
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
	jle	.L767
	movl	%ebx, %eax
	movslq	%ebx, %rdi
.L766:
	movl	%eax, 12(%rbp)
	movl	$32, %esi
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
.L767:
	.cfi_restore_state
	movl	$8, %edi
	movl	$8, %eax
	jmp	.L766
	.cfi_endproc
.LFE65:
	.size	aly_array_new, .-aly_array_new
	.p2align 4
	.globl	aly_array_init
	.type	aly_array_init, @function
aly_array_init:
.LFB66:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	addq	$-128, %rsp
	movq	%rdi, (%rsp)
	movl	$1, %edi
	movq	%r15, -8(%rbp)
	.cfi_offset 15, -24
	movl	%esi, %r15d
	movl	$16, %esi
	movq	%r12, -32(%rbp)
	.cfi_offset 12, -48
	movq	%rdx, %r12
	movq	%r14, -16(%rbp)
	.cfi_offset 14, -32
	call	calloc
	testl	%r15d, %r15d
	movq	%rax, %r14
	jle	.L783
	movq	%rbx, -40(%rbp)
	movl	$32, %esi
	movq	%r15, %rdi
	.cfi_offset 3, -56
	movl	$1, %ebx
	movq	%r13, -24(%rbp)
	.cfi_offset 13, -40
	movl	%r15d, 12(%rax)
	call	calloc
	movl	$0, 8(%r14)
	movq	%rax, (%r14)
	movq	%rax, %rsi
	jmp	.L779
	.p2align 4,,10
	.p2align 3
.L786:
	testl	%r13d, %r13d
	je	.L784
	vmovsd	24(%rsp), %xmm0
	vmovsd	%xmm0, 56(%rsp)
.L777:
	movl	$1, %eax
	xorl	%edx, %edx
.L778:
	movl	%eax, 72(%rsp)
	movq	%rbx, %rax
	addq	$32, %r12
	movl	%r13d, 48(%rsp)
	salq	$5, %rax
	cmpq	%rbx, %r15
	movq	%rdx, 64(%rsp)
	vmovdqu	48(%rsp), %ymm0
	movl	%ebx, 8(%r14)
	vmovdqu	%ymm0, -32(%rsi,%rax)
	je	.L785
	addq	$1, %rbx
.L779:
	movl	(%r12), %r13d
	vmovdqu	(%r12), %ymm0
	cmpl	$2, %r13d
	vmovdqu	%ymm0, 16(%rsp)
	je	.L772
	jbe	.L786
	cmpl	$3, %r13d
	jne	.L776
	movq	%rsi, 8(%rsp)
	movq	24(%rsp), %rdi
	vzeroupper
	call	strdup
	movq	8(%rsp), %rsi
	movq	%rax, 56(%rsp)
	jmp	.L777
	.p2align 4,,10
	.p2align 3
.L772:
	movl	24(%rsp), %eax
	movl	%eax, 56(%rsp)
	jmp	.L777
	.p2align 4,,10
	.p2align 3
.L776:
	movq	16(%r12), %rdx
	movl	24(%r12), %eax
	vmovdqu	%ymm0, 48(%rsp)
	jmp	.L778
	.p2align 4,,10
	.p2align 3
.L784:
	movq	24(%rsp), %rax
	movq	%rax, 56(%rsp)
	jmp	.L777
	.p2align 4,,10
	.p2align 3
.L785:
	vzeroupper
	movq	-40(%rbp), %rbx
	.cfi_restore 3
	movq	-24(%rbp), %r13
	.cfi_restore 13
.L771:
	movq	(%rsp), %rax
	movq	-32(%rbp), %r12
	movq	-8(%rbp), %r15
	movq	%r14, 8(%rax)
	movq	-16(%rbp), %r14
	movl	$4, (%rax)
	movq	$0, 16(%rax)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L783:
	.cfi_restore_state
	movl	$32, %esi
	movl	$8, %edi
	call	calloc
	movq	%rax, (%r14)
	movabsq	$34359738368, %rax
	movq	%rax, 8(%r14)
	jmp	.L771
	.cfi_endproc
.LFE66:
	.size	aly_array_init, .-aly_array_init
	.p2align 4
	.globl	aly_array_push
	.type	aly_array_push, @function
aly_array_push:
.LFB74:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$80, %rsp
	cmpl	$4, 16(%rbp)
	movq	24(%rbp), %rdx
	jne	.L798
	testq	%rdx, %rdx
	je	.L798
	movslq	8(%rdx), %rcx
	movl	12(%rdx), %eax
	movq	(%rdx), %rdi
	cmpl	%eax, %ecx
	jge	.L800
.L789:
	leal	1(%rcx), %eax
	salq	$5, %rcx
	vmovdqu	48(%rbp), %ymm0
	movl	%eax, 8(%rdx)
	leaq	(%rdi,%rcx), %rdx
	movl	48(%rbp), %ecx
	vmovdqu	%ymm0, 16(%rsp)
	cmpl	$2, %ecx
	je	.L790
	ja	.L791
	testl	%ecx, %ecx
	je	.L801
	vmovsd	24(%rsp), %xmm0
	vmovsd	%xmm0, 56(%rsp)
.L795:
	movl	$1, %eax
	xorl	%esi, %esi
.L796:
	movl	%ecx, 48(%rsp)
	movq	%rsi, 64(%rsp)
	movl	%eax, 72(%rsp)
	vmovdqu	48(%rsp), %ymm0
	vmovdqu	%ymm0, (%rdx)
	vzeroupper
.L798:
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L801:
	.cfi_restore_state
	movq	24(%rsp), %rax
	movq	%rax, 56(%rsp)
	jmp	.L795
	.p2align 4,,10
	.p2align 3
.L800:
	addl	%eax, %eax
	movq	%rdx, 8(%rsp)
	movl	%eax, 12(%rdx)
	movslq	%eax, %rsi
	salq	$5, %rsi
	call	realloc
	movq	8(%rsp), %rdx
	movq	%rax, %rdi
	movq	%rax, (%rdx)
	movslq	8(%rdx), %rcx
	jmp	.L789
	.p2align 4,,10
	.p2align 3
.L791:
	cmpl	$3, %ecx
	jne	.L794
	movl	%ecx, 4(%rsp)
	movq	24(%rsp), %rdi
	movq	%rdx, 8(%rsp)
	vzeroupper
	call	strdup
	movq	8(%rsp), %rdx
	movl	4(%rsp), %ecx
	movq	%rax, 56(%rsp)
	jmp	.L795
	.p2align 4,,10
	.p2align 3
.L790:
	movl	24(%rsp), %eax
	movl	%eax, 56(%rsp)
	jmp	.L795
	.p2align 4,,10
	.p2align 3
.L794:
	movq	64(%rbp), %rsi
	movl	72(%rbp), %eax
	vmovdqu	%ymm0, 48(%rsp)
	jmp	.L796
	.cfi_endproc
.LFE74:
	.size	aly_array_push, .-aly_array_push
	.p2align 4
	.globl	aly_object_new
	.type	aly_object_new, @function
aly_object_new:
.LFB75:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movl	$16, %esi
	movl	$1, %edi
	call	calloc
	movl	$48, %esi
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
.LFE75:
	.size	aly_object_new, .-aly_object_new
	.section	.rodata.str1.8,"aMS",@progbits,1
	.align 8
.LC21:
	.base64	"VHlwZUVycm9yOiBBIHZhcmnDoXZlbCAnJXMnIMOpIGNvbnN0YW50ZS9pbXV0w6F2ZWwsIG7Do28gw6kgcG9zc8OtdmVsIGFsdGVyYXIgc2V1IHZhbG9yLgoA"
	.text
	.p2align 4
	.globl	aly_object_set
	.type	aly_object_set, @function
aly_object_set:
.LFB77:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$144, %rsp
	movq	%r10, -40(%rbp)
	movq	24(%rbp), %rax
	.cfi_offset 10, -56
	leaq	16(%rbp), %r10
	cmpl	$5, (%r10)
	movq	%rax, -136(%rbp)
	jne	.L831
	testq	%rax, %rax
	je	.L831
	movq	%r15, -8(%rbp)
	movzbl	(%rdi), %edx
	testb	%dl, %dl
	.cfi_offset 15, -24
	je	.L826
	movq	%rdi, %rcx
	movl	$5381, %eax
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L807:
	movl	%eax, %esi
	addq	$1, %rcx
	sall	$5, %esi
	addl	%esi, %eax
	addl	%edx, %eax
	movzbl	(%rcx), %edx
	testb	%dl, %dl
	jne	.L807
.L806:
	movq	-136(%rbp), %rcx
	movl	12(%rcx), %r15d
	testl	%r15d, %r15d
	jle	.L833
	xorl	%edx, %edx
	movq	%r12, -32(%rbp)
	movq	(%rcx), %rcx
	divl	%r15d
	movq	%r14, -16(%rbp)
	movq	%rdi, -128(%rbp)
	movq	%r10, -144(%rbp)
	movq	%rbx, -48(%rbp)
	movq	%r13, -24(%rbp)
	.cfi_offset 12, -48
	.cfi_offset 14, -32
	.cfi_offset 3, -64
	.cfi_offset 13, -40
	movl	%edx, %ebx
	leal	(%r15,%rdx), %r13d
	.p2align 4,,10
	.p2align 3
.L825:
	xorl	%edx, %edx
	movl	%ebx, %eax
	divl	%r15d
	leaq	(%rdx,%rdx,2), %r14
	salq	$4, %r14
	leaq	(%rcx,%r14), %r12
	movl	40(%r12), %esi
	testl	%esi, %esi
	jne	.L835
	movq	-128(%rbp), %rdi
	call	strdup
	movq	%rax, (%r12)
	movq	-144(%rbp), %rax
	movl	32(%rax), %ebx
	vmovdqu	32(%rax), %ymm0
	cmpl	$2, %ebx
	vmovdqu	%ymm0, -112(%rbp)
	je	.L818
	ja	.L819
	testl	%ebx, %ebx
	je	.L836
	vmovsd	-104(%rbp), %xmm0
	vmovsd	%xmm0, -72(%rbp)
.L823:
	movl	$1, %eax
	xorl	%ecx, %ecx
.L824:
	movl	%eax, -56(%rbp)
	movq	-136(%rbp), %rax
	movl	%ebx, -80(%rbp)
	movq	%rcx, -64(%rbp)
	vmovdqu	-80(%rbp), %ymm0
	movl	$1, 40(%r12)
	vmovdqu	%ymm0, 8(%r12)
	addl	$1, 8(%rax)
	vzeroupper
.L834:
	movq	-48(%rbp), %rbx
	.cfi_restore 3
	movq	-32(%rbp), %r12
	.cfi_restore 12
	movq	-24(%rbp), %r13
	.cfi_restore 13
	movq	-16(%rbp), %r14
	.cfi_restore 14
	movq	-8(%rbp), %r15
	.cfi_restore 15
.L831:
	leave
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L835:
	.cfi_def_cfa 6, 16
	.cfi_offset 3, -64
	.cfi_offset 12, -48
	.cfi_offset 13, -40
	.cfi_offset 14, -32
	.cfi_offset 15, -24
	movq	(%r12), %rdi
	movq	-128(%rbp), %rsi
	movq	%rcx, -120(%rbp)
	call	strcmp
	movq	-120(%rbp), %rcx
	testl	%eax, %eax
	je	.L837
	addl	$1, %ebx
	cmpl	%r13d, %ebx
	jne	.L825
	jmp	.L834
	.p2align 4,,10
	.p2align 3
.L837:
	movl	32(%r12), %edx
	testl	%edx, %edx
	je	.L838
	vmovdqu	8(%r12), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movq	-136(%rbp), %rax
	addq	$32, %rsp
	addq	(%rax), %r14
	movq	-144(%rbp), %rax
	movl	32(%rax), %ebx
	vmovdqu	32(%rax), %ymm0
	cmpl	$2, %ebx
	vmovdqu	%ymm0, -112(%rbp)
	je	.L811
	ja	.L812
	testl	%ebx, %ebx
	je	.L839
	vmovsd	-104(%rbp), %xmm0
	vmovsd	%xmm0, -72(%rbp)
.L816:
	movl	$1, %eax
	xorl	%edx, %edx
.L817:
	movl	%ebx, -80(%rbp)
	movq	%rdx, -64(%rbp)
	movl	%eax, -56(%rbp)
	vmovdqu	-80(%rbp), %ymm0
	vmovdqu	%ymm0, 8(%r14)
	vzeroupper
	jmp	.L834
.L818:
	movl	-104(%rbp), %eax
	movl	%eax, -72(%rbp)
	jmp	.L823
.L836:
	movq	-104(%rbp), %rax
	movq	%rax, -72(%rbp)
	jmp	.L823
.L812:
	cmpl	$3, %ebx
	jne	.L815
	movq	-104(%rbp), %rdi
	vzeroupper
	call	strdup
	movq	%rax, -72(%rbp)
	jmp	.L816
	.p2align 4,,10
	.p2align 3
.L819:
	cmpl	$3, %ebx
	jne	.L822
	movq	-104(%rbp), %rdi
	vzeroupper
	call	strdup
	movq	%rax, -72(%rbp)
	jmp	.L823
.L826:
	.cfi_restore 3
	.cfi_restore 12
	.cfi_restore 13
	.cfi_restore 14
	movl	$5381, %eax
	jmp	.L806
.L822:
	.cfi_offset 3, -64
	.cfi_offset 12, -48
	.cfi_offset 13, -40
	.cfi_offset 14, -32
	movq	48(%rax), %rcx
	vmovdqu	%ymm0, -80(%rbp)
	movl	56(%rax), %eax
	jmp	.L824
.L811:
	movl	-104(%rbp), %eax
	movl	%eax, -72(%rbp)
	jmp	.L816
.L839:
	movq	-104(%rbp), %rax
	movq	%rax, -72(%rbp)
	jmp	.L816
.L815:
	movq	48(%rax), %rdx
	vmovdqu	%ymm0, -80(%rbp)
	movl	56(%rax), %eax
	jmp	.L817
.L833:
	.cfi_restore 3
	.cfi_restore 12
	.cfi_restore 13
	.cfi_restore 14
	movq	-8(%rbp), %r15
	.cfi_restore 15
	leave
	.cfi_def_cfa 7, 8
	ret
.L838:
	.cfi_def_cfa 6, 16
	.cfi_offset 3, -64
	.cfi_offset 12, -48
	.cfi_offset 13, -40
	.cfi_offset 14, -32
	.cfi_offset 15, -24
	movq	stderr(%rip), %rdi
	movq	-128(%rbp), %rdx
	movl	$.LC21, %esi
	call	fprintf
	movl	$1, %edi
	call	exit
	.cfi_endproc
.LFE77:
	.size	aly_object_set, .-aly_object_set
	.p2align 4
	.globl	aly_object_get_ptr
	.type	aly_object_get_ptr, @function
aly_object_get_ptr:
.LFB78:
	.cfi_startproc
	subq	$72, %rsp
	.cfi_def_cfa_offset 80
	movq	88(%rsp), %rax
	cmpl	$5, 80(%rsp)
	movq	%rax, 8(%rsp)
	jne	.L846
	testq	%rax, %rax
	je	.L846
	movq	%rbp, 32(%rsp)
	movzbl	(%rdi), %edx
	testb	%dl, %dl
	.cfi_offset 6, -48
	je	.L850
	movq	%rdi, %rcx
	movl	$5381, %eax
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L845:
	movl	%eax, %esi
	addq	$1, %rcx
	sall	$5, %esi
	addl	%esi, %eax
	addl	%edx, %eax
	movzbl	(%rcx), %edx
	testb	%dl, %dl
	jne	.L845
.L844:
	movq	8(%rsp), %rcx
	movl	12(%rcx), %ebp
	testl	%ebp, %ebp
	jle	.L854
	xorl	%edx, %edx
	movq	%r12, 40(%rsp)
	.cfi_offset 12, -40
	movq	%rdi, %r12
	divl	%ebp
	movq	%r13, 48(%rsp)
	.cfi_offset 13, -32
	movq	(%rcx), %r13
	movq	%r14, 56(%rsp)
	movq	%r15, 64(%rsp)
	movq	%rbx, 24(%rsp)
	.cfi_offset 14, -24
	.cfi_offset 15, -16
	.cfi_offset 3, -56
	movl	%edx, %r15d
	leal	0(%rbp,%rdx), %r14d
	jmp	.L849
	.p2align 4,,10
	.p2align 3
.L847:
	movq	(%rbx), %rdi
	movq	%r12, %rsi
	call	strcmp
	testl	%eax, %eax
	je	.L855
	addl	$1, %r15d
	cmpl	%r14d, %r15d
	je	.L856
.L849:
	xorl	%edx, %edx
	movl	%r15d, %eax
	divl	%ebp
	leaq	(%rdx,%rdx,2), %rbx
	salq	$4, %rbx
	addq	%r13, %rbx
	movl	40(%rbx), %eax
	testl	%eax, %eax
	jne	.L847
	movq	%r12, %rdi
	call	strdup
	movl	$6, 8(%rbx)
	movq	%rax, (%rbx)
	movq	8(%rsp), %rax
	movq	$0, 16(%rbx)
	movq	$0, 24(%rbx)
	movl	$1, 32(%rbx)
	movl	$1, 40(%rbx)
	addl	$1, 8(%rax)
.L855:
	leaq	8(%rbx), %rax
	movq	32(%rsp), %rbp
	.cfi_remember_state
	.cfi_restore 6
	movq	24(%rsp), %rbx
	.cfi_restore 3
	movq	40(%rsp), %r12
	.cfi_restore 12
	movq	48(%rsp), %r13
	.cfi_restore 13
	movq	56(%rsp), %r14
	.cfi_restore 14
	movq	64(%rsp), %r15
	.cfi_restore 15
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L856:
	.cfi_restore_state
	movq	24(%rsp), %rbx
	.cfi_restore 3
	movq	32(%rsp), %rbp
	.cfi_restore 6
	movq	40(%rsp), %r12
	.cfi_restore 12
	movq	48(%rsp), %r13
	.cfi_restore 13
	movq	56(%rsp), %r14
	.cfi_restore 14
	movq	64(%rsp), %r15
	.cfi_restore 15
.L846:
	xorl	%eax, %eax
	addq	$72, %rsp
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L850:
	.cfi_def_cfa_offset 80
	.cfi_offset 6, -48
	movl	$5381, %eax
	jmp	.L844
.L854:
	movq	32(%rsp), %rbp
	.cfi_restore 6
	jmp	.L846
	.cfi_endproc
.LFE78:
	.size	aly_object_get_ptr, .-aly_object_get_ptr
	.p2align 4
	.globl	aly_object_get
	.type	aly_object_get, @function
aly_object_get:
.LFB79:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$96, %rsp
	cmpl	$5, 16(%rbp)
	movq	%r12, -32(%rbp)
	.cfi_offset 12, -48
	movq	%rdi, %r12
	movq	24(%rbp), %rdi
	jne	.L858
	testq	%rdi, %rdi
	je	.L858
	movq	%r13, -24(%rbp)
	movq	%rsi, %rcx
	.cfi_offset 13, -40
	movq	%rsi, %r13
	movl	$5381, %eax
	movq	%rbx, -40(%rbp)
	movzbl	(%rsi), %edx
	testb	%dl, %dl
	.cfi_offset 3, -56
	je	.L860
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L859:
	movl	%eax, %esi
	addq	$1, %rcx
	sall	$5, %esi
	addl	%esi, %eax
	addl	%edx, %eax
	movzbl	(%rcx), %edx
	testb	%dl, %dl
	jne	.L859
.L860:
	movl	12(%rdi), %ebx
	testl	%ebx, %ebx
	jle	.L876
	xorl	%edx, %edx
	movq	%r14, -16(%rbp)
	divl	%ebx
	movq	(%rdi), %rax
	movq	%r15, -8(%rbp)
	.cfi_offset 14, -32
	.cfi_offset 15, -24
	movq	%rax, 8(%rsp)
	leal	(%rbx,%rdx), %eax
	movl	%edx, %r14d
	movl	%eax, 4(%rsp)
	jmp	.L869
	.p2align 4,,10
	.p2align 3
.L880:
	movq	(%r15), %rdi
	movq	%r13, %rsi
	call	strcmp
	testl	%eax, %eax
	je	.L879
	addl	$1, %r14d
	cmpl	4(%rsp), %r14d
	je	.L877
.L869:
	xorl	%edx, %edx
	movl	%r14d, %eax
	divl	%ebx
	movq	8(%rsp), %rax
	leaq	(%rdx,%rdx,2), %rdx
	salq	$4, %rdx
	leaq	(%rax,%rdx), %r15
	movl	40(%r15), %eax
	testl	%eax, %eax
	jne	.L880
.L877:
	movq	-40(%rbp), %rbx
	.cfi_restore 3
	movq	-24(%rbp), %r13
	.cfi_restore 13
	movq	-16(%rbp), %r14
	.cfi_restore 14
	movq	-8(%rbp), %r15
	.cfi_restore 15
.L858:
	movl	$6, (%r12)
	movq	$0, 8(%r12)
	movq	$0, 16(%r12)
	movl	$1, 24(%r12)
.L857:
	movq	%r12, %rax
	movq	-32(%rbp), %r12
	leave
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L879:
	.cfi_def_cfa 6, 16
	.cfi_offset 3, -56
	.cfi_offset 13, -40
	.cfi_offset 14, -32
	.cfi_offset 15, -24
	movl	8(%r15), %eax
	vmovdqu	8(%r15), %ymm0
	cmpl	$2, %eax
	vmovdqu	%ymm0, 16(%rsp)
	je	.L863
	ja	.L864
	testl	%eax, %eax
	je	.L881
	vmovsd	24(%rsp), %xmm0
	movl	$1, (%r12)
	movq	$0, 16(%r12)
	movl	$1, 24(%r12)
	vmovsd	%xmm0, 8(%r12)
.L878:
	vzeroupper
	movq	%r12, %rax
	movq	-40(%rbp), %rbx
	.cfi_remember_state
	.cfi_restore 3
	movq	-24(%rbp), %r13
	.cfi_restore 13
	movq	-16(%rbp), %r14
	.cfi_restore 14
	movq	-8(%rbp), %r15
	.cfi_restore 15
	movq	-32(%rbp), %r12
	leave
	.cfi_def_cfa 7, 8
	ret
.L881:
	.cfi_restore_state
	movq	24(%rsp), %rax
	movl	$0, (%r12)
	movq	$0, 16(%r12)
	movq	%rax, 8(%r12)
	movl	$1, 24(%r12)
	jmp	.L878
	.p2align 4,,10
	.p2align 3
.L864:
	cmpl	$3, %eax
	jne	.L867
	movq	24(%rsp), %rdi
	vzeroupper
	call	strdup
	movq	-40(%rbp), %rbx
	.cfi_remember_state
	.cfi_restore 3
	movq	-24(%rbp), %r13
	.cfi_restore 13
	movl	$3, (%r12)
	movq	-16(%rbp), %r14
	.cfi_restore 14
	movq	-8(%rbp), %r15
	.cfi_restore 15
	movq	%rax, 8(%r12)
	movq	$0, 16(%r12)
	movl	$1, 24(%r12)
	jmp	.L857
.L863:
	.cfi_restore_state
	movl	24(%rsp), %eax
	movl	$2, (%r12)
	movq	$0, 16(%r12)
	movl	%eax, 8(%r12)
	movl	$1, 24(%r12)
	jmp	.L878
.L867:
	vmovdqu	%ymm0, (%r12)
	jmp	.L878
.L876:
	.cfi_restore 14
	.cfi_restore 15
	movq	-40(%rbp), %rbx
	.cfi_restore 3
	movq	-24(%rbp), %r13
	.cfi_restore 13
	jmp	.L858
	.cfi_endproc
.LFE79:
	.size	aly_object_get, .-aly_object_get
	.p2align 4
	.globl	aly_len
	.type	aly_len, @function
aly_len:
.LFB80:
	.cfi_startproc
	subq	$8, %rsp
	.cfi_def_cfa_offset 16
	movl	16(%rsp), %eax
	movq	24(%rsp), %rdi
	cmpl	$4, %eax
	je	.L883
	cmpl	$5, %eax
	je	.L883
	cmpl	$3, %eax
	jne	.L885
	testq	%rdi, %rdi
	je	.L885
	call	strlen
	addq	$8, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L883:
	.cfi_restore_state
	testq	%rdi, %rdi
	je	.L885
	movl	8(%rdi), %eax
	addq	$8, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L885:
	.cfi_restore_state
	xorl	%eax, %eax
	addq	$8, %rsp
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE80:
	.size	aly_len, .-aly_len
	.p2align 4
	.globl	aly_type_of
	.type	aly_type_of, @function
aly_type_of:
.LFB81:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movl	16(%rsp), %eax
	movq	%rdi, %rbx
	movl	$.LC1, %edi
	cmpl	$8, %eax
	ja	.L897
	movq	CSWTCH.89(,%rax,8), %rdi
.L897:
	call	strdup
	movl	$3, (%rbx)
	movq	%rax, 8(%rbx)
	movq	%rbx, %rax
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE81:
	.size	aly_type_of, .-aly_type_of
	.p2align 4
	.globl	aly_pow
	.type	aly_pow, @function
aly_pow:
.LFB82:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movq	%rdi, %rbx
	vxorps	%xmm2, %xmm2, %xmm2
	subq	$16, %rsp
	.cfi_def_cfa_offset 32
	movl	64(%rsp), %eax
	movq	72(%rsp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm1
	je	.L904
	cmpl	$3, %eax
	je	.L903
	testl	%eax, %eax
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L904
	vcvtsi2sdq	%rdi, %xmm2, %xmm1
.L904:
	movl	32(%rsp), %eax
	movq	40(%rsp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm0
	je	.L907
	cmpl	$3, %eax
	je	.L906
	testl	%eax, %eax
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L907
	vcvtsi2sdq	%rdi, %xmm2, %xmm2
	vmovapd	%xmm2, %xmm0
.L907:
	call	pow
	movl	$1, (%rbx)
	movq	%rbx, %rax
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	vmovsd	%xmm0, 8(%rbx)
	addq	$16, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 16
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L903:
	.cfi_restore_state
	xorl	%esi, %esi
	call	strtod
	vxorps	%xmm2, %xmm2, %xmm2
	vmovapd	%xmm0, %xmm1
	jmp	.L904
	.p2align 4,,10
	.p2align 3
.L906:
	xorl	%esi, %esi
	vmovsd	%xmm1, 8(%rsp)
	call	strtod
	vmovsd	8(%rsp), %xmm1
	jmp	.L907
	.cfi_endproc
.LFE82:
	.size	aly_pow, .-aly_pow
	.p2align 4
	.globl	aly_sqrt
	.type	aly_sqrt, @function
aly_sqrt:
.LFB83:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L912
	cmpl	$3, %edx
	je	.L913
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L914
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
.L915:
	vxorpd	%xmm1, %xmm1, %xmm1
	vucomisd	%xmm0, %xmm1
	ja	.L920
	vsqrtsd	%xmm0, %xmm0, %xmm0
.L914:
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	vmovsd	%xmm0, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L913:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rax
	jmp	.L915
	.p2align 4,,10
	.p2align 3
.L912:
	vmovq	%rdi, %xmm0
	jmp	.L915
.L920:
	movq	%rax, 8(%rsp)
	call	sqrt
	movq	8(%rsp), %rax
	jmp	.L914
	.cfi_endproc
.LFE83:
	.size	aly_sqrt, .-aly_sqrt
	.p2align 4
	.globl	aly_round
	.type	aly_round, @function
aly_round:
.LFB84:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L923
	cmpl	$3, %edx
	je	.L924
	xorl	%ecx, %ecx
	testl	%edx, %edx
	jne	.L925
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
.L925:
	movl	$0, (%rax)
	movq	%rcx, 8(%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L924:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	call	round
	movq	8(%rsp), %rax
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L925
	.p2align 4,,10
	.p2align 3
.L923:
	vmovq	%rdi, %xmm0
	movq	%rax, 8(%rsp)
	call	round
	movq	8(%rsp), %rax
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L925
	.cfi_endproc
.LFE84:
	.size	aly_round, .-aly_round
	.p2align 4
	.globl	aly_round_up
	.type	aly_round_up, @function
aly_round_up:
.LFB85:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L929
	cmpl	$3, %edx
	je	.L930
	xorl	%ecx, %ecx
	testl	%edx, %edx
	jne	.L931
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vroundsd	$10, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
.L931:
	movl	$0, (%rax)
	movq	%rcx, 8(%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L930:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rax
	vroundsd	$10, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L931
	.p2align 4,,10
	.p2align 3
.L929:
	vmovq	%rdi, %xmm0
	vroundsd	$10, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L931
	.cfi_endproc
.LFE85:
	.size	aly_round_up, .-aly_round_up
	.p2align 4
	.globl	aly_round_down
	.type	aly_round_down, @function
aly_round_down:
.LFB86:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L935
	cmpl	$3, %edx
	je	.L936
	xorl	%ecx, %ecx
	testl	%edx, %edx
	jne	.L937
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	vroundsd	$9, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
.L937:
	movl	$0, (%rax)
	movq	%rcx, 8(%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L936:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rax
	vroundsd	$9, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L937
	.p2align 4,,10
	.p2align 3
.L935:
	vmovq	%rdi, %xmm0
	vroundsd	$9, %xmm0, %xmm0, %xmm0
	vcvttsd2siq	%xmm0, %rcx
	jmp	.L937
	.cfi_endproc
.LFE86:
	.size	aly_round_down, .-aly_round_down
	.p2align 4
	.globl	aly_abs
	.type	aly_abs, @function
aly_abs:
.LFB87:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rcx
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	movl	16(%rbp), %eax
	vmovsd	24(%rbp), %xmm0
	cmpl	$1, %eax
	je	.L948
	vmovdqu	16(%rbp), %ymm1
	cmpl	$2, %eax
	vmovdqu	%ymm1, 16(%rsp)
	je	.L943
	ja	.L949
	vmovq	%xmm0, %rdi
	movq	%rdi, %rax
	negq	%rax
	cmovs	%rdi, %rax
	vzeroupper
.L946:
	movl	$0, (%rcx)
	movq	%rax, 8(%rcx)
	jmp	.L942
	.p2align 4,,10
	.p2align 3
.L948:
	vandpd	.LC22(%rip), %xmm0, %xmm0
	movl	$1, (%rdi)
	vmovsd	%xmm0, 8(%rdi)
.L942:
	movq	$0, 16(%rcx)
	movq	%rcx, %rax
	movl	$1, 24(%rcx)
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L949:
	.cfi_restore_state
	cmpl	$3, %eax
	jne	.L950
	movq	%rdi, 8(%rsp)
	xorl	%esi, %esi
	movl	$10, %edx
	vmovq	%xmm0, %rdi
	vzeroupper
	call	strtoll
	movq	8(%rsp), %rcx
	movq	%rax, %rdx
	negq	%rax
	cmovs	%rdx, %rax
	jmp	.L946
	.p2align 4,,10
	.p2align 3
.L943:
	movl	24(%rsp), %edx
	xorl	%eax, %eax
	testl	%edx, %edx
	setne	%al
	vzeroupper
	jmp	.L946
	.p2align 4,,10
	.p2align 3
.L950:
	xorl	%eax, %eax
	vzeroupper
	jmp	.L946
	.cfi_endproc
.LFE87:
	.size	aly_abs, .-aly_abs
	.p2align 4
	.globl	aly_random
	.type	aly_random, @function
aly_random:
.LFB88:
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
	vdivsd	.LC23(%rip), %xmm0, %xmm0
	movl	$1, 24(%rbx)
	vmovsd	%xmm0, 8(%rbx)
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE88:
	.size	aly_random, .-aly_random
	.p2align 4
	.globl	aly_sin
	.type	aly_sin, @function
aly_sin:
.LFB89:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L954
	cmpl	$3, %edx
	je	.L955
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L956
	vxorps	%xmm0, %xmm0, %xmm0
	movq	%rax, 8(%rsp)
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	call	sin
	movq	8(%rsp), %rax
.L956:
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	vmovsd	%xmm0, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L955:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	call	sin
	movq	8(%rsp), %rax
	jmp	.L956
	.p2align 4,,10
	.p2align 3
.L954:
	vmovq	%rdi, %xmm0
	movq	%rax, 8(%rsp)
	call	sin
	movq	8(%rsp), %rax
	jmp	.L956
	.cfi_endproc
.LFE89:
	.size	aly_sin, .-aly_sin
	.p2align 4
	.globl	aly_cos
	.type	aly_cos, @function
aly_cos:
.LFB90:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L960
	cmpl	$3, %edx
	je	.L961
	testl	%edx, %edx
	vmovsd	.LC24(%rip), %xmm0
	jne	.L962
	vxorps	%xmm0, %xmm0, %xmm0
	movq	%rax, 8(%rsp)
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	call	cos
	movq	8(%rsp), %rax
.L962:
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	vmovsd	%xmm0, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L961:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	call	cos
	movq	8(%rsp), %rax
	jmp	.L962
	.p2align 4,,10
	.p2align 3
.L960:
	vmovq	%rdi, %xmm0
	movq	%rax, 8(%rsp)
	call	cos
	movq	8(%rsp), %rax
	jmp	.L962
	.cfi_endproc
.LFE90:
	.size	aly_cos, .-aly_cos
	.p2align 4
	.globl	aly_tan
	.type	aly_tan, @function
aly_tan:
.LFB91:
	.cfi_startproc
	subq	$24, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rax
	movl	32(%rsp), %edx
	movq	40(%rsp), %rdi
	cmpl	$1, %edx
	je	.L966
	cmpl	$3, %edx
	je	.L967
	testl	%edx, %edx
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L968
	vxorps	%xmm0, %xmm0, %xmm0
	movq	%rax, 8(%rsp)
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
	call	tan
	movq	8(%rsp), %rax
.L968:
	movl	$1, (%rax)
	movq	$0, 16(%rax)
	movl	$1, 24(%rax)
	vmovsd	%xmm0, 8(%rax)
	addq	$24, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L967:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rax, 8(%rsp)
	call	strtod
	call	tan
	movq	8(%rsp), %rax
	jmp	.L968
	.p2align 4,,10
	.p2align 3
.L966:
	vmovq	%rdi, %xmm0
	movq	%rax, 8(%rsp)
	call	tan
	movq	8(%rsp), %rax
	jmp	.L968
	.cfi_endproc
.LFE91:
	.size	aly_tan, .-aly_tan
	.p2align 4
	.globl	aly_log
	.type	aly_log, @function
aly_log:
.LFB92:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movl	16(%rsp), %eax
	movq	%rdi, %rbx
	movq	24(%rsp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm0
	je	.L974
	cmpl	$3, %eax
	je	.L973
	testl	%eax, %eax
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L974
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
.L974:
	call	log10
	movq	%rbx, %rax
	movl	$1, (%rbx)
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	vmovsd	%xmm0, 8(%rbx)
	popq	%rbx
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L973:
	.cfi_restore_state
	xorl	%esi, %esi
	call	strtod
	jmp	.L974
	.cfi_endproc
.LFE92:
	.size	aly_log, .-aly_log
	.p2align 4
	.globl	aly_ln
	.type	aly_ln, @function
aly_ln:
.LFB93:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movl	16(%rsp), %eax
	movq	%rdi, %rbx
	movq	24(%rsp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm0
	je	.L980
	cmpl	$3, %eax
	je	.L979
	testl	%eax, %eax
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L980
	vxorps	%xmm0, %xmm0, %xmm0
	vcvtsi2sdq	%rdi, %xmm0, %xmm0
.L980:
	call	log
	movq	%rbx, %rax
	movl	$1, (%rbx)
	movq	$0, 16(%rbx)
	movl	$1, 24(%rbx)
	vmovsd	%xmm0, 8(%rbx)
	popq	%rbx
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L979:
	.cfi_restore_state
	xorl	%esi, %esi
	call	strtod
	jmp	.L980
	.cfi_endproc
.LFE93:
	.size	aly_ln, .-aly_ln
	.p2align 4
	.globl	aly_min
	.type	aly_min, @function
aly_min:
.LFB94:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rdx
	vxorps	%xmm2, %xmm2, %xmm2
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	16(%rbp), %eax
	movq	24(%rbp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm1
	je	.L986
	cmpl	$3, %eax
	je	.L985
	testl	%eax, %eax
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L986
	vcvtsi2sdq	%rdi, %xmm2, %xmm1
.L986:
	movl	48(%rbp), %eax
	movq	56(%rbp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm0
	je	.L989
	cmpl	$3, %eax
	je	.L988
	testl	%eax, %eax
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L989
	vcvtsi2sdq	%rdi, %xmm2, %xmm2
	vmovapd	%xmm2, %xmm0
.L989:
	vcomisd	%xmm1, %xmm0
	jbe	.L996
	vmovdqu	16(%rbp), %ymm0
	movq	%rdx, %rax
	vmovdqu	%ymm0, (%rdx)
	vzeroupper
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L996:
	.cfi_restore_state
	vmovdqu	48(%rbp), %ymm0
	movq	%rdx, %rax
	vmovdqu	%ymm0, (%rdx)
	vzeroupper
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L985:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rdx, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rdx
	vxorps	%xmm2, %xmm2, %xmm2
	vmovapd	%xmm0, %xmm1
	jmp	.L986
	.p2align 4,,10
	.p2align 3
.L988:
	xorl	%esi, %esi
	movq	%rdx, (%rsp)
	vmovsd	%xmm1, 8(%rsp)
	call	strtod
	movq	(%rsp), %rdx
	vmovsd	8(%rsp), %xmm1
	jmp	.L989
	.cfi_endproc
.LFE94:
	.size	aly_min, .-aly_min
	.p2align 4
	.globl	aly_max
	.type	aly_max, @function
aly_max:
.LFB95:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rdi, %rdx
	vxorps	%xmm2, %xmm2, %xmm2
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	16(%rbp), %eax
	movq	24(%rbp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm1
	je	.L1001
	cmpl	$3, %eax
	je	.L1000
	testl	%eax, %eax
	vxorpd	%xmm1, %xmm1, %xmm1
	jne	.L1001
	vcvtsi2sdq	%rdi, %xmm2, %xmm1
.L1001:
	movl	48(%rbp), %eax
	movq	56(%rbp), %rdi
	cmpl	$1, %eax
	vmovq	%rdi, %xmm0
	je	.L1004
	cmpl	$3, %eax
	je	.L1003
	testl	%eax, %eax
	vxorpd	%xmm0, %xmm0, %xmm0
	jne	.L1004
	vcvtsi2sdq	%rdi, %xmm2, %xmm2
	vmovapd	%xmm2, %xmm0
.L1004:
	vcomisd	%xmm0, %xmm1
	jbe	.L1011
	vmovdqu	16(%rbp), %ymm0
	movq	%rdx, %rax
	vmovdqu	%ymm0, (%rdx)
	vzeroupper
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L1011:
	.cfi_restore_state
	vmovdqu	48(%rbp), %ymm0
	movq	%rdx, %rax
	vmovdqu	%ymm0, (%rdx)
	vzeroupper
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L1000:
	.cfi_restore_state
	xorl	%esi, %esi
	movq	%rdx, 8(%rsp)
	call	strtod
	movq	8(%rsp), %rdx
	vxorps	%xmm2, %xmm2, %xmm2
	vmovapd	%xmm0, %xmm1
	jmp	.L1001
	.p2align 4,,10
	.p2align 3
.L1003:
	xorl	%esi, %esi
	movq	%rdx, (%rsp)
	vmovsd	%xmm1, 8(%rsp)
	call	strtod
	movq	(%rsp), %rdx
	vmovsd	8(%rsp), %xmm1
	jmp	.L1004
	.cfi_endproc
.LFE95:
	.size	aly_max, .-aly_max
	.p2align 4
	.globl	aly_str_upper
	.type	aly_str_upper, @function
aly_str_upper:
.LFB96:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	pushq	%r13
	.cfi_offset 13, -24
	movq	%rdi, %r13
	pushq	%r12
	leaq	-64(%rbp), %rdi
	pushq	%r10
	pushq	%rbx
	subq	$64, %rsp
	.cfi_offset 12, -32
	.cfi_offset 10, -40
	.cfi_offset 3, -48
	vmovdqu	16(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-56(%rbp), %r12
	addq	$32, %rsp
	movsbq	(%r12), %rbx
	testb	%bl, %bl
	je	.L1014
	call	__ctype_toupper_loc
	movq	%rax, %rdx
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L1015:
	movq	(%rdx), %rax
	addq	$1, %r12
	movl	(%rax,%rbx,4), %eax
	movb	%al, -1(%r12)
	movsbq	(%r12), %rbx
	testb	%bl, %bl
	jne	.L1015
.L1014:
	vmovdqu	-64(%rbp), %ymm0
	movq	%r13, %rax
	vmovdqu	%ymm0, 0(%r13)
	vzeroupper
	leaq	-32(%rbp), %rsp
	popq	%rbx
	popq	%r10
	popq	%r12
	popq	%r13
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE96:
	.size	aly_str_upper, .-aly_str_upper
	.p2align 4
	.globl	aly_str_lower
	.type	aly_str_lower, @function
aly_str_lower:
.LFB97:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	pushq	%r13
	.cfi_offset 13, -24
	movq	%rdi, %r13
	pushq	%r12
	leaq	-64(%rbp), %rdi
	pushq	%r10
	pushq	%rbx
	subq	$64, %rsp
	.cfi_offset 12, -32
	.cfi_offset 10, -40
	.cfi_offset 3, -48
	vmovdqu	16(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-56(%rbp), %r12
	addq	$32, %rsp
	movsbq	(%r12), %rbx
	testb	%bl, %bl
	je	.L1022
	call	__ctype_tolower_loc
	movq	%rax, %rdx
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L1023:
	movq	(%rdx), %rax
	addq	$1, %r12
	movl	(%rax,%rbx,4), %eax
	movb	%al, -1(%r12)
	movsbq	(%r12), %rbx
	testb	%bl, %bl
	jne	.L1023
.L1022:
	vmovdqu	-64(%rbp), %ymm0
	movq	%r13, %rax
	vmovdqu	%ymm0, 0(%r13)
	vzeroupper
	leaq	-32(%rbp), %rsp
	popq	%rbx
	popq	%r10
	popq	%r12
	popq	%r13
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE97:
	.size	aly_str_lower, .-aly_str_lower
	.p2align 4
	.globl	aly_str_trim
	.type	aly_str_trim, @function
aly_str_trim:
.LFB98:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	pushq	%r13
	.cfi_offset 13, -24
	movq	%rdi, %r13
	pushq	%r12
	leaq	-64(%rbp), %rdi
	pushq	%r10
	pushq	%rbx
	subq	$64, %rsp
	.cfi_offset 12, -32
	.cfi_offset 10, -40
	.cfi_offset 3, -48
	vmovdqu	16(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	addq	$32, %rsp
	movq	-56(%rbp), %rbx
	call	__ctype_b_loc
	movq	(%rax), %r12
	movsbq	(%rbx), %rax
	testb	$32, 1(%r12,%rax,2)
	je	.L1030
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L1031:
	movsbq	1(%rbx), %rax
	addq	$1, %rbx
	testb	$32, 1(%r12,%rax,2)
	jne	.L1031
.L1030:
	movq	%rbx, %rdi
	call	strlen
	leaq	-1(%rbx,%rax), %rax
	cmpq	%rax, %rbx
	jb	.L1032
	jmp	.L1033
	.p2align 5
	.p2align 4,,10
	.p2align 3
.L1034:
	subq	$1, %rax
	cmpq	%rbx, %rax
	je	.L1033
.L1032:
	movsbq	(%rax), %rdx
	testb	$32, 1(%r12,%rdx,2)
	jne	.L1034
.L1033:
	movb	$0, 1(%rax)
	movq	%rbx, %rdi
	call	strdup
	vmovdqu	-64(%rbp), %ymm0
	subq	$32, %rsp
	movq	%rax, %rbx
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movq	%rbx, 8(%r13)
	addq	$32, %rsp
	movl	$3, 0(%r13)
	movq	%r13, %rax
	movq	$0, 16(%r13)
	movl	$1, 24(%r13)
	leaq	-32(%rbp), %rsp
	popq	%rbx
	popq	%r10
	popq	%r12
	popq	%r13
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE98:
	.size	aly_str_trim, .-aly_str_trim
	.p2align 4
	.globl	aly_str_contains
	.type	aly_str_contains, @function
aly_str_contains:
.LFB99:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	pushq	%r10
	pushq	%rbx
	.cfi_offset 10, -24
	.cfi_offset 3, -32
	leaq	16(%rbp), %r10
	leaq	-80(%rbp), %rdi
	movq	%r10, %rbx
	subq	$96, %rsp
	vmovdqu	(%r10), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	vmovdqu	32(%rbx), %ymm0
	leaq	-48(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-40(%rbp), %rsi
	movq	-72(%rbp), %rdi
	addq	$32, %rsp
	call	strstr
	vmovdqu	-80(%rbp), %ymm0
	subq	$32, %rsp
	movq	%rax, %rbx
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	vmovdqu	-48(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	addq	$32, %rsp
	xorl	%eax, %eax
	testq	%rbx, %rbx
	setne	%al
	leaq	-16(%rbp), %rsp
	popq	%rbx
	popq	%r10
	popq	%rbp
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE99:
	.size	aly_str_contains, .-aly_str_contains
	.section	.rodata.str1.1
.LC25:
	.string	"r"
.LC26:
	.string	"Error: Cannot open file\n"
	.text
	.p2align 4
	.globl	aly_fs_read
	.type	aly_fs_read, @function
aly_fs_read:
.LFB100:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$112, %rsp
	movq	%r10, -32(%rbp)
	vmovdqu	16(%rbp), %ymm0
	movq	%rbx, -40(%rbp)
	movq	%r12, -24(%rbp)
	.cfi_offset 10, -48
	.cfi_offset 3, -56
	.cfi_offset 12, -40
	movq	%rdi, %r12
	leaq	-80(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-72(%rbp), %rdi
	addq	$32, %rsp
	movl	$.LC25, %esi
	call	fopen
	vmovdqu	-80(%rbp), %ymm0
	subq	$32, %rsp
	movq	%rax, %rbx
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	addq	$32, %rsp
	testq	%rbx, %rbx
	je	.L1048
	movl	$2, %edx
	xorl	%esi, %esi
	movq	%rbx, %rdi
	movq	%r13, -16(%rbp)
	movq	%r14, -8(%rbp)
	.cfi_offset 13, -32
	.cfi_offset 14, -24
	call	fseek
	movq	%rbx, %rdi
	call	ftell
	xorl	%edx, %edx
	xorl	%esi, %esi
	movq	%rbx, %rdi
	movq	%rax, %r14
	call	fseek
	leaq	1(%r14), %rdi
	call	malloc
	movq	%rbx, %rcx
	movq	%r14, %rdx
	movl	$1, %esi
	movq	%rax, %r13
	movq	%rax, %rdi
	call	fread
	movb	$0, 0(%r13,%r14)
	movq	%rbx, %rdi
	call	fclose
	movq	%r13, %rdi
	call	strdup
	movq	%r13, %rdi
	movq	%rax, %rbx
	call	free
	movq	-16(%rbp), %r13
	.cfi_restore 13
	movq	-8(%rbp), %r14
	.cfi_restore 14
.L1046:
	movq	%rbx, 8(%r12)
	movq	%r12, %rax
	movq	-40(%rbp), %rbx
	movl	$3, (%r12)
	movq	$0, 16(%r12)
	movl	$1, 24(%r12)
	movq	-24(%rbp), %r12
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
.L1048:
	.cfi_restore_state
	movl	$24, %edx
	movl	$1, %esi
	movl	$.LC26, %edi
	movq	stderr(%rip), %rcx
	call	fwrite
	movl	$.LC20, %edi
	call	strdup
	movq	%rax, %rbx
	jmp	.L1046
	.cfi_endproc
.LFE100:
	.size	aly_fs_read, .-aly_fs_read
	.section	.rodata.str1.1
.LC27:
	.string	"w"
	.text
	.p2align 4
	.globl	aly_fs_write
	.type	aly_fs_write, @function
aly_fs_write:
.LFB101:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$112, %rsp
	movq	%r13, -8(%rbp)
	vmovdqu	16(%rbp), %ymm0
	leaq	-80(%rbp), %rdi
	.cfi_offset 13, -24
	leaq	16(%rbp), %r13
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	vmovdqu	32(%r13), %ymm0
	leaq	-48(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-72(%rbp), %rdi
	addq	$32, %rsp
	movl	$.LC27, %esi
	call	fopen
	testq	%rax, %rax
	je	.L1050
	movq	-40(%rbp), %rdi
	movq	%rbx, -16(%rbp)
	.cfi_offset 3, -32
	movq	%rax, %rsi
	movq	%rax, %rbx
	call	fputs
	movq	%rbx, %rdi
	call	fclose
	movq	-16(%rbp), %rbx
	.cfi_restore 3
.L1050:
	vmovdqu	-80(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	vmovdqu	-48(%rbp), %ymm0
	addq	$32, %rsp
	vmovdqu	%ymm0, 0(%r13)
	vzeroupper
	movq	-8(%rbp), %r13
	leave
	.cfi_def_cfa 7, 8
	jmp	aly_free
	.cfi_endproc
.LFE101:
	.size	aly_fs_write, .-aly_fs_write
	.section	.rodata.str1.1
.LC28:
	.string	"a"
	.text
	.p2align 4
	.globl	aly_fs_append
	.type	aly_fs_append, @function
aly_fs_append:
.LFB102:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$112, %rsp
	movq	%r13, -8(%rbp)
	vmovdqu	16(%rbp), %ymm0
	leaq	-80(%rbp), %rdi
	.cfi_offset 13, -24
	leaq	16(%rbp), %r13
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	vmovdqu	32(%r13), %ymm0
	leaq	-48(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-72(%rbp), %rdi
	addq	$32, %rsp
	movl	$.LC28, %esi
	call	fopen
	testq	%rax, %rax
	je	.L1056
	movq	-40(%rbp), %rdi
	movq	%rbx, -16(%rbp)
	.cfi_offset 3, -32
	movq	%rax, %rsi
	movq	%rax, %rbx
	call	fputs
	movq	%rbx, %rdi
	call	fclose
	movq	-16(%rbp), %rbx
	.cfi_restore 3
.L1056:
	vmovdqu	-80(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	vmovdqu	-48(%rbp), %ymm0
	addq	$32, %rsp
	vmovdqu	%ymm0, 0(%r13)
	vzeroupper
	movq	-8(%rbp), %r13
	leave
	.cfi_def_cfa 7, 8
	jmp	aly_free
	.cfi_endproc
.LFE102:
	.size	aly_fs_append, .-aly_fs_append
	.p2align 4
	.globl	aly_fs_exists
	.type	aly_fs_exists, @function
aly_fs_exists:
.LFB103:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	addq	$-128, %rsp
	movq	%r10, -32(%rbp)
	vmovdqu	16(%rbp), %ymm0
	leaq	-80(%rbp), %rdi
	movq	%rbx, -40(%rbp)
	movq	%r12, -24(%rbp)
	movq	%r13, -16(%rbp)
	vmovdqu	%ymm0, (%rsp)
	.cfi_offset 10, -48
	.cfi_offset 3, -56
	.cfi_offset 12, -40
	.cfi_offset 13, -32
	vzeroupper
	call	aly_to_str
	movq	-72(%rbp), %rbx
	addq	$32, %rsp
	movl	$.LC25, %esi
	movq	%rbx, %rdi
	call	fopen
	movq	-64(%rbp), %r13
	movq	%rax, %r12
	movl	-80(%rbp), %eax
	cmpl	$4, %eax
	je	.L1062
	cmpl	$5, %eax
	je	.L1063
	cmpl	$3, %eax
	jne	.L1064
.L1086:
	movq	%rbx, %rdi
	call	free
.L1064:
	testq	%r13, %r13
	je	.L1071
	movq	%r13, %rdi
	call	free
.L1071:
	testq	%r12, %r12
	je	.L1073
	movq	%r12, %rdi
	call	fclose
	movl	$1, %eax
.L1061:
	movq	-40(%rbp), %rbx
	movq	-24(%rbp), %r12
	movq	-16(%rbp), %r13
	leave
	.cfi_remember_state
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L1063:
	.cfi_restore_state
	testq	%rbx, %rbx
	je	.L1064
	movl	12(%rbx), %esi
	movq	(%rbx), %rdi
	testl	%esi, %esi
	jle	.L1067
	movq	%r14, -8(%rbp)
	.cfi_offset 14, -24
	xorl	%r14d, %r14d
.L1070:
	leaq	(%r14,%r14,2), %rdx
	salq	$4, %rdx
	leaq	(%rdi,%rdx), %rcx
	movl	40(%rcx), %eax
	testl	%eax, %eax
	jne	.L1087
	addq	$1, %r14
	cmpl	%r14d, %esi
	jg	.L1070
.L1085:
	movq	-8(%rbp), %r14
	.cfi_restore 14
.L1067:
	call	free
	jmp	.L1086
	.p2align 4,,10
	.p2align 3
.L1062:
	testq	%rbx, %rbx
	je	.L1064
	movl	8(%rbx), %edx
	testl	%edx, %edx
	jle	.L1065
	movq	%r14, -8(%rbp)
	.cfi_offset 14, -24
	xorl	%r14d, %r14d
.L1066:
	movq	%r14, %rax
	subq	$32, %rsp
	salq	$5, %rax
	addq	(%rbx), %rax
	vmovdqu	(%rax), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	addq	$1, %r14
	addq	$32, %rsp
	cmpl	%r14d, 8(%rbx)
	jg	.L1066
	movq	-8(%rbp), %r14
	.cfi_restore 14
.L1065:
	movq	(%rbx), %rdi
	call	free
	movq	%rbx, %rdi
	call	free
	jmp	.L1064
	.p2align 4,,10
	.p2align 3
.L1087:
	.cfi_offset 14, -24
	movq	(%rcx), %rdi
	movq	%rdx, -88(%rbp)
	call	free
	movq	(%rbx), %rcx
	movq	-88(%rbp), %rdx
	subq	$32, %rsp
	vmovdqu	8(%rcx,%rdx), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	movl	12(%rbx), %esi
	addq	$1, %r14
	addq	$32, %rsp
	movq	(%rbx), %rdi
	cmpl	%r14d, %esi
	jg	.L1070
	jmp	.L1085
.L1073:
	.cfi_restore 14
	xorl	%eax, %eax
	jmp	.L1061
	.cfi_endproc
.LFE103:
	.size	aly_fs_exists, .-aly_fs_exists
	.p2align 4
	.globl	aly_ref
	.type	aly_ref, @function
aly_ref:
.LFB104:
	.cfi_startproc
	ret
	.cfi_endproc
.LFE104:
	.size	aly_ref, .-aly_ref
	.p2align 4
	.globl	aly_init
	.type	aly_init, @function
aly_init:
.LFB105:
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
.LFE105:
	.size	aly_init, .-aly_init
	.p2align 4
	.globl	aly_cleanup
	.type	aly_cleanup, @function
aly_cleanup:
.LFB106:
	.cfi_startproc
	ret
	.cfi_endproc
.LFE106:
	.size	aly_cleanup, .-aly_cleanup
	.p2align 4
	.globl	aly_sys_args
	.type	aly_sys_args, @function
aly_sys_args:
.LFB107:
	.cfi_startproc
	subq	$40, %rsp
	.cfi_def_cfa_offset 48
	movl	$16, %esi
	movq	%r13, 24(%rsp)
	.cfi_offset 13, -24
	movl	_aly_argc(%rip), %r13d
	movq	%r14, 32(%rsp)
	.cfi_offset 14, -16
	movq	%rdi, %r14
	movl	$1, %edi
	movq	%r12, 16(%rsp)
	.cfi_offset 12, -32
	call	calloc
	testl	%r13d, %r13d
	movq	%rax, %r12
	jle	.L1098
	movq	%rbx, (%rsp)
	movq	%r13, %rdi
	movl	$32, %esi
	movq	%rbp, 8(%rsp)
	.cfi_offset 3, -48
	.cfi_offset 6, -40
	movl	%r13d, 12(%rax)
	salq	$5, %r13
	call	calloc
	movq	_aly_argv(%rip), %rbp
	movl	$0, 8(%r12)
	movq	%rax, (%r12)
	movq	%rax, %rbx
	addq	%rax, %r13
	.p2align 4,,10
	.p2align 3
.L1095:
	movq	0(%rbp), %rdi
	addq	$32, %rbx
	addq	$8, %rbp
	call	strdup
	movl	$3, -32(%rbx)
	movq	%rax, -24(%rbx)
	movq	$0, -16(%rbx)
	movl	$1, -8(%rbx)
	addl	$1, 8(%r12)
	cmpq	%r13, %rbx
	jne	.L1095
	movq	(%rsp), %rbx
	.cfi_restore 3
	movq	8(%rsp), %rbp
	.cfi_restore 6
.L1094:
	movq	%r12, 8(%r14)
	movq	%r14, %rax
	movq	16(%rsp), %r12
	movl	$4, (%r14)
	movq	24(%rsp), %r13
	movq	$0, 16(%r14)
	movq	32(%rsp), %r14
	addq	$40, %rsp
	.cfi_remember_state
	.cfi_def_cfa_offset 8
	ret
	.p2align 4,,10
	.p2align 3
.L1098:
	.cfi_restore_state
	movl	$32, %esi
	movl	$8, %edi
	call	calloc
	movq	%rax, (%r12)
	movabsq	$34359738368, %rax
	movq	%rax, 8(%r12)
	jmp	.L1094
	.cfi_endproc
.LFE107:
	.size	aly_sys_args, .-aly_sys_args
	.section	.rodata.str1.1
.LC29:
	.string	"nome"
.LC30:
	.string	"\"Pedro\""
.LC31:
	.string	"tomb"
	.section	.rodata.str1.8
	.align 8
.LC32:
	.string	"Var 'nome' agora \303\251 imut\303\241vel, "
	.section	.rodata.str1.1
.LC33:
	.string	"\"Peter\""
	.text
	.p2align 4
	.globl	fn_0
	.type	fn_0, @function
fn_0:
.LFB112:
	.cfi_startproc
	leaq	8(%rsp), %r10
	.cfi_def_cfa 10, 0
	andq	$-32, %rsp
	pushq	-8(%r10)
	pushq	%rbp
	movq	%rsp, %rbp
	.cfi_escape 0x10,0x6,0x2,0x76,0
	pushq	%r13
	pushq	%r12
	pushq	%r10
	.cfi_escape 0xf,0x3,0x76,0x68,0x6
	.cfi_escape 0x10,0xd,0x2,0x76,0x78
	.cfi_escape 0x10,0xc,0x2,0x76,0x70
	pushq	%rbx
	.cfi_escape 0x10,0x3,0x2,0x76,0x60
	movq	%rdi, %rbx
	movl	$.LC29, %edi
	subq	$32944, %rsp
	movl	$6, -32816(%rbp)
	movq	$0, -32808(%rbp)
	movq	$0, -32800(%rbp)
	movl	$1, -32792(%rbp)
	vmovdqu	-32816(%rbp), %ymm0
	vmovdqu	%ymm0, 32(%rsp)
	vmovdqa	aly_globals(%rip), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_set
	vmovdqa	aly_globals(%rip), %ymm0
	addq	$32, %rsp
	leaq	-32816(%rbp), %rdi
	movl	$.LC29, %esi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_get
	addq	$32, %rsp
	cmpl	$3, -32816(%rbp)
	je	.L1115
	vmovdqu	-32816(%rbp), %ymm0
	subq	$32, %rsp
	leaq	-32848(%rbp), %rdi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_to_str
	movq	-32840(%rbp), %rdi
	addq	$32, %rsp
	call	puts
	vmovdqu	-32848(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	addq	$32, %rsp
.L1101:
	movl	$.LC30, %edi
	call	strdup
	subq	$64, %rsp
	movl	$.LC29, %edi
	movl	$3, -32816(%rbp)
	movq	%rax, -32808(%rbp)
	movq	$0, -32800(%rbp)
	movl	$1, -32792(%rbp)
	vmovdqu	-32816(%rbp), %ymm0
	vmovdqu	%ymm0, 32(%rsp)
	vmovdqa	aly_globals(%rip), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_set
	vmovdqa	aly_globals(%rip), %ymm0
	addq	$32, %rsp
	leaq	-32784(%rbp), %rdi
	movl	$.LC31, %esi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_get
	vmovdqa	aly_globals(%rip), %ymm0
	movq	-32776(%rbp), %r12
	movl	-32784(%rbp), %r13d
	movl	$.LC29, %edi
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_get_ptr
	addq	$32, %rsp
	testq	%r12, %r12
	movl	$8, -32752(%rbp)
	movq	%rax, -32744(%rbp)
	movq	$0, -32736(%rbp)
	movl	$1, -32728(%rbp)
	vmovdqu	-32752(%rbp), %ymm0
	vmovdqu	%ymm0, -32880(%rbp)
	je	.L1107
	cmpl	$7, %r13d
	je	.L1116
.L1107:
	movl	$1, %eax
	xorl	%edx, %edx
	xorl	%ecx, %ecx
	movl	$6, %esi
.L1102:
	movq	%rcx, -32840(%rbp)
	movl	$.LC32, %edi
	movq	%rdx, -32832(%rbp)
	movl	%esi, -32848(%rbp)
	movl	%eax, -32824(%rbp)
	vmovdqu	-32848(%rbp), %ymm0
	vmovdqu	%ymm0, -32784(%rbp)
	vzeroupper
	call	strdup
	vmovdqa	aly_globals(%rip), %ymm0
	subq	$32, %rsp
	movl	$.LC29, %esi
	leaq	-32752(%rbp), %rdi
	movq	%rax, %r12
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_get
	vmovdqu	-32752(%rbp), %ymm0
	subq	$32, %rsp
	movq	%r12, -32776(%rbp)
	leaq	-32912(%rbp), %rdi
	movl	$3, -32784(%rbp)
	movq	$0, -32768(%rbp)
	movl	$1, -32760(%rbp)
	vmovdqu	%ymm0, 32(%rsp)
	vmovdqu	-32784(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_add
	movq	-32912(%rbp), %rax
	movq	-32904(%rbp), %rsi
	movq	-32896(%rbp), %rcx
	movq	-32888(%rbp), %rdx
	addq	$64, %rsp
	cmpl	$3, %eax
	movq	%rax, -32784(%rbp)
	movq	%rsi, -32776(%rbp)
	movq	%rcx, -32768(%rbp)
	movq	%rdx, -32760(%rbp)
	je	.L1117
	subq	$32, %rsp
	leaq	-32848(%rbp), %rdi
	movq	%rsi, 8(%rsp)
	movq	%rcx, 16(%rsp)
	movq	%rdx, 24(%rsp)
	movq	%rax, (%rsp)
	call	aly_to_str
	movq	-32840(%rbp), %rdi
	addq	$32, %rsp
	call	puts
	vmovdqu	-32848(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	addq	$32, %rsp
.L1104:
	movl	$.LC33, %edi
	call	strdup
	subq	$64, %rsp
	movl	$.LC29, %edi
	movl	$3, -32784(%rbp)
	movq	%rax, -32776(%rbp)
	movq	$0, -32768(%rbp)
	movl	$1, -32760(%rbp)
	vmovdqu	-32784(%rbp), %ymm0
	vmovdqu	%ymm0, 32(%rsp)
	vmovdqa	aly_globals(%rip), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_set
	addq	$64, %rsp
	movl	$.LC32, %edi
	call	strdup
	subq	$32, %rsp
	vmovdqa	aly_globals(%rip), %ymm0
	movl	$.LC29, %esi
	leaq	-32720(%rbp), %rdi
	movq	%rax, %r12
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_get
	vmovdqu	-32720(%rbp), %ymm0
	subq	$32, %rsp
	movq	%r12, -32744(%rbp)
	leaq	-32912(%rbp), %rdi
	movl	$3, -32752(%rbp)
	movq	$0, -32736(%rbp)
	movl	$1, -32728(%rbp)
	vmovdqu	%ymm0, 32(%rsp)
	vmovdqu	-32752(%rbp), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_add
	movq	-32912(%rbp), %rax
	movq	-32904(%rbp), %rsi
	movq	-32896(%rbp), %rcx
	movq	-32888(%rbp), %rdx
	addq	$64, %rsp
	cmpl	$3, %eax
	movq	%rax, -32752(%rbp)
	movq	%rsi, -32744(%rbp)
	movq	%rcx, -32736(%rbp)
	movq	%rdx, -32728(%rbp)
	je	.L1118
	subq	$32, %rsp
	leaq	-32848(%rbp), %rdi
	movq	%rsi, 8(%rsp)
	movq	%rcx, 16(%rsp)
	movq	%rdx, 24(%rsp)
	movq	%rax, (%rsp)
	call	aly_to_str
	movq	-32840(%rbp), %rdi
	addq	$32, %rsp
	call	puts
	vmovdqu	-32848(%rbp), %ymm0
	subq	$32, %rsp
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_free
	addq	$32, %rsp
.L1106:
	movl	$6, -32752(%rbp)
	movq	%rbx, %rax
	movq	$0, -32744(%rbp)
	movq	$0, -32736(%rbp)
	movl	$1, -32728(%rbp)
	vmovdqu	-32752(%rbp), %ymm0
	vmovdqu	%ymm0, (%rbx)
	vzeroupper
	leaq	-32(%rbp), %rsp
	popq	%rbx
	popq	%r10
	.cfi_remember_state
	.cfi_def_cfa 10, 0
	popq	%r12
	popq	%r13
	popq	%rbp
	leaq	-8(%r10), %rsp
	.cfi_def_cfa 7, 8
	ret
	.p2align 4,,10
	.p2align 3
.L1116:
	.cfi_restore_state
	leaq	-32880(%rbp), %rdx
	movl	$1, %esi
	leaq	-32848(%rbp), %rdi
	vzeroupper
	call	*%r12
	movl	-32848(%rbp), %esi
	movq	-32840(%rbp), %rcx
	movq	-32832(%rbp), %rdx
	movl	-32824(%rbp), %eax
	jmp	.L1102
	.p2align 4,,10
	.p2align 3
.L1115:
	movq	-32808(%rbp), %rdi
	call	puts
	jmp	.L1101
	.p2align 4,,10
	.p2align 3
.L1118:
	movq	%rsi, %rdi
	call	puts
	jmp	.L1106
	.p2align 4,,10
	.p2align 3
.L1117:
	movq	%rsi, %rdi
	call	puts
	jmp	.L1104
	.cfi_endproc
.LFE112:
	.size	fn_0, .-fn_0
	.section	.rodata.str1.1
.LC34:
	.string	"print"
.LC35:
	.string	"input"
.LC36:
	.string	"len"
	.section	.text.startup,"ax",@progbits
	.p2align 4
	.globl	main
	.type	main, @function
main:
.LFB113:
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
	movl	$48, %esi
	movl	$32, %edi
	movq	%rax, %rbx
	call	calloc
	movl	$1, -120(%rbp)
	subq	$64, %rsp
	movl	$.LC34, %edi
	movl	$7, -144(%rbp)
	movq	$native_print, -136(%rbp)
	movq	$0, -128(%rbp)
	vmovdqu	-144(%rbp), %ymm0
	movq	%rax, (%rbx)
	movabsq	$137438953472, %rax
	movq	%rax, 8(%rbx)
	movq	%rbx, aly_globals+8(%rip)
	movq	$0, aly_globals+16(%rip)
	vmovdqu	%ymm0, 32(%rsp)
	vmovdqa	aly_globals(%rip), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_set
	movl	$1, -88(%rbp)
	movl	$.LC35, %edi
	movl	$7, -112(%rbp)
	movq	$native_input, -104(%rbp)
	movq	$0, -96(%rbp)
	vmovdqu	-112(%rbp), %ymm0
	vmovdqu	%ymm0, 32(%rsp)
	vmovdqa	aly_globals(%rip), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_set
	movl	$1, -56(%rbp)
	movl	$.LC31, %edi
	movl	$7, -80(%rbp)
	movq	$native_tomb, -72(%rbp)
	movq	$0, -64(%rbp)
	vmovdqu	-80(%rbp), %ymm0
	vmovdqu	%ymm0, 32(%rsp)
	vmovdqa	aly_globals(%rip), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_set
	movl	$1, -24(%rbp)
	movl	$.LC36, %edi
	movl	$7, -48(%rbp)
	movq	$native_len, -40(%rbp)
	movq	$0, -32(%rbp)
	vmovdqu	-48(%rbp), %ymm0
	vmovdqu	%ymm0, 32(%rsp)
	vmovdqa	aly_globals(%rip), %ymm0
	vmovdqu	%ymm0, (%rsp)
	vzeroupper
	call	aly_object_set
	addq	$64, %rsp
	xorl	%esi, %esi
	leaq	-208(%rbp), %rdi
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
.LFE113:
	.size	main, .-main
	.section	.rodata.str1.1
.LC37:
	.string	"int"
.LC38:
	.string	"float"
.LC39:
	.string	"bool"
.LC40:
	.string	"string"
.LC41:
	.string	"array"
.LC42:
	.string	"object"
.LC43:
	.string	"function"
.LC44:
	.string	"reference"
	.section	.rodata
	.align 32
	.type	CSWTCH.89, @object
	.size	CSWTCH.89, 72
CSWTCH.89:
	.quad	.LC37
	.quad	.LC38
	.quad	.LC39
	.quad	.LC40
	.quad	.LC41
	.quad	.LC42
	.quad	.LC6
	.quad	.LC43
	.quad	.LC44
	.globl	aly_globals
	.bss
	.align 32
	.type	aly_globals, @object
	.size	aly_globals, 32
aly_globals:
	.zero	32
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
.LC22:
	.long	-1
	.long	2147483647
	.long	0
	.long	0
	.section	.rodata.cst8
	.align 8
.LC23:
	.long	-4194304
	.long	1105199103
	.align 8
.LC24:
	.long	0
	.long	1072693248
	.ident	"GCC: (GNU) 16.1.1 20260515 (Red Hat 16.1.1-2)"
	.section	.note.GNU-stack,"",@progbits
