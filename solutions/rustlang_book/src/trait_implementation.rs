use std::println;

struct Article {
    author: String,
    headline: String,
    content: String,
}

impl Summarize for Article {
    fn summarize(&self) -> String {
        return format!("{},  by {}", self.author, self.headline);
    }
}
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summarize for Tweet {
    fn summarize(&self) -> String {
        return format!("{},  by {}", self.username, self.content);
    }
}
trait Summarize {
    fn summarize(&self) -> String;
}

pub fn content_summarization() {
    let article = Article {
        author: String::from("Caragher"),
        headline: String::from("Umpalumpas?"),
        content: String::from("UMpalumpa uwu quties uwu"),
    };

    let tweet = Tweet {
        username: "Read Head".to_string(),
        content: "Woooa wooooa".to_string(),
        reply: true,
        retweet: false,
    };

    println!("{} - {}", article.summarize(), article.content);
    println!(
        "{} - {} - {}",
        tweet.summarize(),
        tweet.reply,
        tweet.retweet
    );
}
