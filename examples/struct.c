struct myS {
    int a, b;
};

int main() {
    int x, y;
    struct myS s = {.a=10, .b=20};

    x = s.a + s.b;
    y = 0;
    y += 1;


    return x ^ y;
}
