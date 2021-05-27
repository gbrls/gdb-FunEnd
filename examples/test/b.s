test:

movq $123, %rcx
xorq %rdx, %rdx

retq

.def	__main;	.scl	2;	.type	32;	.endef
.globl	main
.def	main;	.scl	2;	.type	32;	.endef
.seh_proc	main
main:
call test
movq $42, %rax
movq $10, %rbx
imulq %rbx, %rax

retq
.seh_endproc
