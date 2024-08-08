use rand::seq::{IteratorRandom, SliceRandom};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs;
use std::io::{self, stdout, BufRead, BufReader, Write};


struct Prophet{
    script: String,
    sfx: String,
    delay: u64,
}

impl Prophet{
    fn init(lines: Vec<String>) -> Self{
        Prophet{
            script: lines.get(0).cloned().unwrap_or_default(),
            sfx: lines.get(1).cloned().unwrap_or_default(),
            delay: lines.get(2).cloned().unwrap_or_default().parse().expect("fuck"),
        }
    }
}

fn main() -> io::Result<()> {

    let conf_file = "dracula.conf";

    let config_dir = "/home/jayson/prophet/config/";
    let sfx_dir = "/home/jayson/prophet/sfx/";

    let config = config_dir.to_string() + conf_file;

    let f = fs::File::open(config)?;
    let r = io::BufReader::new(f);

    let conf_lines: Vec<String> = r.lines().filter_map(Result::ok).collect();

    let mut prophet = Prophet::init(conf_lines);

    prophet.script = config_dir.to_owned() + &prophet.script;
    prophet.sfx = sfx_dir.to_owned() + &prophet.sfx;

    let _sfx = set_sfx(prophet.sfx);
    let mut rng = rand::thread_rng();

    let full_script = fs::File::open(prophet.script)?;
    let reader = io::BufReader::new(full_script);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let Some(bar) = lines.choose(&mut rng) else {
        todo!()
    };


    println!("{}", '\n');

    for c in bar.chars() {
        print!("{}", c);
        let _ = stdout().flush();
        let _ = play_sfx(_sfx.clone(), prophet.delay);
    }

    println!("{}", '\n');

    Ok(())
}

fn set_sfx(s: String) -> String {
    let mut rng = rand::thread_rng();
    let sounds = fs::read_dir(s).unwrap();
    let sfx_path = sounds.choose(&mut rng).unwrap().unwrap();
    return sfx_path.path().display().to_string();
}

fn play_sfx(_sfx: String, delay: u64) -> io::Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sfx_file = BufReader::new(fs::File::open(_sfx).unwrap());
    let source = Decoder::new(sfx_file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_millis(delay));

    Ok(())
}
