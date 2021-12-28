#include <cstdint>
#include <iostream>

#include "queue.h"

using namespace std;


void run(Queue& queue) {
    queue.enqueue(21);
    queue.enqueue(42);
    queue.enqueue(84);

    cout << "Expect 21: " << queue.dequeue() << endl;
    cout << "Expect 42: " << queue.dequeue() << endl;
    cout << "Expect 84: " << queue.dequeue() << endl;
}


int main() {
    Queue queue;
    run(queue);
    return 0;
}
