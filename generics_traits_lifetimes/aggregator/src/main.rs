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

    notify(&article);
    let x = Pair::new(3,5);

    x.cmp_display()
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}