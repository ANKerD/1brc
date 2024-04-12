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
2. Set LTO to `thin`, this decreases binary size and promises 10~20% better runtime speeds.
3. Set LTO to `fat`.

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
./src/bin/3_heap_allocator_flags_pgo/run-all.sh 2>/dev/null
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

PGO is an advanced technique where we run the code in instrumented mode and create profiles that later can be used as input to ther compiler. I'll use this technique now, but I don't like the idea of implementing it into my pipeline right now at least not before I start getting my hands dirty with the code. More details here [https://doc.rust-lang.org/rustc/profile-guided-optimization.html](https://doc.rust-lang.org/rustc/profile-guided-optimization.html)
```
rm -rf /tmp/pgo-data
RUSTFLAGS="-Cprofile-generate=/tmp/pgo-data" cargo build --release --target=aarch64-apple-darwin
./target/aarch64-apple-darwin/release/3_heap_allocator_flags_pgo ./data/1m.txt
./target/aarch64-apple-darwin/release/3_heap_allocator_flags_pgo ./data/10m.txt
./target/aarch64-apple-darwin/release/3_heap_allocator_flags_pgo ./data/100m.txt
~/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
RUSTFLAGS="-Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release --target=aarch64-apple-darwin
time ./target/aarch64-apple-darwin/release/3_heap_allocator_flags_pgo ./data/1b.txt
```

| Input size | tikv-jemallocator | +PGO     | mimalloc | +PGO     | mimalloc+flags |
|------------|-------------------|----------|----------|----------| ---------------|
| 1m         | 0.64s             | 0.412s   | 0.65s    | 0.404s   | 0.48s          |
| 10m        | 4.19s             | 3.8s     | 4.14s    | 3.789s   | 4.0s           |
| 100m       | 39.51s            | 37.167s  | 40.16s   | 37.17    | 39s            |
| 1b         | 405.17s           | 390.291s | 410.24s  | 380s     | 390s           |

For the record, I noticed some performance loss when removing the power cable and running just on battery so from now on every benchmark will be run with the cable connected.

Keeping all the above compile-time optimizations except for PGO and on top of that taking the following steps allowed me to cut runtime significantly.

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

NVM, the extra 10% performance of PGO are to much to be ignored, adding it to this comparison as well.

| Input size | mimalloc+flags | +HashMap | +PGO   |
|------------|----------------|----------|--------|
| 1m         | 0.48s          | 0.21s    | 0.15s  |
| 10m        | 4.0s           | 1.26s    | 0.915s | 
| 100m       | 39s            | 10.21s   | 8.72s  |
| 1b         | 390s           | 103.62s  | 89s    |

## 4. Optmize data structures

The hashing for the stdlib trades off performaance for security as having a "weak" hashing policy can make the system vulnerable do DOS (Denial of Service) attacks if a attacker can force collisions to degrade applications performance. As I'm not processing user input but my own randomly generated data, that type of safety is not my concern.

I used FxHashMap which has the weakest hashing with the fastest performance.

| Input size | base   | FxHashMap | +PGO   |
|------------|--------|-----------|--------|
| 1m         | 0.13s  | 0.13s     | 0.15s  |
| 10m        | 1.01s  | 0.93s     | 0.915s | 
| 100m       | 10.59s | 9.29s     | 8.72s  |
| 1b         | 99.24s | 91.56s    | 89s    |

I tried using a haspmap of i128 and squeeze all the for variables into i using 16 bits for each min/max, 32 bits for measures count and 64 bit for the toal sum. I used an offset for min/values to simplify its calculations without relying on negative values. 

To avoid floating point operations I converted the measures to int since by definition there'll be only on number after the dot and I just need to divide the values by 10 before calculating the mean. Besides making the code a little bit performant, I noticed mmy program was suffering from floating point imprecisions some stations showed a `0.1` discrepancy.

### How to run

```
./src/bin/4_better_ds/run-all.sh 2>/dev/null
```

## 5. Multi threads

It's time to think about all the CPUs the machine has to offer. I'll start creating a worker pool for each CPU and leave the main thread read rows in chunks;

First attempt was to try builtin `sync::mpsc` but it has so many sharps edges to the point I gave up and decided to try crossbeam channel as thread communication solution. My first implementation had a very degraded performance usually 10x worse for small inputs and about 3~4x slower for the 1BR. I made some optimizations trying to mitigate the amount of time spent by copying thing through threads.

I haven't been able to go beyond my previous record and I assume it's because the bottleneck is on the input not the processing. Making the reader thread use `try_send()` (non-blocking) over `send()` had a significant performance improvement.

Apparently spawning a worker thread for each CPU isn't that optimal at this point. I ran the code with just 1 worker thread and had the best result than running with all cpus or even 2 threads per CPU. Looking in retrospect, this makes sense as I have a single thread writing to the channel a just one consumer.

Replacing the `String` with a `Vec<u8>` as the key fot the stations gave me good results. Once randomly I got times as low as `38s` but I'm not able to reproduce that consistently.

| Input size | Base   | Naive thread | try_send 8 thread | 1 thread  | +PGO  |
|------------|--------|--------------|-------------------|-----------|-------|
| 1m         | 0.13s  | 1.12         | 0.62s             | 0.64s     | 0.13s |
| 10m        | 0.93s  | 3.0          | 1.22s             | 0.93s     | 0.75s |
| 100m       | 9.29s  | 30.2         | 9.73s             | 7.51s     | 7.12s |
| 1b         | 91.56s | 300.0s       | 118.86            | 74.09s    | 71s   |

The bottleneck was in a single thread reading and fanning out the lines through channels to other threads. I changed the program to spawn many threads and set each one in charge of a slice of the file. Each thread read a chunk of data and parse it to text/measure values until the end their partition ends. This required careful manipulation of each partition especially when the measure lied in between each partition. I tested with smaller chunk sizes (around a 1k) and bigger chunk sizes (about 64M) and this later approach was faster but the first had good results without slowing down my laptop. The best result were in the 16 threads range with reasonable chunk size size (4MB).

| Input size | Base   | 8th cnk=1k | 8th cnk=4k | 8th cnk=4m  | 16th cnk=4m | 16th cnk=64m | 128th cnk=64m | PGO    |
|------------|--------|------------|------------|-------------|-------------|--------------|---------------|--------|
| 1m         | 0.64s  | 0.26s      | 0.20s      | 0.16s       | 0.49s       | 0.50s        | 0.50s         | 0.24s  |
| 10m        | 0.93s  | 0.77s      | 0.43s      | 0.33s       | 0.40s       | 0.62s        | 0.62s         | 0.16s  |
| 100m       | 7.51s  | 5.64s      | 3.23s      | 1.85s       | 2.02s       | 2.54s        | 2.06s         | 1.35s  |
| 1b         | 74.09s | 32.94s     | 31.81s     | 17.32s      | 16.88s      | 16.88s       | 19.79s        | 15.99s |


### How to run

```
./src/bin/5_multi_thread/run-all.sh 2>/dev/null
```