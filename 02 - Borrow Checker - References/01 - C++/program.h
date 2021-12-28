#pragma once

#include <cstdint>
#include <iostream>

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

    std::cout << "A: " << a.get_value() << std::endl;
    std::cout << "B: " << b.load() << std::endl;
}
