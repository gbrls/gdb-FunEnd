	.file	"a.c"
	.text
.Ltext0:
	.cfi_sections	.debug_frame
	.globl	amazing_func
	.def	amazing_func;	.scl	2;	.type	32;	.endef
	.seh_proc	amazing_func
amazing_func:
.LFB0:
	.file 1 "./b.h"
	.loc 1 1 34
	.cfi_startproc
	pushq	%rbp
	.seh_pushreg	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.seh_setframe	%rbp, 0
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	.seh_stackalloc	16
	.seh_endprologue
	movl	%ecx, 16(%rbp)
	movl	%edx, 24(%rbp)
	.loc 1 2 9
	movl	16(%rbp), %eax
	movl	%eax, -4(%rbp)
	.loc 1 3 7
	movl	-4(%rbp), %edx
	movl	%edx, %eax
	sall	$2, %eax
	addl	%edx, %eax
	sall	$2, %eax
	movl	%eax, -4(%rbp)
	.loc 1 4 7
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
	.loc 1 5 7
	movl	$1, %eax
	subl	-4(%rbp), %eax
	movl	%eax, -4(%rbp)
	.loc 1 6 7
	movl	$1, %eax
	subl	-4(%rbp), %eax
	movl	%eax, -4(%rbp)
	.loc 1 12 9
	movl	16(%rbp), %eax
	andl	$2, %eax
	movl	%eax, 24(%rbp)
	.loc 1 14 20
	movl	-4(%rbp), %eax
	.loc 1 15 1
	addq	$16, %rsp
	popq	%rbp
	.cfi_restore 6
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE0:
	.seh_endproc
	.globl	do_something
	.def	do_something;	.scl	2;	.type	32;	.endef
	.seh_proc	do_something
do_something:
.LFB1:
	.file 2 "./a.c"
	.loc 2 2 32
	.cfi_startproc
	pushq	%rbp
	.seh_pushreg	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.seh_setframe	%rbp, 0
	.cfi_def_cfa_register 6
	.seh_endprologue
	movl	%ecx, 16(%rbp)
	movl	%edx, 24(%rbp)
	.loc 2 3 7
	sall	16(%rbp)
	.loc 2 4 7
	addl	$5, 24(%rbp)
	.loc 2 6 7
	movl	16(%rbp), %eax
	imull	24(%rbp), %eax
	movl	%eax, 16(%rbp)
	.loc 2 8 7
	subl	$1, 16(%rbp)
	.loc 2 10 7
	movl	16(%rbp), %eax
	cltd
	idivl	24(%rbp)
	movl	%eax, 16(%rbp)
	.loc 2 12 12
	movl	16(%rbp), %eax
	.loc 2 13 1
	popq	%rbp
	.cfi_restore 6
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE1:
	.seh_endproc
	.def	__main;	.scl	2;	.type	32;	.endef
	.globl	main
	.def	main;	.scl	2;	.type	32;	.endef
	.seh_proc	main
main:
.LFB2:
	.loc 2 15 12
	.cfi_startproc
	pushq	%rbp
	.seh_pushreg	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.seh_setframe	%rbp, 0
	.cfi_def_cfa_register 6
	subq	$48, %rsp
	.seh_stackalloc	48
	.seh_endprologue
	.loc 2 15 12
	call	__main
.LVL0:
	.loc 2 17 13
	movl	$5, %edx
	movl	$2, %ecx
	call	do_something
	movl	%eax, -4(%rbp)
	.loc 2 19 12
	movl	$0, %eax
	.loc 2 20 1
	addq	$48, %rsp
	popq	%rbp
	.cfi_restore 6
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc
.LFE2:
	.seh_endproc
.Letext0:
	.section	.debug_info,"dr"
