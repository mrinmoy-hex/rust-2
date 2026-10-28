trait Summary {
    fn get_author(&self) -> &str;
    fn summarize(&self) -> String {
        format!("{}: (Read More)...", self.get_author())
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let content = format!("News by {}: {}", self.author, self.content);
        content
    }

    fn get_author(&self) -> &str {
        self.author.as_str()
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Tweet {
    fn new(username: String, content: String, reply: bool, retweet: bool) -> Self {
        Self {
            username,
            content,
            reply,
            retweet,
        }
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let content = format!("Tweet by {}: {}", self.username, self.content);
        content
    }

    fn get_author(&self) -> &str {
        self.username.as_str()
    }
}

fn main() {
    // a trait defines the functionality a particular type has and can share with other types
    // we can use traits to defien shared behavior in an abstract way

    // we can use trait bounds to specify that a generic type can be any type that has certain
    // behavior
    //
    let tweet = Tweet::new(
        String::from("mrinmoy-hex"),
        String::from("This is a test content just to check my skill"),
        false,
        false,
    );

    let news_article = NewsArticle {
        author: String::from("default user"),
        content: String::from("Testing a news article"),
        headline: String::from("Hello World"),
        location: String::from("Mars"),
    };

    news_aggregator(&tweet);
    news_aggregator(&news_article);

    mixup_news(&tweet, &news_article);

    println!("{}", tweet.summarize());
}

fn news_aggregator(source: &impl Summary) {
    println!("There is a new news in market");
    // println!("The news is that {} and it is published by {}", tweet.cotent, tweet.username);
    println!("{}", source.summarize());
}
//
// fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }
//
// fn mixup_news<T: Summary, U: Summary>(primary: &T, other: &U) {
//     println!("{} and {}", primary.summarize(), other.summarize());
// }
//

fn mixup_news(primary: &impl Summary, other: &impl Summary) {
    println!("{} and {}", primary.summarize(), other.summarize());
}
