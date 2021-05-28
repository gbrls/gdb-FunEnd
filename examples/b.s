	.text
	.globl	main
main:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$48, %rsp
	call	__main
	movl	$10, -4(%rbp)
	movl	$20, -8(%rbp)
	subl	$5, -8(%rbp)
	movl	-8(%rbp), %eax
	xorl	%eax, -4(%rbp)
	movl	-4(%rbp), %eax
	cltd
	idivl	-8(%rbp)
	addq	$48, %rsp
	popq	%rbp
	ret
