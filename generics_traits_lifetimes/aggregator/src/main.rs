use aggregator::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Drizzles devoured rain hopes, Heat wave continues"),
        location: String::from("Bengaluru"),
        author: String::from("Some person"),
        content: String::from("Lorem ipsum")
    };
    println!("Breaking news: {}", article.summarize());
}