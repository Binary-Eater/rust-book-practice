use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x) /* not allocated on the heap */
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; /* associated type (learn more in Chapter 20) */

    /* Reference is returned to avoid moving value out of MyBox */
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    //let y = &x;
    //let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m); /* deref coercion */
}
