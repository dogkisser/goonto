use std::fs::{self, File};
use std::io;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use rand::prelude::{IteratorRandom, SliceRandom};
use serde::Deserialize;
use futures::{stream, StreamExt};

use crate::sources;

#[derive(Debug)]
pub struct E621 {
    images: Arc<Mutex<Vec<String>>>,
    prompts: Vec<String>,
    babble: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct E6Posts {
    posts: Vec<E6Post>,
}

#[derive(Debug, Deserialize)]
struct E6Post {
    sample: E6File,
}

#[derive(Debug, Deserialize)]
struct E6File {
    url: Option<String>,
}

impl E621 {
    pub fn new(tags: Vec<String>) -> anyhow::Result<Self> {
        let r = Self {
            images: Arc::new(Mutex::new(Vec::new())),
            prompts: sources::LOCAL_PROMPTS.iter().map(|s| String::from(*s)).collect(),
            babble:  sources::LOCAL_BABBLE.iter().map(|s| String::from(*s)).collect()
        };

        let _ = fs::create_dir_all("./goonto-cache");

        thread::spawn({
            let clone = Arc::clone(&r.images);
            move || { let _ = stocktake(tags, clone); }
        });

        Ok(r)
    }
}

impl sources::Source for E621 {
    fn prompt(&self) -> String {
        random_from(&self.prompts)
    }

    fn image(&self) -> String {
        take_random(&mut self.images.lock().unwrap())
    }
    
    fn babble(&self) -> String {
        random_from(&self.babble)
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

        let mut url = "https://e621.net/posts.json?limit=25&tags=-animated rating:e order:random score:>300 "
            .to_string();
        url.push_str(&random_from(&tags));

        let client = reqwest::Client::new();

        let resp = client
            .get(url)
            .header(reqwest::header::USER_AGENT, "Goonto/1.0.69")
            .send()
            .await
            .unwrap()
            .json::<E6Posts>()
            .await
            .unwrap()
            .posts
            .into_iter()
            .filter(|p| p.sample.url.is_some())
            .map(   |p| p.sample.url.unwrap())
            .collect::<Vec<String>>();

        let imagez = &images;
        let new_images = stream::iter(resp)
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

fn random_from<T: std::default::Default + Clone>(x: &Vec<T>) -> T {
    if x.is_empty() {
        return T::default()
    }

    x.choose(&mut rand::thread_rng()).unwrap().clone()
}

fn take_random<T: std::default::Default>(x: &mut Vec<T>) -> T {
    if x.is_empty() {
        return T::default()
    }

    let i = (0..x.len()).choose(&mut rand::thread_rng()).unwrap();
    x.swap_remove(i)
}