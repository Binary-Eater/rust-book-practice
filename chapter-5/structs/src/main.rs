struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/* Tuple structs */
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

/* Unit-like structs */
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // must by last for filling in remaining fields by moving
                // semantics for underlying move types. If Copy trait only semantics are
                // used, user1 would still be valid.
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
