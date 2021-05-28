#include "b.h"
int do_something(int x, int y) {
    x *= 2;
    y += 5;

    x *= y;

    x -= 1;

    x /= y;

    int z = other_file(x, y);

    x += z;

    return x;
}

int main() {

    int z = do_something(2, 5);

    return 0;
}
