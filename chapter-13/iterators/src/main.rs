/* Iterator trait implementation
 *
 * pub trait Iterator {
 *     type Item;
 *
 *     fn next(&mut self) -> Option<Self::Item>;
 *
 *     // methods with default implementations elided
 * }
 */

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    // .map is an adapter that creates a new type of iterator
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
