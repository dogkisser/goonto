use std::thread;
use std::fs::DirEntry;
use std::sync::{Arc, Mutex};

use crate::sources;

pub struct Local {
    images: Arc<Mutex<Vec<String>>>,
    first_person: Vec<String>,
    third_person: Vec<String>,
}

impl Local {
    pub fn new(cfg: &crate::Config) -> anyhow::Result<Self> {
        // This architecture is a little weird but it echoes the E6 impl
        // With big porn folders it may take a prohibitively long time to index everything
        // So relegate it to a thread.. I guess? :)
        let (first, third) = crate::sources::get_babble(cfg);
        let r = Self {
            images: Arc::new(Mutex::new(Vec::new())),
            first_person: first,
            third_person: third,
        };

        let source = cfg.image_source.local.clone();
        thread::spawn({
            let clone = Arc::clone(&r.images);
            move || { let _ = stocktake(source, clone); }
        });

        Ok(r)
    }
}

impl sources::Source for Local {
    fn first_person(&self) -> String {
        crate::sources::random_from(&self.first_person)
    }

    fn third_person(&self) -> String {
        crate::sources::random_from(&self.third_person)
    }

    fn image(&self) -> String {
        crate::sources::random_from(&mut self.images.lock().unwrap())
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