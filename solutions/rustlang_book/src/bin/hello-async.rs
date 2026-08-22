use trpl::{Either, Html};

pub async fn page_title(url: &str) -> (&str, Option<String>) {
    // let response = trpl::get(url).await;
    // let response_text = response.text().await;

    let response_text = trpl::get(url).await.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let future_tit_1 = page_title(&args[1]);
        let future_tit_2 = page_title(&args[2]);

        let (url, maybe_title) = match trpl::select(future_tit_1, future_tit_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} return first");

        match maybe_title {
            Some(title) => println!("It's page title was :'{title}'"),
            None => println!("It has not title."),
        }
    })
}