.Ldebug_info0:
	.long	0x14e
	.word	0x4
	.secrel32	.Ldebug_abbrev0
	.byte	0x8
	.uleb128 0x1
	.ascii "GNU C17 8.1.0 -mtune=core2 -march=nocona -g\0"
	.byte	0xc
	.ascii ".\\a.c\0"
	.ascii "C:\\Users\\gabri\\Programming\\GDB-FunEnd\\examples\0"
	.quad	.Ltext0
	.quad	.Letext0-.Ltext0
	.secrel32	.Ldebug_line0
	.uleb128 0x2
	.ascii "main\0"
	.byte	0x2
	.byte	0xf
	.byte	0x5
	.long	0xb3
	.quad	.LFB2
	.quad	.LFE2-.LFB2
	.uleb128 0x1
	.byte	0x9c
	.long	0xb3
	.uleb128 0x3
	.ascii "z\0"
	.byte	0x2
	.byte	0x11
	.byte	0x9
	.long	0xb3
	.uleb128 0x2
	.byte	0x91
	.sleb128 -20
	.byte	0
	.uleb128 0x4
	.byte	0x4
	.byte	0x5
	.ascii "int\0"
	.uleb128 0x5
	.ascii "do_something\0"
	.byte	0x2
	.byte	0x2
	.byte	0x5
	.long	0xb3
	.quad	.LFB1
	.quad	.LFE1-.LFB1
	.uleb128 0x1
	.byte	0x9c
	.long	0x100
	.uleb128 0x6
	.ascii "x\0"
	.byte	0x2
	.byte	0x2
	.byte	0x16
	.long	0xb3
	.uleb128 0x2
	.byte	0x91
	.sleb128 0
	.uleb128 0x6
	.ascii "y\0"
	.byte	0x2
	.byte	0x2
	.byte	0x1d
	.long	0xb3
	.uleb128 0x2
	.byte	0x91
	.sleb128 8
	.byte	0
	.uleb128 0x7
	.ascii "amazing_func\0"
	.byte	0x1
	.byte	0x1
	.byte	0x5
	.long	0xb3
	.quad	.LFB0
	.quad	.LFE0-.LFB0
	.uleb128 0x1
	.byte	0x9c
	.uleb128 0x6
	.ascii "x\0"
	.byte	0x1
	.byte	0x1
	.byte	0x16
	.long	0xb3
	.uleb128 0x2
	.byte	0x91
	.sleb128 0
	.uleb128 0x6
	.ascii "idk\0"
	.byte	0x1
	.byte	0x1
	.byte	0x1d
	.long	0xb3
	.uleb128 0x2
	.byte	0x91
	.sleb128 8
	.uleb128 0x3
	.ascii "y\0"
	.byte	0x1
	.byte	0x2
	.byte	0x9
	.long	0xb3
	.uleb128 0x2
	.byte	0x91
	.sleb128 -20
	.byte	0
	.byte	0
	.section	.debug_abbrev,"dr"
.Ldebug_abbrev0:
	.uleb128 0x1
	.uleb128 0x11
	.byte	0x1
	.uleb128 0x25
	.uleb128 0x8
	.uleb128 0x13
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x1b
	.uleb128 0x8
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x12
	.uleb128 0x7
	.uleb128 0x10
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0x2
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x12
	.uleb128 0x7
	.uleb128 0x40
	.uleb128 0x18
	.uleb128 0x2116
	.uleb128 0x19
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x3
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x18
	.byte	0
	.byte	0
	.uleb128 0x4
	.uleb128 0x24
	.byte	0
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x3e
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0x8
	.byte	0
	.byte	0
	.uleb128 0x5
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x12
	.uleb128 0x7
	.uleb128 0x40
	.uleb128 0x18
	.uleb128 0x2117
	.uleb128 0x19
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x6
	.uleb128 0x5
	.byte	0
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x18
	.byte	0
	.byte	0
	.uleb128 0x7
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x39
	.uleb128 0xb
	.uleb128 0x27
	.uleb128 0x19
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x12
	.uleb128 0x7
	.uleb128 0x40
	.uleb128 0x18
	.uleb128 0x2117
	.uleb128 0x19
	.byte	0
	.byte	0
	.byte	0
	.section	.debug_aranges,"dr"
	.long	0x2c
	.word	0x2
	.secrel32	.Ldebug_info0
	.byte	0x8
	.byte	0
	.word	0
	.word	0
	.quad	.Ltext0
	.quad	.Letext0-.Ltext0
	.quad	0
	.quad	0
	.section	.debug_line,"dr"
.Ldebug_line0:
	.section	.debug_str,"dr"
	.ident	"GCC: (x86_64-posix-seh-rev0, Built by MinGW-W64 project) 8.1.0"
