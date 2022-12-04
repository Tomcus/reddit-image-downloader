use crate::subreddit;
use reqwest::Client;

pub struct Reddit<'a> {
    pub client: &'a Client,
}

impl<'a> Reddit<'a> {
    pub fn new(client: &'a Client) -> Self {
        Reddit { client }
    }

    fn subreddit(&'a self, name: &str) -> subreddit::Subreddit<'a> {
        subreddit::Subreddit {
            reddit: self,
            name: String::from(name),
        }
    }

    pub async fn download_images_from_subredits(&self, subreddits: Vec<&str>) {
        let mut to_join = Vec::new();
        for subreddit in subreddits.into_iter() {
            to_join.push(async move {
                let sub = self.subreddit(subreddit);
                sub.download_images("hot.json").await.unwrap();
                sub.download_images("new.json").await.unwrap();
            });
        }
        for joinable in to_join {
            joinable.await;
        }
    }
}
