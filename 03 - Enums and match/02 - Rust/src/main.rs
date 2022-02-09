use self::A::*;

#[derive(Debug, Clone, Copy)]
enum A {
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl A {
    pub fn iter() -> impl Iterator<Item = A> {
        [U, V, W, X, Y, Z].iter().copied()
    }
}

fn do_something(a: A) {
    match a {
        U => println!("{:?}: U", a),
        V => println!("{:?}: V", a),
        //W => println!("{:?}: W", a),
        X => println!("{:?}: X", a),
        Y => println!("{:?}: Y", a),
        Z => println!("{:?}: Z", a),
        //_ => println!("{:?}: Match all (default case)", a),
    }
}

fn main() {
    A::iter().for_each(do_something);
}
