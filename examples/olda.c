#include "b.h"
int fib(int x) {

    x = amazing_func(x, x * x);

    if (x < 2)
        return x;
    else
        return fib(x - 1) + fib(x - 2);
}

int main() {
    int a[5];
    a[5] = 10;
    a[7] = fib(a[5]);

    return 0;
}
