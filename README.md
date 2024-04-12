# 1brc

# Billion Row Challenge

This is my contribution to the one billion row challenge. The goal? Process 1 billion row (1brc) in the least amount of time. Taking this challenge to learn and hone my Rust skills.

You can find the rules and boundaries here [https://www.morling.dev/blog/one-billion-row-challenge/](https://www.morling.dev/blog/one-billion-row-challenge/)

## 0. Measures generator

The original project had it's java generator. But since that's a learning project, why not port it to Rust.

Update: It took me abot 4 hours to port.

Update 2: Plus some 2 extra hours to make it work like the original script including 10k+ stations for bigger inputs

#### How to generate the data

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
cargo run --bin 1_naive_implementation data/1b.txt
```
