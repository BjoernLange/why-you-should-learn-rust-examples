#!/bin/python3

class A:
    def __init__(self, value):
        self.x = value

    def fun(self):
        result = 0
        for i in range(self.x):
            result = result + i
            self.y = self.y + 2
            result = result - self.y
        return result

    def fun2(self, value):
        self.y = value


def main():
    a = A(32)
    print(f'Result: {a.fun()}')
    a.fun2(-12)
    print(f'Result: {a.fun()}')


if __name__ == '__main__':
    main()
