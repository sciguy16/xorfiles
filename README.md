# xorfiles
A command-line utility to XOR two files. Useful for certain types of
CTF challenge.

## Usage
`./xorfiles FILE1 FILE2`

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
* Benchmark against Ruby/C/Python implementations
