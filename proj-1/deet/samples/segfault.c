#include <stdio.h>

void func2(int a) {
    printf("About to segfault... a=%d\n", a);
    *(int*)0 = a;
    printf("Did segfault!\n");
}

void func1(int a) {
    printf("Calling func2\n");
    for(int i = 0; i < 5; i++) {
        printf("called %d\n", i);
    }
    func2(a % 5);
}

int main() {
    func1(42);
    return 0;
}
