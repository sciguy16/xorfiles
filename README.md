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
* Benchmark against C/Python implementations

## Benchmarks
Benchmarks run with a 1 GB file XORed with itself, generated with
`dd if=/dev/zero of=big_file bs=1M count=1000`.

The following command was used to measure the throughput:
```
<implementation> big_file big_file | pv > /dev/null
```

| Implementation | Approx. throughput |
|:--------------:|:------------------:|
| Rust           | 6.5 MiB/s          |
| Ruby           | 2.45 MiB/s          |
| Python         | 4.7 MiB/s          |
