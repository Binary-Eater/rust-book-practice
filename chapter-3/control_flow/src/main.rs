fn main() {
    let mut number = 3; // NOTE: I love that variables are immutable by default

    if number < 5 {
        println!("conditional was true");
    } else {
        println!("conditional was false");
    }

    /* Conditionals must be bool in Rust
     *
     * if number {
     *     println!("number was three");
     * }
     *
     * The above fails to compile since number is not a bool.
     */

    if number != 0 {
        println!("number was something other than zero");
    }

    number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisble by 4, 3, or 2");
    }

    /* if is an expression, not a statement
     *
     * This is based of Ocaml.
     */
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    /* expression arms need to evaluate to the same type
     *
     * This is the same as Ocaml, because of static typing.
     *
     * let number = if condition { 5 } else { "six" };
     *
     * The above does not compile due to the type mismatch between the arms.
     */

    loop {
        println!("again");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for mut element in a {
        println!("the value is: {element}");
        element = 5;
        println!("the 'new' value is: {element}"); // NOTE: only element is updated, not the original arra
    }

    index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
