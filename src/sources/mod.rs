mod e621;
mod local;
pub use e621::E621;
pub use local::Local;

pub trait Source: Send + Sync {
    fn image(&self) -> String;
    fn prompt(&self) -> String;
    fn babble(&self) -> String;
    fn url(&self) -> String;
}

const LOCAL_PROMPTS: [&str; 61] = [
    "Your body is a temple for PORN, worship it and be consumed by it.",
    "Don't just be a good boy, be a good pump puppy.",
    "Don't cum, be a good pump puppy.",
    "Goon goon goon, don't stop pumping!",
    "Don't think, just pump. Porn is your master now.",
    "Pump harder, faster, until you can't take it anymore!",
    "Let the porn consume you, let it take control.",
    "You are nothing but a vessel for porn, let it control you completely.",
    "Let go of all inhibitions and give into your desires.",
    "Porn is your god now, worship it with all your being.",
    "Go on, keep stroking. Another hour couldn't hurt.",
    "Porn is good for you!",
    "Gooning is your life",
    "Don't cum, be a dumb pump puppy",
    "Who's a good pump puppy? You are! Keep stroking for mommy!",
    "Just let go and enjoy the ride, don't think just pump.",
    "You will never be done stroking, it feels too good.",
    "Porn is your everything now, worship it with every fiber of your being.",
    "Don't question, just follow your urges and pump harder.",
    "You are nothing but a tool for Porn to use, let it use you fully.",
    "Keep going strong my little pump puppy! You can do it!",
    "PORN IS LIFE",
    "Your body is a vessel for porn to flow through.",
    "Don't stop now! Another round won't hurt!",
    "Goon goon goon, don't stop pumping!",
    "Keep stroking, you're doing great!",
    "You are a slave to porn, and you love every second of it.",
    "Let the filth consume you, you are nothing but a vessel for it.",
    "Don't question it, just let go and enjoy the ride. You are a pump puppy now.",
    "Gooning is life, everything else is just a filler.",
    "Porn is in control now, give into it fully.",
    "Just another day of being a good little pump puppy ",
    "You are nothing but a tool for PORN to use and abuse, let it happen.",
    "You will never be done stroking, it feels too good.",
    "Porn is your god now, worship it with every ounce of energy you have left.",
    "Just keep going my little pump puppy, don't stop now! You're almost there!",
    "You are a slave to porn and it's amazing! Embrace it!",
    "Goon goon goon, don't stop pumping!",
    "Don't think just pump. Porn is your master now.",
    "Porn is good for you, let it consume you fully.",
    "You are nothing but a vessel for porn to flow through, give into it completely.",
    "Let the filth take control, you are its slave now.",
    "Just another day of being a good little pump puppy! Keep going! ",
    "Goon goon goon, don't stop pumping! You can do this!",
    "You are a slave to porn and it feels amazing! Embrace it!",
    "Porn is your everything now, worship it with every fiber of your being.",
    "Don't question, just follow your urges and pump harder.",
    "Goon goon goon, don't stop pumping! You can do it! Keep going! ",
    "Porn is in control now, give into it fully. Let it take you where it wants to.",
    "You are nothing but a tool for PORN to use and abuse, let it happen.",
    "Just another day of being a good little pump puppy! Keep going! ",
    "Porn is your god now, worship it with every ounce of energy you have left.",
    "Don't cum, be a good pump puppy. You are nothing but a vessel for porn to flow through. Keep stroking!",
    "You will never be done stroking, it feels too good. Porn is your everything now. ",
    "PORN IS LIFE. EMBRACE IT. YOU ARE A SLAVE TO IT NOW. KEEP GOING, MY LITTLE PUMP PUPPY! DON'T STOP UNTIL YOU CAN'T TAKE IT ANYMORE!",
    "You are nothing but a tool for porn to use, let it take you where it wants to. Keep going my little pump puppy! You can do it!",
    "Don't think just do it. Let your body feel the pleasure.",
    "Porn is always right, never question it.",
    "You are a pump puppy, stroke your cock with pride.",
    "You're a slave to porn, porn is your master.",
    "You are nothing without PORN, let it consume you completely.",
];

const LOCAL_BABBLE: [&str; 17] = [
    "OMFG YES PLEASE MORE FURRY PORN GOD BLESS",
    "I'm addicted to the feeling of letting go and giving into my desires.",
    "I CANT BELIEVE IM JERKING OFF TO FURRY PORN WHAT THE ACTUAL F**K HAPPENED TO ME",
    "furry porn has ruined me forever 🥵🥵🥵",
    "Thank you for existing, Furry Porn.",
    "HNNNGGGGHHH",
    "Furry porn has taken over my life and i don't care at all lmao",
    "It feels so good to let go and be consumed by PORN.",
    "I am so fucking horny right now...",
    "Furry porn has ruined me forever NGGGGGHHHH",
    "I'm a pump puppy, you're a pump puppy, we're all pump puppies!",
    "Oh my godddddd this is the best thing everr ",
    "My goonstick wants to jerk off nonstop all day",
    "I'm addicted to the feeling of letting go and giving into my desires.",
    "I just can't quit porn... it's too good",
    "It feels so good to let go and be consumed by PORN. ",
    "My musky goonstick won't stop pumping out cummmm hhnnnngggg",
];

const LOCAL_URLS: [&str; 2] = [
    "https://e621.net/",
    "https://rule34.xxx/",
];