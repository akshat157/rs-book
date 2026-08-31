use aggregator::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("akshat"),
        content: String::from("Hey everyone! This is my hello world post here!"),
        reply: false,
        repost: false,
    };
    println!("1 new post: {}", post.summarize());
}
