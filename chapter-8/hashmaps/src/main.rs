use std::collections::HashMap;

fn main() {
    // Type deduced from insert operations
    // Similar to Haskell
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Rust has a concept of prelude just like Haskell
    // Rust's prelude does not "use" HashMap since it
    // is not a commonly used collection.
    // HashMap also lacks a construction macro like format! or vec!.

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Insert on occurs if key is not present
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // Rust's default has function is SipHash.
    // Can pass a different "hasher" to change algorithm.
    // A "hasher" implements the BuildHasher trait.
}
