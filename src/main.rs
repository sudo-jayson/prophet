pub mod prophet;

use rand::seq::{IteratorRandom, SliceRandom};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs;
use std::io::{self, stdout, BufRead, BufReader, Write};

const SCRIPT_PATH: &str = "/home/jayson/prophet/config/script.txt";
const SFX_PATH: &str = "/home/jayson/prophet/sfx/undertale";


fn main() -> io::Result<()> {
    let _sfx = set_sfx();
    let mut rng = rand::thread_rng();

    let _script = fs::File::open(SCRIPT_PATH)?;
    let reader = io::BufReader::new(_script);

    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let Some(bar) = lines.choose(&mut rng) else {
        todo!()
    };

    println!("{}", '\n');

    for c in bar.chars() {
        print!("{}", c);
        let _ = stdout().flush();
        let _ = play_sfx(_sfx.clone());
    }

    println!("{}", '\n');

    Ok(())
}

fn set_sfx() -> String {
    let mut rng = rand::thread_rng();
    let sounds = fs::read_dir(SFX_PATH).unwrap();
    let sfx_path = sounds.choose(&mut rng).unwrap().unwrap();
    return sfx_path.path().display().to_string();
}

fn play_sfx(_sfx: String) -> io::Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sfx_file = BufReader::new(fs::File::open(_sfx).unwrap());
    let source = Decoder::new(sfx_file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_millis(63));

    Ok(())
}
