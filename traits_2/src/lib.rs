/// Shared behavior for types that can produce a short text summary.
///
/// The trait defines the common API for "summarizable" items while allowing each
/// type to provide its own wording. A default implementation is supplied for
/// `summarize`, and implementors may override it if they need a more specific
/// description.
pub trait Summary {
    /// Returns the author's name or handle for the item being summarized.
    fn summarize_author(&self) -> String;

    /// Produces a concise summary for the item.
    ///
    /// This default is intentionally generic and is used when a type does not
    /// provide a more specific summary implementation.
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

/// A news item that can be summarized in a publication-friendly format.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    /// News summaries start with the headline, then the author, then the location.
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    /// News articles identify their author using the author's name.
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

/// A social media post that can be summarized with its author and a short body.
pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    /// Social posts are summarized as "username: message".
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    /// Social posts use the account handle as the author label.
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
