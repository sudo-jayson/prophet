use rand::seq::{IteratorRandom, SliceRandom};
use std::fs;
use std::io::{self, BufRead, BufReader, stdout, Write};
use rodio::{Decoder, OutputStream, source::Source};

fn main() -> io::Result<()> {

    let script_path = "/home/jayson/draculations/src/script.txt";
    let _sfx = set_sfx();
    let mut rng = rand::thread_rng();

    let _script = fs::File::open(&script_path)?;
    let reader = io::BufReader::new(_script);
    
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let Some(bar) = lines.choose(&mut rng) else {
        todo!()
    };

    println!("{}", '\n');

    for c in bar.chars(){
        print!("{}", c);
        let _ = stdout().flush();
        let _ = play_sfx(_sfx.clone());
    }

    println!("{}", '\n');

    Ok(())
}

fn set_sfx() -> String{
    let mut rng = rand::thread_rng();
    let sounds = fs::read_dir("/home/jayson/draculations/sfx/").unwrap();
    let sfx_path = sounds.choose(&mut rng).unwrap().unwrap();
    return sfx_path.path().display().to_string();
}

fn play_sfx(_sfx: String) -> io::Result<()>{
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sfx_file = BufReader::new(fs::File::open(_sfx).unwrap());
    let source = Decoder::new(sfx_file).unwrap();
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_millis(100));

    Ok(())
}
