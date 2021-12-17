#[derive(Debug)]
struct A {
    value: u32,
}

impl A {
    fn new(value: u32) -> A {
        A { value }
    }

    fn value(&self) -> u32 {
        self.value
    }

    fn set_value(&mut self, value: u32) {
        self.value = value;
    }
}

#[derive(Debug)]
struct B<'a> {
    a: &'a mut A,
}

impl<'a> B<'a> {
    fn new<'x>(a: &'x mut A) -> B<'x> {
        B { a }
    }

    fn store(&mut self, value: u32) {
        self.a.set_value(value);
    }

    fn load(&self) -> u32 {
        self.a.value()
    }
}

fn run(a: &mut A, b: &mut B) {
    let x = a.value();
    b.store(21);
    a.set_value(x + 42);

    println!("A: {:?}", a.value());
    println!("B: {:?}", b.load());
}

fn main() {
    // /*
    let mut a = A::new(42);
    let mut b = B::new(&mut a);
    run(&mut a, &mut b);
    // */

    /*
    let mut a1 = A::new(42);
    let mut a2 = A::new(0);
    let mut b = B::new(&mut a2);
    run(&mut a1, &mut b);
    */
}
