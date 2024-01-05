use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        },
    };

    /* Alternative using closures
     *
     * let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
     *     if error.kind() == ErrorKind::NotFound {
     *         File::create("hello.txt").unwrap_or_else(|error| {
     *             panic!("Problem creating the file: {:?}", error);
     *         })
     *     } else {
     *         panic!("Problem opening the file: {:?}", error);
     *     }
     * });
     */

    /* unwrap/expect call panic! if Result is an Err type
     *
     * let greeting_file = File::open("hello.txt").unwrap();
     *
     * let greeting_file = File::open("hello.txt")
     *     .expect("hello.txt should be included in this project");
     */

    /* ? operator can only be used with functions that return a Result, Option,
     * or FromResidual trait implementer
     *
     * let greeting_file = File::open("hello.txt")?;
     */

    // Works with Result<(), Box<dyn Error>> return type
    let greeting_file = File::open("hello.txt")?;

    // main can return any type that implements the std::process::termination
    // trait
    Ok(())
}

// Error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    /* Verbose form
     *
     * let username_file_result = File::open("hello.txt");
     *
     * let mut username_file = match username_file_result {
     *     Ok(file) => file,
     *     Err(e) => return Err(e),
     * };
     */
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();

    /* Verbose form
     *
     * match username_file.read_to_string(&mut username) {
     *     Ok(_) => Ok(username),
     *     Err(e) => Err(e),
     * }
     */
    username_file.read_to_string(&mut username)?;

    // From trait can be used from error conversion

    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)

    /* One-liner equivalent to the above
     *
     * fs::read_to_string("hello.txt")
     */
}

// ? operator on Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    // Can return None pre-emptively
    text.lines().next()?.chars().last()
}
