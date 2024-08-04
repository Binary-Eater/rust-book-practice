use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

/* unwrap_or_else implementation
 *
 * impl<T> Option<T> {
 *     pub fn unwrap_or_else<F>(self, f: F) -> T
 *     where
 *         F: FnOnce() -> T
 *     {
 *         match self {
 *             Some(x) => x,
 *             None => f(),
 *         }
 *     }
 * }
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    _height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    /* Closure type inference and annotation
     *
     * let expensive_closure = |num: u32| -> u32 {
     *     println!("calculating slowly...");
     *     thread::sleep(Duration::from_secs(2));
     *     num
     * };
     *
     * fn add_one_v1(x: u32) -> u32 {
     *     x + 1
     * }
     * let add_one_v2 = |x: u32| -> u32 { x + 1 };
     * let add_one_v3 = |x| x + 1;
     * let add_one_v4 = |x| x + 1;
     *
     * let example_closure = |x| x;
     *
     * let s = example_closure(String::from("hello"));
     * let n = example_closure(5); // Illegal since x is inferred as String
     */

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    println!("Before defining mutable closure: {list:?}");
    let mut borrows_mutably = || list.push(7);

    /* Illegal since a mutable borrow of list occurs when defining
     * borrows_mutably
     *
     * println!("Before calling mutable closure: {list:?}");
     */
    borrows_mutably(); // Borrow of list ends
    println!("After calling mutable closure: {list:?}");

    // move required since closure can outlive current function
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    /* Even with join, borrow checker considers list moved into the closure
     *
     * println!("After calling moved closure: {list:?}");
     */

    let mut list = [
        Rectangle {
            width: 10,
            _height: 1,
        },
        Rectangle {
            width: 3,
            _height: 5,
        },
        Rectangle {
            width: 7,
            _height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    /* FnMut trait cannot move variables out of closures unlike FnOnce
     *
     *
     * let mut sort_operations = vec![];
     * let value = String::from("closure called");
     *
     * list.sort_by_key(|r| {
     *     sort_operations.push(value); // Irreproducible move of value into sort_operations
     *     r.width
     * });
     */

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
