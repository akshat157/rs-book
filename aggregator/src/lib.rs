pub trait Summary {
    fn summarize_author(&self) -> String;

    // A default implementation for the summarize method.
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    // Another default implementated method that calls another method inside the trait, which may
    // actually be defined elsewhere.
    fn summarize_by_author(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub location: String,
    pub content: String,
}

// In case we want to use the default implementation of
// the summarize method, we specify an empty impl block
// for that struct.
//
// Comment the contents of the impl block to get that behavior.
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.summarize_author())
    }

    fn summarize_author(&self) -> String {
        format!("{} ({})", self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
