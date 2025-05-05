use trpl::{Either, Html};
use std::future::Future;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // trpl::run(async {
    //     let url = &args[1];
    //     match page_title(url).await {
    //         Some(title) => println!("The title for {url} was {title}"),
    //         None => println!("{url} had no title"),
    //     }
    // })

    trpl::run(async {
        let title_fut_1 = page_url_title(&args[1]);
        let title_fut_2 = page_url_title(&args[2]);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) -> Option<String> {
    let text = trpl::get(url).await.text().await;
    Html::parse(&text)
        .select_first("title")
        .map(|title_element| title_element.inner_html())
}

async fn page_url_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}