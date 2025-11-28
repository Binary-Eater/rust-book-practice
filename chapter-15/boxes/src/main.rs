/* cons list are a functional programming concept */
enum List {
    Cons(i32, Box<List>), /* Second member is a pointer (fixed-size) */
    //Cons(i32, List), /* Cannot compute the size due to recursive type */
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5); /* allocated on the heap */
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
