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
            move || loop {
                if let Err(e) = stocktake(&source, &clone) {
                    crate::dialog(&format!("Stocktaking failed: {:?}\nTrying again.", e));
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
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
        crate::sources::random_from(&self.images.lock().unwrap())
    }
}

// TODO: make safer etc
fn stocktake(source: &str, images: &Arc<Mutex<Vec<String>>>) -> anyhow::Result<()> {
    std::fs::read_dir(source)?
        .map(|path| {
            let path = path?;
            
            if is_file(&path)? && allowed_extension(&path) {
                let path_str = path.path().to_string_lossy().to_string();

                images.lock().unwrap().push(path_str);
            }

            anyhow::Ok(())
        })
        .for_each(|res| {
            if let Err(res) = res {
                log::warn!("Stocktake error: {:?}", res);
            }
        });
    
    Ok(())
}

fn allowed_extension(entry: &DirEntry) -> bool {
    let allowed = ["jpg", "jpeg", "png", "bmp", "gif"];

    entry
        .path()
        .extension()
        .is_some_and(|e| allowed.contains(&e
                .to_string_lossy()
                .to_string()
                .to_lowercase()
                .as_str()))
}

fn is_file(entry: &DirEntry) -> anyhow::Result<bool> {
    Ok(entry.file_type()?.is_file())
}