#pragma once

#include <cstdint>

class Queue {
public:
    Queue() : has_value(false), value(0) {}

    bool enqueue(uint32_t value) {
        if (has_value) {
            return false;
        } else {
            this->value = value;
            has_value = true;
            return true;
        }
    }

    bool empty() const {
        return !has_value;
    }

    uint32_t dequeue() {
        has_value = false;
        uint32_t tmp = value;
        value = 0;
        return tmp;
    }

private:
    bool has_value;
    uint32_t value;
};
