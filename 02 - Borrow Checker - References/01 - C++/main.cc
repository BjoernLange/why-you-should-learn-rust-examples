#include "program.h"

int main() {
    A a(42);
    B b(a);
    run(a, b);
    return 0;
}
