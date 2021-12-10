use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
struct A {
    value: String,
}

impl A {
    fn new(value: String) -> A {
        A { value }
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn set_value(&mut self, value: String) {
        self.value = value;
    }
}

fn main() {
    let mut map: HashMap<A, String> = HashMap::new();

    let mut a = A::new(String::from("A"));

    map.insert(a, String::from("A"));
    map.insert(A::new(String::from("B")), String::from("B"));
    map.insert(A::new(String::from("C")), String::from("C"));

    a.set_value(String::from("D"));

    println!("A: {:?}", map.get(&A::new(String::from("A"))));
    println!("B: {:?}", map.get(&A::new(String::from("B"))));
    println!("C: {:?}", map.get(&A::new(String::from("C"))));
    println!("D: {:?}", map.get(&A::new(String::from("D"))));
}
