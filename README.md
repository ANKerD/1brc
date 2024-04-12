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

### How to run

```
./src/bin/2_optimize_build/run-all.sh 2>/dev/null
```

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

## 3. Tweak heap allocators + cpu specific compilations

Now testing different heap allocators as suggestted here [https://nnethercote.github.io/perf-book/build-configuration.html](https://nnethercote.github.io/perf-book/build-configuration.html)

### How to run

```
./src/bin/3_heap_allocator_flags_pgc/run-all.sh 2>/dev/null
```

First experiment is using `tikv-jemallocator`. A potential performance gain can come from enabling THP (Transparent Huge Pages), but as I am using Mac this is not possible.

Second we tweak the program to use mimalloc as the allocator. As their behavior and performance might change accordingly to the shape of the program, I'm considering changing the `run-all.sh` script to test both in every workload run;

| Input size | codegen-units=1+lto fat | tikv-jemallocator | mimalloc |
|------------|-------------------------|-------------------|----------|
| 1k         | 0.49s                   | 0.27s             | 0.32s    |
| 10k        | 0.21s                   | 0.26s             | 0.21s    |
| 100k       | 0.24s                   | 0.23s             | 0.33s    |
| 1m         | 0.73s                   | 0.64s             | 0.65s    |
| 10m        | 5.34s                   | 4.19s             | 4.14s    |
| 100m       | 51.01s                  | 39.51s            | 40.16s   |
| 1b         | -                       | 405.17s           | 410.24s  |

Both allocators share a similar performance so I'll test each one denpending on my code implementation. There's the possibility to use platform specific compilation flags but running this `diff <(rustc --print cfg) <(rustc --print cfg -C target-cpu=native)` showed no difference.

PGC is an advanced technique where we run the code in instrumented mode and create profiles that later can be used as input to ther compiler. I'll use this technique now, but I don't like the idea of implementing it into my pipeline right now at least not before I start getting my hands dirty with the code. More details here [https://doc.rust-lang.org/rustc/profile-guided-optimization.html](https://doc.rust-lang.org/rustc/profile-guided-optimization.html)
```
rm -rf /tmp/pgo-data
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release --target=aarch64-apple-darwin
./target/aarch64-apple-darwin/release/3_heap_allocator_flags_pgc ./data/1m.txt
./target/aarch64-apple-darwin/release/3_heap_allocator_flags_pgc ./data/10m.txt
./target/aarch64-apple-darwin/release/3_heap_allocator_flags_pgc ./data/100m.txt
~/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release --target=aarch64-apple-darwin
time ./target/aarch64-apple-darwin/release/3_heap_allocator_flags_pgc ./data/1b.txt
```

| Input size | tikv-jemallocator | +PGC     | mimalloc | +PGC     | mimalloc+flags |
|------------|-------------------|----------|----------|----------| ---------------|
| 1m         | 0.64s             | 0.412s   | 0.65s    | 0.404s   | 0.48s          |
| 10m        | 4.19s             | 3.8s     | 4.14s    | 3.789s   | 4.0s           |
| 100m       | 39.51s            | 37.167s  | 40.16s   | 37.17    | 39s            |
| 1b         | 405.17s           | 390.291s | 410.24s  | 380s     | 390s           |

For the record, I noticed some performance loss when removing the power cable and running just on battery so from now on every benchmark will be run with the cable connected.

Keeping all the above compile-time optimizations except for PGC and on top of that taking the following steps allowed me to cut runtime significantly.

1. replacing the data structure from BTreeMap to HashMap
2. use itertools to sort the keys.
3. Better handle strings and map keys

```rust
let mut map: HashMap<String, (f32, f32, f32, i32)> = HashMap::new();
for line in reader.lines() {
    let liner = line.unwrap();
    let lines: Vec<&str> = liner.split(";").collect();
    let v = lines[1].parse::<f32>().unwrap(); 
    
    let station = map.entry(lines[0].to_string()).or_insert( (f32::MAX, f32::MIN, 0.0, 0));
    [...]
}
```


| Input size | mimalloc+flags | +HashMap |
|------------|----------------|----------|
| 1m         | 0.48s          | 0.21s    |
| 10m        | 4.0s           | 1.26s    |
| 100m       | 39s            | 10.21s   |
| 1b         | 390s           | 103.62s  |