struct A {
    x: i32,
    y: Option<i32>,
}

impl A {
    fn new(value: i32) -> A {
        A { x: value, y: None }
    }

    fn fun(&mut self) -> i32 {
        let mut result = 0;
        for i in 0..self.x {
            result += i;
            self.y = Some(self.y.unwrap_or(0) + 2);
            result -= self.y.unwrap();
        }
        result
    }

    fn fun2(&mut self, value: i32) {
        self.y = Some(value);
    }
}

fn main() {
    let mut a = A::new(32);
    println!("Result: {}", a.fun());
    a.fun2(-12);
    println!("Result: {}", a.fun());
}
