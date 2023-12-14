use std::rc::Rc;
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use log::info;
use discord_rich_presence::{
    activity::{Activity, Button, Assets, Timestamps},
    DiscordIpc,
    DiscordIpcClient
};
use defaults::Defaults;

const DEFAULT_DESC: &str = "Stroking their brains out~";

#[derive(Serialize, Deserialize, Defaults)]
#[serde(default)]
pub struct Discord {
    pub enabled: bool,
    #[def = "true"]
    shill: bool,
    #[def = "String::from(DEFAULT_DESC)"]
    description: String,
}

impl Discord {
    // this library uses Box<dyn Error> in Result, which is stupid. Maybe I should change library.
    // Oh well, it works.
    pub fn run<T: crate::sources::Source + 'static + ?Sized>(self, _source: Rc<T> ) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        std::thread::spawn(move || {
            let mut client = match DiscordIpcClient::new("1144372997390614689") {
                Ok(v) => v,
                Err(e) => {
                    info!("couldn't init discord ipc client: {:?}", e);
                    return;
                },
            };
        
            info!("[Discord] connect: {:?}", client.connect());
        
            let mut activity = Activity::new()
                .details(if self.description.is_empty() { DEFAULT_DESC } else { &self.description })
                .timestamps(Timestamps::new().start(now))
                .assets(Assets::new().large_image("icon"));

            if self.shill {
                activity = activity.buttons(vec![
                    Button::new("Join them~", "https://zoomasochist.github.io")]
                );
            }

            info!("[Discord] set_activity: {:?}", client.set_activity(activity));
            
            loop {
                std::thread::sleep(Duration::from_secs(100));
            }
        });
    }
}