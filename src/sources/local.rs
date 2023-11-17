use std::thread;
use std::fs::DirEntry;
use std::sync::{Arc, Mutex};
use rand::prelude::SliceRandom;

use crate::sources;

pub struct Local {
    images: Arc<Mutex<Vec<String>>>,
    prompts: Vec<String>,
    babble: Vec<String>,
    urls: Vec<String>,        
}

impl Local {
    pub fn new(source: String) -> anyhow::Result<Self> {
        // This architecture is a little weird but it echoes the E6 impl
        // With big porn folders it may take a prohibitively long time to index everything
        // So relegate it to a thread.. I guess? :)
        let r = Self {
            images: Arc::new(Mutex::new(Vec::new())),
            prompts: sources::LOCAL_PROMPTS.iter().map(|s| String::from(*s)).collect(),
            babble:  sources::LOCAL_BABBLE.iter().map(|s| String::from(*s)).collect(),
            urls: sources::LOCAL_URLS.iter().map(|s| String::from(*s)).collect(),
        };

        thread::spawn({
            let clone = Arc::clone(&r.images);
            move || { let _ = stocktake(source, clone); }
        });

        Ok(r)
    }
}

impl sources::Source for Local {
    fn prompt(&self) -> String {
        random_from(&self.prompts)
    }

    fn image(&self) -> String {
        random_from(&mut self.images.lock().unwrap())
    }
    
    fn babble(&self) -> String {
        random_from(&self.babble)
    }

    fn url(&self) -> String {
        random_from(&self.urls)
    }
}

// TODO: make safer etc
fn stocktake(source: String, images: Arc<Mutex<Vec<String>>>) {
    std::fs::read_dir(source)
        .unwrap()
        .for_each(|path| {
            let path = path.unwrap();
            
            if is_file(&path) && allowed_extension(&path) {
                let path_str = path.path().to_string_lossy().to_string();

                images.lock().unwrap().push(path_str);
            }
        });
}

fn random_from<T: std::default::Default + Clone>(x: &Vec<T>) -> T {
    if x.is_empty() {
        return T::default()
    }

    x.choose(&mut rand::thread_rng()).unwrap().clone()
}

fn allowed_extension(entry: &DirEntry) -> bool {
    let allowed = ["jpg", "jpeg", "png", "bmp", "gif"];

    entry
        .path()
        .extension()
        .is_some_and(|e| allowed.contains(&e.to_string_lossy().to_string().as_str()))
}

fn is_file(entry: &DirEntry) -> bool {
    entry.file_type().unwrap().is_file()
}