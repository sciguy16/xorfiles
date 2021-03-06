/*
 * A command-line utility to XOR two files together. Useful for CTFs and
 * the like.
 * Copyright (C) 2019 David Young
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 *
 */

use structopt::StructOpt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
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
    let buf1 = BufReader::new(f1).bytes();

    if let Some(file2) = opts.file2 {
        // If we are in two-file mode then XOR the two files
        let f2 = File::open(file2)?;
        let buf2 = BufReader::new(f2).bytes();

        for pair in buf1.zip(buf2).map(|p| (p.0.unwrap(), p.1.unwrap())) {
            io::stdout().write(&[pair.0 ^ pair.1])?;
        }
    } else {
        // File-and-stdin mode, so XOR the file with stdin
        for pair in BufReader::new(io::stdin())
            .bytes()
            .zip(buf1)
            .map(|p| (p.0.unwrap(), p.1.unwrap()))
        {
            io::stdout().write(&[pair.0 ^ pair.1])?;
        }
    }

    io::stdout().flush()?;

    Ok(())
}
