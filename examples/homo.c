#include <stdio.h>

int main() {
    int arr[] = {1,2,3};
    int* p = &arr[0];

    printf("%d\n",*p);

    return 0;
}
