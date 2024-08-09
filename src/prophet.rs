use rand::seq::IteratorRandom;
use std::fs::read_dir;

pub struct Prophet {
    pub script: String,
    pub sfx: String,
    pub delay: u64,
}
impl Prophet {
    pub fn init(lines: Vec<String>) -> Prophet {
        Prophet {
            script: lines.get(0).cloned().unwrap_or_default(),
            sfx: lines.get(1).cloned().unwrap_or_default(),
            delay: lines
                .get(2)
                .cloned()
                .unwrap_or_default()
                .parse()
                .expect("fuck"),
        }
    }

    pub fn set_sfx(s: String) -> String {
        let mut rng = rand::thread_rng();
        let sounds = read_dir(s).unwrap();
        let sfx_path = sounds.choose(&mut rng).unwrap().unwrap();
        return sfx_path.path().display().to_string();
    }
}
