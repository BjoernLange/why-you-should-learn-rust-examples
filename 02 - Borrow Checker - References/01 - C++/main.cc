#include <cstdint>
#include <iostream>

using namespace std;

class A {
public:
    A(uint32_t value) : value(value) {}

    void set_value(uint32_t value) {
        this->value = value;
    }

    uint32_t get_value() {
        return value;
    }

private:
    uint32_t value;
};

class B {
public:
    B(A& a) : a(a) {}

    void store(uint32_t value) {
        a.set_value(value);
    }

    uint32_t load() {
        return a.get_value();
    }

private:
    A& a;
};


void run(A& a, B& b) {
    uint32_t x = a.get_value();
    b.store(21);
    a.set_value(x + 42);

    cout << "A: " << a.get_value() << endl;
    cout << "B: " << b.load() << endl;
}


int main() {
    A a(42);
    B b(a);
    run(a, b);
    return 0;
}
