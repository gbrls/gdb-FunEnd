/* main.c simple program to test assembler program */

#include <stdio.h>
#define i64 long long int
#define u64 unsigned long long int

extern i64 qst1(i64 a, i64 b);
extern i64 qst2(i64 a, i64 b, i64 c);
extern i64 qst3();
extern i64 qst4(i64 a);
extern i64 qst5(i64 a);
extern i64 qst6(char* s);
extern i64 qst7(char* s);

extern i64 qst8(i64 x);
extern i64 fat(i64 x);
extern i64 mod10(i64 x);

extern i64 div(i64 x, i64 y);
extern i64 min(i64 x, i64 y);

extern i64 qst9(char* set, char* pat);

void assert_eq(i64 a, i64 b) {
	if(a != b) {
		//printf("[ERROR] Expected %lld found %lld\n", b, a);
	} else {
		//printf("[OK]\n");
	}
}

i64 qst1_sol(i64 a, i64 b) {
	i64 m = a;

	if (b == m) m = b - a;
	else m = a - b;

	return m;
}

void test1(i64 x, i64 y) {
	i64 a = qst1(x, y);
	i64 b = qst1_sol(x, y);

	assert_eq(a, b);
}

int example_string(char* str) {
    return str[5] - 5;
}


int main(void)
{

    test1(5, 5);
    test1(10, 3);
    test1(3, 10);

    //assert_eq(qst2(1, 60, 30), 1);
    //assert_eq(qst2(0, 64, 25), 1);
    //assert_eq(qst2(-1, 65, 24), 0);
    //assert_eq(qst2(10, 65, 24), 0);
    //assert_eq(qst2(-1, 50, 24), 0);
    //assert_eq(qst2(-1, 65, 25), 0);

    //i64 a = qst3();
    //i64 b = qst4(0x0F0FF);

    //assert_eq(qst6("vogAIs do AlfaBETO oU?"), 6);
    //assert_eq(qst7("GOTICO"), 10);

    //for(int i = 0; i <= 12; i++) {
    //    printf("%d % 10 = %d\n", 2055 + i, mod10(2055 + i));
    //}
    //i64 z = fat(8);

    //i64 x = qst8(1234);
    //i64 y = qst8(678);
    //assert_eq(x, 33);
    //assert_eq(y, 46080);

    //i64 w = qst9("grnrclszemskvbgcluwtgyvieip", "leg"); // 2

    //i64 u = div(10, 3);
    //i64 v = div(120, 10);
    //i64 i = div(5, 1);
    //i64 j = div(5, 2);
    //i64 k = div(10, 12);
    //i64 m = div(5, 6);
    //i64 n = div(4, 4);

    //i64 x = min(10, 2);
    //i64 y = min(-3, 20);

    return 0;
}
