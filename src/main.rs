use reddit::Reddit;
use reqwest::Client;
use std::env;

mod error;
mod reddit;
mod subreddit;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let reddit = Reddit::new(&client);
    let subreddits: Vec<String> = env::args().collect();
    let subreddits_ref: Vec<&str> = (&subreddits[1..]).iter().map(|s| &**s).collect();
    reddit.download_images_from_subredits(subreddits_ref).await;
}
