	.globl qst1, qst2, qst3, qst4, qst5, qst6, qst7, qst8, fat, mod10, qst9, div, min

qst1:
	sub sp, sp, #0x20    // alocando memoria na pilha
	stur x0, [sp, #0x0]  // a
	stur x1, [sp, #0x8]  // b

	// Solucao em si
	ldur x0, [sp, #0x0]
	ldur x1, [sp, #0x8]
	sub x0, x0, x1
	stur x0, [sp, #0x10]
	// Fim da solucao

	add sp, sp, #0x20 // liberando o stack frame
	br X30			  // retornando do procedimento

qst2:
	sub  sp, sp, #0x30   // alocando espaco na pilha
	stur x0, [sp, #0x0]  // a
	stur x1, [sp, #0x8]  // b
	stur x2, [sp, #0x10] // c
	eor  x3, x3, x3 	 // x3 = 0
	stur x3, [sp, #0x18] // x


    // @TODO: Talvez seja possível diminuir ainda
	// Solucao em si
	ldur x0, [sp, #0x0]
	adds x0, x0, #0x0 // testando se a >= 0
	b.mi .exit

	ldur x0, [sp, #0x8]
	subs x0, x0, #65 // (b - 65 < 0) <=> (b <= 64)
	b.pl exit

	ldur x0, [sp, #0x10]
	subs x0, x0, #25
	b.mi .exit

	movz x0, #1
	stur x0, [sp, #0x18]
	.exit:
	// Fim da solucao

	ldur x0, [sp, #0x18]

	add sp, sp, #0x30 // liberando o stack frame
	br X30			  // retornando do procedimento

// teste

/*
    extrai os bits 11 até 16 do registrador X10 e usa o valor desse campo para
    substituir os bits 26 até 31 no registrador X11 

    11 -> 26 = 15 (lsl 15)
    16 + 15 = 31 (ok)

    16 - 11 = 5 bits

    */
qst3:

    movz x10, #0x0
    orn x10, x10, x10 // x10 = ~0

    movz x11, #0x10FE
    mov x7,   #0xFF0F
    mul x11, x11, x7
    mul x11, x11, x7
    mul x11, x11, x7 // escrevendo algo grande em x11 para testar


    // Solucao em si
    movz x12, #0x3F // máscara de 5 bits 00011111
    and x9, x10, x12, lsl 11  // 11-16 x10 mask
    and x8, x11, x12, lsl 26  // 26-31 x11 mask


    eor x11, x11, x9, lsl 15 // xoring x10's mask into x11
    eor x11, x11, x8         // removing original x11 bits
    // Fim da solucao

    br X30			  // retornando do procedimento

// x8 =      1010100 00000000 00000000 00000000
// x11 =     1010100 01110010 01001010 10100010
// new x11 = 1111100 01110010 01001010 10100010

qst4:
    mov x11, x0

    // Solucao em si
    eor x10, x10, x10
    sub x10, x10, #1 // Setando todos os bits de x10 
    eor x10, x11, x10
    // Fim da solucao

    mov x0, x10

    br x30

qst5:

    // a) Usaria o formato CB pois nele é possível passar um endereço para o
    // goto e um registrador para a operação

    .loop:

    // inicio da solução
    cbz x12, .exit2
    sub x12, x12, #1
    b .loop
    // fim da solução
    .exit2:
    br x30

qst6:
    eor x3, x3, x3

    mov x4, 'A'
    mov x5, 'E'
    mov x6, 'I'
    mov x7, 'O'
    mov x8, 'U'

    loop_start:
        ldurb w1, [x0, #0]
        cbz x1, loop_end

        add x0, x0, #1

        eor x2, x1, x4
        cbz x2, is_vowel
        eor x2, x1, x5
        cbz x2, is_vowel
        eor x2, x1, x6
        cbz x2, is_vowel
        eor x2, x1, x7
        cbz x2, is_vowel
        eor x2, x1, x8
        cbz x2, is_vowel

        b loop_start
    is_vowel:
        add x3, x3, #1
        b loop_start
    loop_end:

    mov x0, x3

    br x30

qst7:
	sub sp, sp, #0x100    // alocando memoria na pilha
    mov x1,  #1
    mov x2,  #2
    mov x3,  #3
    mov x4,  #4
    mov x5,  #5
    mov x8,  #8
    mov x10, #10

    mov x11, sp

    sturb w1, [sp, 'A']
    sturb w1, [sp, 'E']
    sturb w1, [sp, 'I']
    sturb w1, [sp, 'O']
    sturb w1, [sp, 'U']
    sturb w1, [sp, 'N']
    sturb w1, [sp, 'R']
    sturb w1, [sp, 'S']

    sturb w2, [sp, 'D']
    sturb w2, [sp, 'G']
    sturb w2, [sp, 'T']

    sturb w3, [sp, 'B']
    sturb w3, [sp, 'C']
    sturb w3, [sp, 'M']
    sturb w3, [sp, 'P']

    sturb w4, [sp, 'F']
    sturb w4, [sp, 'H']
    sturb w4, [sp, 'V']
    sturb w4, [sp, 'W']
    sturb w4, [sp, 'Y']

    sturb w5, [sp, 'K']

    sturb w8, [sp, 'J']
    sturb w8, [sp, 'L']
    sturb w8, [sp, 'X']

    sturb w10, [sp, 'Q']
    sturb w10, [sp, 'Z']

    eor x3, x3, x3

    loop_start2:
        ldurb w1, [x0, #0]
        cbz x1, loop_end2

        add x0, x0, #1

        add x4, x1, x11
        ldurb w4, [x4, #0]
        add x3, x3, x4

        b loop_start2
    loop_end2:

	add sp, sp, #0x100    // desalocando memoria da pilha

    mov x0, x3

    br x30


fat:
    mov x1, #1

    start_fat:
    cbz x0, end_fat

    mul x1, x1, x0
    sub x0, x0, #1

    b start_fat
    end_fat:
    mov x0, x1

    br x30

mod10:

    eor x1, x1, x1
    mod_start:

    subs x2, x0, #10
    b.mi mod_end

    sub x0, x0, #10
    add x1, x1, #1

    b mod_start
    mod_end:

    br x30

qst8:
    eor x10, x10, x10 // x10 vai guardar a resposta
    mov x3, x0        // x3 guarda o valor que estamos dividindo
    mov x12, x30

    start8:
    cbz x3, end8

    mov x0, x3
    bl mod10
    mov x11, x1 // x11 = x0/10, x0 = x0%10
    bl fat

    add x10, x10, x0

    mov x3, x11

    b start8
    end8:

    mov x0, x10

    mov x30, x12
    br x30

div:
    eor x2, x2, x2
    cbz x1, div_end // avoiding division by 0
    div_start:

        subs x3, x0, x1
        b.mi div_end

        subs x0, x0, x1
        add x2, x2, #1

        b div_start
    div_end:

    mov x0, x2

    br x30

min:
    subs x2, x0, x1
    b.mi min_b
    mov x0, x1
    min_b:
    br x30

qst9:
	sub sp, sp, #0x300    // alocando dois vetores de frequencia

    mov x10, 0
    mov x8, 0
    mov x12, x1
    mov x17, x30

    memset_start:
    subs x9, x10, #0x300
    b.pl memset_end

        add x7, sp, x10

        add x10, x10, #1
        sturb w8, [x7, #0]

    b memset_start
    memset_end:

    // setando o primeiro vetor de frequencia
    loop_start8:
        ldurb w1, [x0, #0]
        cbz x1, loop_end8

        add x0, x0, #1

        add x7, sp, x1

        // sp[x1]++
        ldurb w8, [x7, #0]
        add x8, x8, #1
        sturb w8, [x7, #0]

        b loop_start8
    loop_end8:
    mov x0, x12
    // setando o segundo vetor de frequencia
    loop_start9:
        ldurb w1, [x0, #0]
        cbz x1, loop_end9

        add x0, x0, #1

        add x7, sp, x1
        add x7, x7, #0x100 // offset do segundo vetor na memória

        // sp[x1]++
        ldurb w8, [x7, #0]
        add x8, x8, #1
        sturb w8, [x7, #0]

        b loop_start9
    loop_end9:


    eor x4, x4, x4
    mov x13, #0x100 // x5 guarda a resposta
    mloop_start:

    subs x5, x4, #0x100
    b.pl mloop_end

    eor x5, x5, x5
    add x5, sp, x4

    add x4, x4, #1

    ldurb w0, [x5, #0]
    add x5, x5, #0x100
    ldurb w1, [x5, #0]

    cbz x1, is_zero

    bl div // x0 = x0 / x1
    mov x1, x13
    bl min // x0 = min(x0, x1)
    mov x13, x0 // x13 = min(x13, x0)

    is_zero:
    b mloop_start
    mloop_end:

    mov x0, x13
	add sp, sp, #0x300    

    mov x30, x17
    br x30
