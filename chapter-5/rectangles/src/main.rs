#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
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
