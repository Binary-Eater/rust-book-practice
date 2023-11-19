#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /* NOTE: typical practice is to make width() a getter for width */
    fn width(&self) -> bool {
        self.width > 0
    }
}

/* Can have separate impl blocks for the same type */
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function but not a method of Rectangle */
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    /* Iteration 1
     *
     * let width1 = 30;
     * let height1 = 50;
     *
     * println!(
     *     "The area of the rectangle is {} square pixels.",
     *     area(width1, height1)
     * );
     */

    /* Iteration 2
     *
     * let rect1 = (30, 50);
     *
     * println!(
     *     "The area of the rectangle is {} square pixels.",
     *     area(rect1)
     * );
     */

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect2 is {}", rect2); // ERROR - requires Display trait

    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);

    /* dbg! macro takes and returns ownership of expression, unlike println!
     * which borrows.
     */

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!(
        "The area of the square rectangle is {} square pixels.",
        sq.area()
    );
}

/* Iteration 1
 *
 * fn area(width: u32, height: u32) -> u32 {
 *     width * height
 * }
 */

/* Iteration 2
 *
 * fn area(dimensions: (u32, u32)) -> u32 {
 *     dimensions.0 * dimensions.1
 * }
 */

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
