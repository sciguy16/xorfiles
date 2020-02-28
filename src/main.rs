use structopt::StructOpt;
use std::io::stdin;
//use std::io::Stdin;
//use std::io::BufReader;
//use std::io::BufRead;
use std::io;
use std::fs::File;
use std::path::PathBuf;
use std::io::prelude::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "xorfiles", about = "XOR two files together")]
struct Opts {
    /// First file
    #[structopt(name = "FILE1")]
    #[structopt(parse(from_os_str))]
    file1: PathBuf,

    /// Second file
    #[structopt(name = "FILE2")]
    #[structopt(parse(from_os_str))]
    file2: Option<PathBuf>,

    /// Optionally loop the shorter file around until the longer file
    /// runs out
    // "loop" is a reserved keyword so we can't name the variable that :(
    #[structopt(short = "l", long = "loop")]
    loop_shorter: bool,
}

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let opts = Opts::from_args();
    println!("{:?}", opts);

    let f1 = File::open(opts.file1)?;
    let mut buf1 = f1.bytes();

    let mut buf2 = if let Some(file2) = opts.file2 {
        Some(File::open(file2)?.bytes())
    } else {
        None
    };

    let mut buf2 = buf2.unwrap();

    let mut writer = io::stdout();

    loop {
        let b1 = buf1.next();
        let b2 = buf2.next();
        match (b1, b2) {
            (None, None) => {
                eprintln!("The end :(");
                break;
            },
            (Some(Ok(a)), Some(Ok(b))) => {
                writer.write(&[a ^ b]);
            },
            _ => break,
        }

    }

    Ok(())
}
