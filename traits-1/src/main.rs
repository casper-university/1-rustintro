use aggregator::{Summary, Tweet, NewsArticle};
mod aggregator;

fn main() {
    let article = NewsArticle {
        headline: String::from("Literate horse amazes onlookers"),
        content: String::from(
            "A literal horse is literally literate.",
        ),
        location: String::from("Dublin"),
        author: String::from("Mel Padden"),
    };
    
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people...",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new article: {}", article.summarize());

    summarize_generic(&tweet);
    summarize_generic(&article);
}

fn summarize_generic(item: &dyn Summary) {
    println!("1 new article: {}", item.summarize());
}