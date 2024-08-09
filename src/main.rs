#![allow(dead_code)]

mod prophet;

use rand::seq::SliceRandom;
use rodio::{source::Source, Decoder, OutputStream};
use std::env::args;
use std::fs;
use std::io::{self, stdout, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = args().collect();

    let config_dir = "/home/jayson/prophet/config/";
    let sfx_dir = "/home/jayson/prophet/sfx/";

    let config = config_dir.to_string() + &args[1] + ".conf";

    let fil = fs::File::open(config)?;
    let read = io::BufReader::new(fil);

    let conf_lines: Vec<String> = read.lines().filter_map(Result::ok).collect();

    let mut p = prophet::Prophet::init(conf_lines);

    p.script = config_dir.to_owned() + &p.script + ".txt";
    p.sfx = sfx_dir.to_owned() + &p.sfx;

    let _sfx = prophet::Prophet::set_sfx(p.sfx);
    let mut rng = rand::thread_rng();

    let full_script = fs::File::open(p.script);

    let full_script = match full_script {
        Ok(file) => file,
        Err(error) => {
            panic!("File not found stoopid {error:?}");
        }
    };

    let reader = io::BufReader::new(full_script);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let Some(bar) = lines.choose(&mut rng) else {
        panic!("OH FUCK WHAT THE HELL DID YOU DO");
    };

    println!("{}", '\n');

    for c in bar.chars() {
        print!("{}", c);
        let _ = stdout().flush();
        let _ = play_sfx(_sfx.clone(), p.delay);
    }

    println!("{}", '\n');

    Ok(())
}

fn play_sfx(_sfx: String, delay: u64) -> io::Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sfx_file = BufReader::new(fs::File::open(_sfx).unwrap());
    let source = Decoder::new(sfx_file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_millis(delay));

    Ok(())
}
