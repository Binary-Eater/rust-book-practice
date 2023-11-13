fn main() {
    // _s is not valid here, it’s not yet declared
    {
        let _s = "hello"; // _s is valid from this point forward

        // do stuff with _s
    } // this scope is now over, and _s is no longer valid

    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    {
        let _s = String::from("hello"); // _s is valid from this point forward

        // do stuff with _s
    } // this scope is now over, and _s is no longer valid
      // (drop is called behind the scenes)

    let x = 5;
    let y = x; // x value is copied into y

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let _s2 = s1; // move semantics by default for heap-related types

    // println!("{}, world!", s1); // s1 cannot be used because it has been moved
    // caught at compile time rather than a runtime problem

    /* Borrow checker adds safety to concepts like move semantics */

    let s3 = String::from("hello");
    let s4 = s3.clone(); // deep copy

    println!("s3 = {}, s4 = {}", s3, s4);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward

    {
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                     // value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    let s5 = String::from("hello");

    let (s6, len) = calculate_length(s5);

    println!("The length of '{}' is {}.", s6, len);
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    // This function can be improved by using references
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
