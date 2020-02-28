# xorfiles
A command-line utility to XOR two files. Useful for certain types of
CTF challenge.

## Usage
`./xorfiles FILE1 FILE2`

## Features
* XOR two files: `xorfiles FILE1 FILE2`
* XOR one file with stdin: `xorfiles FILE1 < FILE2`
* High throughput compared to Python and Ruby implementations

## Installation
```
git clone https://github.com/sciguy16/xorfiles
cargo build --release
cargo run --release -- -h
```

## Roadmap/Todo
* XOR file with stdin
* Optionally loop the shorter file (like a multi-byte XOR)
* Limit the number of bytes read/written
* Output to file
* Hexdump on the input and output stages
* Benchmark against C implementations
* Optimise the Ruby and Python benchmarks further for better competition

## Benchmarks
Benchmarks run with a 1 GB file XORed with itself, generated with
`dd if=/dev/zero of=big_file bs=1M count=1000`. The Rust version was
tested in two operating modes: XOR two files together and one file with
 stdin.

The following command was used to measure the throughput:
```
<implementation> big_file big_file | pv > /dev/null
```

| Implementation | Approx. throughput |
|:--------------:|:------------------:|
| Rust (files)   | 6.5 MiB/s          |
| Rust (stdin)   | 6.4 MiB/s          |
| Ruby           | 2.45 MiB/s         |
| Python         | 4.7 MiB/s          |
