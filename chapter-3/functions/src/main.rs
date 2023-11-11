fn main() {
    /* Statements vs Expressions
     *
     * Pretty common topic in PL theory courses.
     *
     * Cannot assign values from statements.
     * Can only use expressions to assign values in statements.
     *
     * let x = (let y = 6);
     *
     * The above is illegal since the rhs is a let statement (same as Ocaml).
     *
     * New scope blocks are expressions.
     */
    let x = plus_one(5);
    let y = {
        let x = 3;
        x + 1 // NOTE: adding a semicolon will cause the block scope to evaluate to a unit type value
    };

    println!("The value of x is: {x}");
    println!("The value of y is: {y}");

    print_labeled_measurement(5, 'h');
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
