fn main() {
    // String is a Vec of bytes
    let mut s = String::new();

    let data = "initial contents";

    // to_string is offered by the Display trait
    let s = data.to_string();

    // the method also works on a liternal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    /* String and str are UTF-8
     *
     * let hello = String::from("السلام عليكم");
     * let hello = String::from("Dobrý den");
     * let hello = String::from("Hello");
     * let hello = String::from("שָׁלוֹם");
     * let hello = String::from("नमस्ते");
     * let hello = String::from("こんにちは");
     * let hello = String::from("안녕하세요");
     * let hello = String::from("你好");
     * let hello = String::from("Olá");
     * let hello = String::from("Здравствуйте");
     * let hello = String::from("Hola");
     */

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // Takes a string slice, &str
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // + is a shorthand for fn add(self, s: &str) -> String
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    /* String does not support indexing
     *
     * let s1 = String::from("hello");
     * let h = s1[0];
     *
     * Unicode scalar values can take two bytes of space, so indexing UTF-8
     * String in Rust is non-sensical. UTF-8 strings can be handled as bytes,
     * scalar values, or grapheme clusters.
     */

    let hello = "Здравствуйте";
    // Will contain the first four bytes of hello which maps to the first two
    // characters of hello
    let s = &hello[0..4];

    /* Partial character slicing is not supported
     *
     * let s = &hello[0..4];
     *
     * This code will panic since it violates a char boundary.
     */

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
            println!("{b}");
    }

    // Support for grapheme clusters is not part of the standard library
}
