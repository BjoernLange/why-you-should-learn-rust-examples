struct A {
    x: i32,
    y: i32,
}

impl A {
    fn new(value: i32) -> A {
        A { x: value }
    }

    fn fun(&mut self) -> i32 {
        let result = 0;
        for i in 0..self.x {
            result += i;
            self.y += 2;
            result -= self.y;
        }
        result
    }

    fn fun2(&mut self, value: i32) {
        self.y = value;
    }
}

fn main() {
    let mut a = A::new(32);
    println!("Result: {}", a.fun());
    a.fun2(-12);
    println!("Result: {}", a.fun());
}
