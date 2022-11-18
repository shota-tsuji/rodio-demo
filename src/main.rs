use std::env;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    //let file = std::fs::File::open("Sample_BeeMoved_96kHz24bit.flac").unwrap();
    let file = std::fs::File::open(&args[1]).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}
