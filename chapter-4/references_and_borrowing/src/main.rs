fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&mut s1);

    println!("s1 after mutation is '{}'.", s1);

    /* Mutable references
     *
     * Ownership model only allows one mutable reference to an instance to exist
     * at a time. Therefore, the below example is illegal. The goal is to
     * prevent data races.
     *
     * let r1 = &mut s1;
     * let r2 = &mut s1;
     *
     * println!("{}, {}", r1, r2);
     */

    {
        let _r1 = &mut s1; // no problem
    } // _r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s1; // no problem

    /* Even a single mutable reference in the same context makes any other
     * reference illegal.
     *
     * let r3 = &s1; // BIG PROBLEM
     */

    println!("{}", r2);
    // variable r2 will not be used after this point

    let r3 = &s1; // no problem
    println!("{}", r3);

    //let reference_to_nothing = dangle();

    let moved_value = no_dangle();
    println!("{}", moved_value);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/* Borrow checker prevents dangling references.
 *
 * fn dangle() -> &String {
 *     let s = String::from("hello");
 *
 *     &s
 * }
 */

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
