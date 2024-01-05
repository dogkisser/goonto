use std::fs::{self, File};
use std::io;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::Path;
use serde::Deserialize;
use futures::stream::{self, StreamExt};

use crate::sources;
use crate::config::ImageRes;

#[derive(Debug)]
pub struct E621 {
    images: Arc<Mutex<Vec<String>>>,
    first_person: Vec<String>,
    third_person: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct E6Posts {
    posts: Vec<E6Post>,
}

#[derive(Debug, Deserialize)]
struct E6Post {
    file: E6File,
    sample: E6File,
}

#[derive(Debug, Deserialize)]
struct E6File {
    url: Option<String>,
}

impl E621 {
    pub fn new(cfg: &crate::Config) -> anyhow::Result<Self> {
        let (first, third) = crate::sources::get_babble(cfg);
        let r = Self {
            images: Arc::new(Mutex::new(Vec::new())),
            first_person: first,
            third_person: third,
        };

        let _ = fs::create_dir_all("./goonto-cache");

        let tags = cfg.image_source.web.tags.clone();
        let full_res = cfg.image_source.web.image_res == ImageRes::Full;
        let prefix = cfg.image_source.web.tag_prefix.clone();
        thread::spawn({
            let clone = Arc::clone(&r.images);
            move || loop {
                if let Err(e) = stocktake(&tags, &clone, full_res, &prefix) {
                    crate::dialog(&format!("Stocktaking failed: {:?}\nTrying again.", e));
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        });

        Ok(r)
    }
}

impl sources::Source for E621 {
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

fn stocktake(tags: &[String], images: &Arc<Mutex<Vec<String>>>, full_res: bool, prefix: &str)
    -> anyhow::Result<()>
{
    tokio::runtime::Runtime::new()?.block_on(async { loop {
        if images.lock().unwrap().len() > 25 {
            thread::sleep(Duration::from_secs(1));
            continue;
        }

        let url = format!("https://e621.net/posts.json?limit=25&tags=-animated order:random {} {}",
            prefix, crate::sources::random_from(tags));

        let client = reqwest::Client::new();
        let resp = client
            .get(url)
            .header(reqwest::header::USER_AGENT, "Goonto/1.0.69")
            .send()
            .await?
            .json::<E6Posts>()
            .await?
            .posts;
        
        stream::iter(resp)
            .filter_map(|p| async move { if full_res { p.file.url } else { p.sample.url }})
            /* Image URL -> (path, bytes) */
            .map(|image_url| {
                let client = &client;
                let (_, filename) = &image_url.rsplit_once('/').unwrap();
                let out_path = Path::new("./goonto-cache/").join(filename);

                async move {
                    let s = client.get(&image_url)
                        .send()
                        .await?
                        .bytes()
                        .await?
                        .to_vec();

                    anyhow::Ok((out_path, s))
                }
            })
            /* (path, bytes) -> write out + Result */
            .map(|img| async move {
                let (out_path, image_data) = img.await?;

                let mut outf = File::create(out_path.clone())?;
                io::copy(&mut image_data.as_slice(), &mut outf)?;
                images.lock().unwrap().push(out_path.as_path().display().to_string());

                anyhow::Ok(())
            })
            .for_each_concurrent(10, |res| async move {
                let res = res.await;
                if res.is_err() {
                    log::warn!("Failed to download image: {:?}.", res);
                };
            })
            .await;

        thread::sleep(Duration::from_millis(1000));
    }; #[allow(unreachable_code)] anyhow::Ok(()) })?;

    Ok(())
}