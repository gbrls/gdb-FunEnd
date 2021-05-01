	.file	"a.c"
	.text
	.globl	amazing_func
	.def	amazing_func;	.scl	2;	.type	32;	.endef
	.seh_proc	amazing_func
amazing_func:
	pushq	%rbp
	.seh_pushreg	%rbp
	movq	%rsp, %rbp
	.seh_setframe	%rbp, 0
	subq	$16, %rsp
	.seh_stackalloc	16
	.seh_endprologue
	movl	%ecx, 16(%rbp)
	movl	16(%rbp), %eax
	movl	%eax, -4(%rbp)
	movl	-4(%rbp), %edx
	movl	%edx, %eax
	sall	$2, %eax
	addl	%edx, %eax
	sall	$2, %eax
	movl	%eax, -4(%rbp)
	movl	-4(%rbp), %ecx
	movl	$1717986919, %edx
	movl	%ecx, %eax
	imull	%edx
	sarl	$3, %edx
	movl	%ecx, %eax
	sarl	$31, %eax
	subl	%eax, %edx
	movl	%edx, %eax
	movl	%eax, -4(%rbp)
	movl	$1, %eax
	subl	-4(%rbp), %eax
	movl	%eax, -4(%rbp)
	movl	$1, %eax
	subl	-4(%rbp), %eax
	movl	%eax, -4(%rbp)
	movl	-4(%rbp), %eax
	addq	$16, %rsp
	popq	%rbp
	ret
	.seh_endproc
	.globl	fib
	.def	fib;	.scl	2;	.type	32;	.endef
	.seh_proc	fib
fib:
	pushq	%rbp
	.seh_pushreg	%rbp
	pushq	%rbx
	.seh_pushreg	%rbx
	subq	$40, %rsp
	.seh_stackalloc	40
	leaq	128(%rsp), %rbp
	.seh_setframe	%rbp, 128
	.seh_endprologue
	movl	%ecx, -64(%rbp)
	movl	-64(%rbp), %ecx
	call	amazing_func
	movl	%eax, -64(%rbp)
	cmpl	$1, -64(%rbp)
	jg	.L4
	movl	-64(%rbp), %eax
	jmp	.L5
.L4:
	movl	-64(%rbp), %eax
	subl	$1, %eax
	movl	%eax, %ecx
	call	fib
	movl	%eax, %ebx
	movl	-64(%rbp), %eax
	subl	$2, %eax
	movl	%eax, %ecx
	call	fib
	addl	%ebx, %eax
.L5:
	addq	$40, %rsp
	popq	%rbx
	popq	%rbp
	ret
	.seh_endproc
	.def	__main;	.scl	2;	.type	32;	.endef
	.globl	main
	.def	main;	.scl	2;	.type	32;	.endef
	.seh_proc	main
main:
	pushq	%rbp
	.seh_pushreg	%rbp
	movq	%rsp, %rbp
	.seh_setframe	%rbp, 0
	subq	$64, %rsp
	.seh_stackalloc	64
	.seh_endprologue
	call	__main
	movl	$10, -12(%rbp)
	movl	-12(%rbp), %eax
	movl	%eax, %ecx
	call	fib
	movl	%eax, -4(%rbp)
	movl	$0, %eax
	addq	$64, %rsp
	popq	%rbp
	ret
	.seh_endproc
	.ident	"GCC: (x86_64-posix-seh-rev0, Built by MinGW-W64 project) 8.1.0"
