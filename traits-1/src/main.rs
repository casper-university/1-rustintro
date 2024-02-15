use aggregator::{NewsArticle, Summary, Tweet};
mod aggregator;

fn main() {
    let article = NewsArticle {
        headline: String::from("Literate horse amazes onlookers"),
        content: String::from("A literal horse is literally literate."),
        location: String::from("Dublin"),
        author: String::from("Mel Padden"),
    };

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people..."),
        reply: false,
        retweet: false,
    };

    //println!("1 new tweet: {}", tweet.summarize());
    //println!("1 new article: {}", article.summarize());

    //summarize(&tweet);
    //summarize(&article);
    summarize_list(vec! [ &tweet, &article ] );
}

fn summarize(item: &dyn Summary) {
    println!("1 new article: {}", item.summarize());
}
fn summarize_list(items: Vec<&dyn Summary>) {
    for item in items.into_iter() {
        summarize(item);
    }
}
