use trpl::{Either, Html};

/* Pseudo state machine managed by the Rust compiler
 *
 * enum PageTitleFuture<'a> {
 *     Initial { url: &'a str },
 *     GetAwaitPoint { url: &'a str },
 *     TextAwaitPoint { response: trpl::Response },
 * }
 */
/*
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
*/

/* Compiler translation of async function
 *
 * use std::future::Future;
 *
 * fn page_title(url: &str) -> impl Future<Output = Option<String>> {
 *     // 'async move' will be discussed in a later chapter
 *     async move {
 *         let text = trpl::get(url).await.text().await;
 *         Html::parse(&text)
 *             .select_first("title")
 *             .map(|title| title.inner_html())
 *     }
 * }
 */

/* async requires a 'runtime'.
 *
 * main can initialize runtimes, but it is not a runtime.
 */
/*
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
*/

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            /* select returns an enum that contains the first completed future
             *
             * enum Either<A, B> {
             *     Left(A),
             *     Right(B),
             * }
             */
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}
