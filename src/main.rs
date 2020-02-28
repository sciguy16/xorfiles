use structopt::StructOpt;
//use std::io::stdin;
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

    // /// Optionally loop the shorter file around until the longer file
    // /// runs out
    // "loop" is a reserved keyword so we can't name the variable that :(
    //#[structopt(short = "l", long = "loop")]
    //loop_shorter: bool,

    // /// Optional limit on the number of bytes to read
    //#[structopt(short = "c", long = "count", default_value = "0")]
    //count: usize,
}

fn main() -> io::Result<()> {
    let opts = Opts::from_args();
    //if opts.loop_shorter {
    //    unimplemented!();
    //}

    let f1 = File::open(opts.file1)?;
    //let f1meta = f1.metadata()?;
    let mut buf1 = f1.bytes();

    let f2;
    let buf2 = if let Some(file2) = opts.file2 {
        f2 = File::open(file2)?;
        Some(f2.bytes())
    } else {
        None
    };

    let mut buf2 = buf2.unwrap();

    loop {
        let b1 = buf1.next();
        let b2 = buf2.next();
        //eprintln!("Loop: ({:?}, {:?})", b1, b2);
        match (b1, b2) {
            (Some(Ok(a)), Some(Ok(b))) => {
                io::stdout().write(&[a ^ b])?;
                //eprint!("=");
            },
            (None, None) => {
                //eprintln!("The end :(");
                break;
            },
            _x => {
                //eprintln!("End: {:?}", x);
                break;
            },
        }

    }

    io::stdout().flush()?;

    Ok(())
}
