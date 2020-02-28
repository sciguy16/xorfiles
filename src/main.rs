use structopt::StructOpt;
use std::path::PathBuf;

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

fn main() {
    println!("Hello, world!");

    let opt = Opts::from_args();
    println!("{:?}", opt);
}
