enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Explicitly type annotate since inner type cannot be deduced
    let v: Vec<i32> = Vec::new();

    // Inner type deduced as i32
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    // [] will panic when out-of-bounds
    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third_option: Option<&i32> = v2.get(2);
    match third_option {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    /* Cannot operate on an immutable reference and mutable reference
     * in the same scope.
     *
     * let mut v4 = vec![1, 2, 3, 4, 5];
     *
     * let first = &v4[0];
     *
     * v4.push(6);
     *
     * println!("The first element is: {first}");
     *
     * The above does not compile due to the the immtable reference
     * to the first element in v4 while needing to update v4 using
     * the mutable reference. (Think realloc invalidating pointers)
     */

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50;
    }
    for i in &v5 {
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
}
