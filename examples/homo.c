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

    x = t1();
    x = t2();
    x = t3();

    return 0;
}
