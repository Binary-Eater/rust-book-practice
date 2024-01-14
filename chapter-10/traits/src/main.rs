use std::fmt::Display;
use traits::aggregator::{Summary, Tweet};

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/* Trait bound syntax
 *
 * impl Trait is just semantic sugar for a trait bound.
 * fn notify(item: &impl Summary) and fn notify<T: Summary>(item: &T) share the
 * same signature.
 *
 * Using a trait bound can be appropriate over the semantic sugar when wanting
 * to constrain the types of parameters to be the same amongst each other.
 *
 * fn notify(item1: &impl Summary, item2: &impl Summary)
 *
 * The above signature allows item1 and item2 to be different types as long as
 * they implement the Summary trait.
 *
 * fn notify<T: Summary>(item1: &T, item2: &T)
 *
 * The above signature enforces that item1 and item2 share the same type that
 * must implement the Summary trait.
 */

// pub fn notify<T: Summary + Display>(item: &T)
pub fn notify_display(item: &(impl Summary + Display)) {
    println!("Breaking news! {} -> {}", item.summarize(), item);
}

/* where Clause
 *
 * fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
 *
 *     is equivalent to
 *
 * fn some_function<T, U>(t: &T, u: &U) -> i32
 * where
 *     T: Display + Clone,
 *     U: Clone + Debug,
 * {
 */

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

/* impl Trait is just semantic sugar
 *
 * fn returns_summarizable(switch: bool) -> impl Summary {
 *     if switch {
 *         NewsArticle {
 *             headline: String::from(
 *                 "Penguins win the Stanley Cup Championship!",
 *             ),
 *             location: String::from("Pittsburgh, PA, USA"),
 *             author: String::from("Iceburgh"),
 *             content: String::from(
 *                 "The Pittsburgh Penguins once again are the best \
 *                  hockey team in the NHL.",
 *             ),
 *         }
 *     } else {
 *         Tweet {
 *             username: String::from("horse_ebooks"),
 *             content: String::from(
 *                 "of course, as you probably already know, people",
 *             ),
 *             reply: false,
 *             retweet: false,
 *         }
 *     }
 * }
 *
 * The above does not compile because NewsArticle and Tweet are diverging types.
 *
 * fn returns_summarizable(switch: bool) -> impl Summary
 *
 *     is equivalent to
 *
 * fn returns_summarizable<T: Summary>(switch: bool) -> T
 *
 *     where T needs to be deduced to a single type.
 */

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Using trait bounds on implementations
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

/* Blanket implementations
 *
 * Can be used to conditionally implement a trait for any type that implements
 * another trait.
 *
 * impl<T: Display> ToString for T {
 *     // --snip--
 * }
 */

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    /*
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    */
}
