use std::fs::{self, File};
use std::io;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::Path;
use serde::Deserialize;
use futures::{stream, StreamExt};

use crate::sources;

#[derive(Debug)]
pub struct Rule34 {
    images: Arc<Mutex<Vec<String>>>,
    first_person: Vec<String>,
    third_person: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Posts {
    post: Vec<Post>,
}

#[derive(Debug, Deserialize)]
struct Post {
    sample_url: String,
}

impl Rule34 {
    pub fn new(cfg: &crate::Config) -> anyhow::Result<Self> {
        let (first, third) = crate::sources::get_babble(cfg);
        let r = Self {
            images: Arc::new(Mutex::new(Vec::new())),
            first_person: first,
            third_person: third,
        };

        let _ = fs::create_dir_all("./goonto-cache");

        let tags = cfg.image_source.web.tags.clone();
        thread::spawn({
            let clone = Arc::clone(&r.images);
            move || { let _ = stocktake(tags, clone); }
        });

        Ok(r)
    }
}

impl sources::Source for Rule34 {
    fn first_person(&self) -> String {
        crate::sources::random_from(&self.first_person)
    }

    fn third_person(&self) -> String {
        crate::sources::random_from(&self.third_person)
    }

    fn image(&self) -> String {
        crate::sources::take_random(&mut self.images.lock().unwrap())
    }
}

/* TODO: hacky in general */
fn stocktake(tags: Vec<String>, images: Arc<Mutex<Vec<String>>>)
    -> anyhow::Result<()>
{
    tokio::runtime::Runtime::new().unwrap().block_on(async { loop {
        if images.lock().unwrap().len() > 25 {
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        let mut url = "https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&limit=10&tags="
            .to_string();
        url.push_str(&crate::sources::random_from(&tags));

        let client = reqwest::Client::new();

        let resp_text = client
            .get(url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        
        let parsed = serde_xml_rs::from_str::<Posts>(&resp_text)
            .unwrap()
            .post
            .into_iter()
            .map(|p| p.sample_url)
            .collect::<Vec<String>>();

        let imagez = &images;
        let new_images = stream::iter(parsed)
            .map(|image_url| {
                let client = &client;
                let (_, filename) = &image_url.rsplit_once('/').unwrap();
                let out_path = Path::new("./goonto-cache/").join(filename);

                async move {
                    let s = client.get(&image_url)
                        .send()
                        .await
                        .unwrap()
                        .bytes()
                        .await
                        .unwrap()
                        .to_vec();

                    (out_path, s)
                }
            })
            .buffer_unordered(5);

        new_images.for_each(|(out_path, image_data)| async move {
            let mut outf = File::create(out_path.clone()).unwrap();
            io::copy(&mut image_data.as_slice(), &mut outf).unwrap();
            imagez.lock().unwrap().push(out_path.as_path().display().to_string());
        }).await;

        thread::sleep(Duration::from_millis(1000));
    }});

    Ok(())
}