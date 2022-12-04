use crate::error;
use crate::reddit;
use std::path::Path;
use regex::Regex;
use std::sync::Arc;

pub struct Subreddit<'a> {
    pub reddit: &'a reddit::Reddit<'a>,
    pub name: String,
}

async fn process_image(cl: reqwest::Client, url: String, reg: Arc<Regex>) -> error::RedditResult<()> {
    let cursor = std::io::Cursor::new(cl.get(&url).send().await?.bytes().await?);
    let img = image::io::Reader::new(cursor).with_guessed_format()?.decode()?;
    if std::cmp::max(img.width(), img.height()) >= 1600 {
        let file_name_prefix;
        if img.width() > img.height() {
            file_name_prefix = "sir";
        } else {
            file_name_prefix = "vys";
        }
        let file_name = format!(
            "{}_{}",
            file_name_prefix,
            reg.find(&url).unwrap().as_str()
        );
        if !Path::new(&file_name).exists() {
            let mut file = tokio::fs::File::create(file_name).await?;
            let mut buff = std::io::Cursor::new(Vec::new());
            img.write_to(&mut buff, image::ImageOutputFormat::Jpeg(100))?;
            buff.set_position(0);
            tokio::io::copy(&mut buff, &mut file).await?;
        }
    }
    Ok(())
}

impl<'a> Subreddit<'a> {
    async fn url(&self, secondary: &str) -> String {
        format!("https://www.reddit.com/r/{}/{}", self.name, secondary)
    }

    async fn images_for(&self, sort: &str) -> error::RedditResult<serde_json::Value> {
        let sub_url = self.url(sort).await;
        println!("Downloading data from: {}", sub_url);
        let request = self.reddit.client.get(sub_url);
        let response = request.send().await?;
        Ok(serde_json::from_str(&response.text().await?)?)
    }

    fn get_image_data(&self, entries: &serde_json::Value) -> Vec<serde_json::Value> {
        let array_opt = entries["data"]["children"].as_array();
        match array_opt {
            Some(entry_array) => entry_array.to_owned(),
            None => {
                println!("Unable to locate images for subreddit: {}", self.name);
                vec![]
            }
        }
    }

    pub async fn download_images(&self, sort: &str) -> error::RedditResult<()> {
        let entries = self.images_for(sort).await.unwrap();
        let mut images = vec![];
        for entry in self.get_image_data(&entries) {
            let data = &entry["data"];
            if let serde_json::Value::Object(media_metadata) = &data["media_metadata"] {
                for (image_id, _) in media_metadata {
                    images.push(format!("https://i.redd.it/{}.jpg", image_id));
                }
            } else if let Some(url) = data["url"].as_str() {
                if url.ends_with(".jpg") {
                    images.push(String::from(url));
                }
            }
        }

        let file_name_regex = Arc::new(Regex::new("[^/]+$").unwrap());
        let mut to_join = vec![];
        for image_url in images {
            let reg = file_name_regex.clone();
            let url = String::from(image_url);
            let cl = self.reddit.client.clone();
            to_join.push(tokio::spawn(async move {
                process_image(cl, url, reg).await.unwrap();
            }));
        }
        let mut ok = true;
        for thread in to_join {
            match thread.await {
                Err(e) => { println!("{}", e); ok = false; },
                _ => {}
            }
        }
        if ok {
            Ok(())
        } else {
            Err("Thread joining failed")?
        }
    }
}
