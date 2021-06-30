#include <stdio.h>

int t1() {
    return 0x99;
}

int t2() {
    return 0x88;
}

int t3() {
    return 0x77;
}

int main() {
    int arr[] = {1,2,3};
    int* p = &arr[0];

    printf("%d\n",*p);

    int x;
    unsigned char* fnp;

    x = t1();
    x = t2();

    fnp = &t3;

    x = t3();

    int a;
    a = 10;
    // this is basically a NOP
    while(a--) {}

    return 0;
}
