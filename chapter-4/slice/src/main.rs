fn main() {
    let mut s = String::from("hello world");

    let _word = first_word(&s); // _word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // _word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. _word is now totally invalid!

    let s1 = String::from("hello world");

    let _hello = &s1[..5];
    let _world = &s1[6..];

    let word = first_word_slice_ret(&s1);

    /* mutable borrow cannot be taken here due to immutable slice reference in
     * this context.
     *
     * s.clear(); // error!
     */

    println!("the first word is: {}", word);

    /* String literals are of slice type &str */

    let my_string = String::from("hello world");

    // `first_word_slice` works on slices of `String`s, whether partial or whole
    let _word = first_word_slice(&my_string[0..6]);
    let _word = first_word_slice(&my_string[..]);
    // `first_word_slice` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word_slice(&my_string);

    let my_string_literal = "hello world";

    // `first_word_slice` works on slices of string literals, whether partial or whole
    let _word = first_word_slice(&my_string_literal[0..6]);
    let _word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word_slice(my_string_literal);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice_ret(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
