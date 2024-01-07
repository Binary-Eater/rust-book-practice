struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Special function defined for the f32 constraint on Point
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

enum AnotherOption<T> {
    Some(T),
    None,
}

enum AnotherResult<T, E> {
    Ok(T),
    Err(E),
}

struct AnotherPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> AnotherPoint<X1, Y1> {
    // Compose new variants of generic types in function implementations
    // NOTE: this pattern is known as a "mixup"
    fn mixup<X2, Y2>(self, other: AnotherPoint<X2, Y2>) -> AnotherPoint<X1, Y2> {
        AnotherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest number is {}", result);

    // Type deduction occurs for the template parameter
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    /* Type deduction cannot match two types to one type parameter
     *
     * let wont_work = Point { x: 5, y: 4.0 };
     *
     * Adding another type parameter U for the type of y would enable the above
     * to work.
     */

    println!("integer.x = {}", integer.x());

    let p1 = AnotherPoint { x: 5, y: 10.4 };
    let p2 = AnotherPoint { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Rust generics are efficient due to monomorphization
}
