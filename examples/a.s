.global amazing_func

amazing_func:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$16, %rsp

	addq	$16, %rsp
	popq	%rbp
	ret
