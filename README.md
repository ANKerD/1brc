# 1brc

# Billion Row Challenge

This is my contribution to the one billion row challenge. The goal? Process 1 billion row (1brc) in the least amount of time. Taking this challenge to learn and hone my Rust skills.

You can find the rules and boundaries here [https://www.morling.dev/blog/one-billion-row-challenge/](https://www.morling.dev/blog/one-billion-row-challenge/)

## 0. Measures generator

The original project had it's java generator. But since that's a learning project, why not port it to Rust.

Update: It took me abot 4 hours to port.

Update 2: Plus some 2 extra hours to make it work like the original script including 10k+ stations for bigger inputs

### How to generate the data

```

mkdir data
cargo run --bin create_measurements 1000 data/1k.txt
cargo run --bin create_measurements 10000 data/10k.txt
cargo run --bin create_measurements 100000 data/100k.txt
cargo run --bin create_measurements 1000000 data/1m.txt
cargo run --bin create_measurements 10000000 data/10m.txt
cargo run --bin create_measurements 100000000 data/100m.txt
cargo run --bin create_measurements 1000000000 data/1b.txt
```

## 1. First Naive implementation

Simply read line-by-line iteratively and use a map to keep track of each station individually. No concurrency and no clever tricks, just set the base line. 

### How to run

```
./src/bin/1_naive_implementation/run-all.sh 2>/dev/null
```

## 2. Optmizing the build

Running cargo with `--release` falg has dramatically cut the running time of the script. For 10M rows the time went from 50 seconds down to 8.7s.

##### Warning

The results may vary as the benchmarks were ran while tens of chrome tabs were open along with vs code and slack running as an app.

| Input size | Naive solution | Release Build | + codegen-units = 1 |
|------------|----------------|---------------|---------------------|
| 1k         | 0.25s          | 0.66s         | 0.82s               |
| 10k        | 0.26s          | 0.39s         | 0.33s               |
| 100k       | 0.51s          | 0.28s         | 0.36s               |
| 1m         | 4.51s          | 1.07s         | 0.79s               |
| 10m        | 44.23s         | 8.57s         | 5.24s               |
| 100m       | 262.96s        | 85.40s        | 49.88s              |
| 1b         | -              | 747.49s       | 474.06s             |

Some other build options will be added to increase the binary speed without touching the code.

1. Set codegen-units. This will slow down compilation but allows the compiler to better optimize.
2. Set LNO to `thin`, this decreases binary size and promises 10~20% better runtime speeds.
3. Set LNO to `fat`.

| Input size | Naive solution | Release Build | codegen-units=1 | lto = thin | lto = fat |
|------------|----------------|---------------|-----------------|------------|-----------|
| 1k         | 0.25s          | 0.66s         | 0.74s           | 0.29s      | 0.49s     |
| 10k        | 0.26s          | 0.39s         | 0.29s           | 0.23s      | 0.21s     |
| 100k       | 0.51s          | 0.28s         | 0.28s           | 0.26s      | 0.24s     |
| 1m         | 4.51s          | 1.07s         | 0.79s           | 0.76s      | 0.73s     |
| 10m        | 44.23s         | 8.57s         | 5.41s           | 5.34s      | 5.34s     |
| 100m       | 262.96s        | 85.40s        | 51.98s          | 53.16s     | 52.01s    |
| 1b         | 4306.22        | 747.49s       | 474.06s         | -          | -         |

