#![cfg_attr(rustfmt, rustfmt_skip)]

use std::fmt::Display;

/* a outlives b
 *
 * fn main() {
 *     let r;                // ---------+-- 'a
 *                           //          |
 *     {                     //          |
 *         let x = 5;        // -+-- 'b  |
 *         r = &x;           //  |       |
 *     }                     // -+       |
 *                           //          |
 *     println!("r: {}", r); //          |
 * }                         // ---------+
*/

/* Lifetime annotation syntax
 *
 * &i32        // a reference
 * &'a i32     // a reference with an explicit lifetime
 * &'a mut i32 // a mutable reference with an explicit lifetime
 */

// Example of a generic lifetime annotation.
// Ensures that the return value has the same lifetime as the minimal lifetime
// between x and y.
// Lifetime contracts are made explicitly part of the language semantics to
// simplify complexity in the compiler.
/* Quoting the Rust Book
 *
 * If there’s a problem with the way a function is annotated or the way it is
 * called, the compiler errors can point to the part of our code and the
 * constraints more precisely. If, instead, the Rust compiler made more
 * inferences about what we intended the relationships of the lifetimes to be,
 * the compiler might only be able to point to a use of our code many steps away
 * from the cause of the problem.
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/* Good lifetime annotation practices
 *
 * fn longest<'a>(x: &'a str, y: &str) -> &'a str {
 *     x
 * }
 *
 * Since only x is returned, constrain the lifetime of the return value to x's
 * lifetime, not lifetime_min(x, y).
 */

struct ImportantExcerpt<'a> {
    part: &'a str,
}

/* Lifetime elision in Rust
 *
 * Rust developers found that there were certain simple patterns that could be
 * simplified with deterministic implicit lifetime rules to avoid needing to
 * annotate all functions with lifetime annotations. Chapter 4's first_word
 * function is an example of a function that works with implicit lifetime
 * elision rules.
 *
 * Rule 1
 *
 *     Each input reference parameter gets its own lifetime.
 *     fn foo<'a>(x: &'a i32)
 *     fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
 *
 * Rule 2
 *
 *     If there is exactly one input lifetime parameter, all output lifetime
 *     parameters get the same assignment
 *     fn foo<'a>(x: &'a i32) -> &'a i32
 *
 * Rule 3
 *
 *     Multiple input lifetime parameters but one of them is &self or &mut self
 *     (a method), the lifetime of self is assigned to all output parameters
 *     fn foo<'a, 'b>(&'a self, x: &'b i32) -> &'a i32
 */

impl<'a> ImportantExcerpt<'a> {
    // No lifetime annotation needed since return value is not a reference
    fn level(&self) -> i32 {
        3
    }

    // First and third rule of lifetime elision apply here
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Combining generic type parameters, trait bounds, and lifetime annotations
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 'static is the only lifetime annotation that can change a references
    // lifetime. Think static keyword in C programs. All string literals have a
    // 'static lifetime, similar to C in the sense that they are valid for the
    // entire duration of the program. Be vary of using 'static. Typically
    // compiler suggestions for using 'static are wrong and most likely the
    // issue is a basic design problem related to lifetime management.
    let s: &'static str = "I have a static lifetime.";

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    // Same lifetime as first_sentence/novel
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        // result has the same lifetime as string2, not string1, even though a
        // reference to string1's string slice is returned
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let result;
    {
        let string2 = String::from("xyz");
        // result has the same lifetime as string2, not string1, even though a
        // reference to string1's string slice is returned
        result = longest(string1.as_str(), string2.as_str());
    }
    //println!("The longest string is {}", result);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
