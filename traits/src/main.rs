/// Shared behavior for values that can provide a short summary and an author.
trait Summary {
    /// Returns the author or account name associated with this item.
    fn get_author(&self) -> &str;

    /// Creates a short summary, using the author's name by default.
    fn summarize(&self) -> String {
        format!("{}: (Read More)...", self.get_author())
    }
}

/// A news story with its publication details and full content.
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    /// Includes the article's author and content in the summary.
    fn summarize(&self) -> String {
        let content = format!("News by {}: {}", self.author, self.content);
        content
    }

    fn get_author(&self) -> &str {
        self.author.as_str()
    }
}

/// A short social-media post.
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Tweet {
    /// Creates a post, recording whether it is a reply or a repost.
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
    /// Includes the account name and post text in the summary.
    fn summarize(&self) -> String {
        let content = format!("Tweet by {}: {}", self.username, self.content);
        content
    }

    fn get_author(&self) -> &str {
        self.username.as_str()
    }
}

fn main() {
    // A trait describes behavior that unrelated types can share. Both values
    // below can be passed to `news_aggregator` because both implement Summary.
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

/// Accepts any borrowed value whose type implements [`Summary`].
fn news_aggregator(source: &impl Summary) {
    println!("There is a new news in market");
    println!("{}", source.summarize());
}

/// Combines summaries from two values that may have different concrete types.
fn mixup_news(primary: &impl Summary, other: &impl Summary) {
    println!("{} and {}", primary.summarize(), other.summarize());
}

#[cfg(test)]
mod tests {
    use super::*;

    // Implementing only `get_author` exercises the trait's default summary.
    struct AuthorOnly {
        author: String,
    }

    impl Summary for AuthorOnly {
        fn get_author(&self) -> &str {
            &self.author
        }
    }

    #[test]
    fn default_summary_uses_the_author() {
        let item = AuthorOnly {
            author: String::from("Ada"),
        };

        assert_eq!(item.summarize(), "Ada: (Read More)...");
    }

    #[test]
    fn tweet_summary_includes_the_account_and_post() {
        let tweet = Tweet::new(
            String::from("rustacean"),
            String::from("Traits share behavior."),
            false,
            false,
        );

        assert_eq!(
            tweet.summarize(),
            "Tweet by rustacean: Traits share behavior."
        );
    }

    #[test]
    fn article_summary_includes_its_author_and_content() {
        let article = NewsArticle {
            headline: String::from("Rust traits"),
            location: String::from("Online"),
            author: String::from("Ada"),
            content: String::from("Traits define shared behavior."),
        };

        assert_eq!(
            article.summarize(),
            "News by Ada: Traits define shared behavior."
        );
    }
}
