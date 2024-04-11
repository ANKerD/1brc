# declare -a arr=("10" "100" "1k" "10k" "100k" "1m" "10m" "100m" "1b")
declare -a arr=("1m" "10m" "100m" "1b")

solution=5_multi_thread
folder=src/bin/$solution/output
mkdir -p $folder

rm -rf target
time cargo build --release --bin $solution
date '+%Y-%m-%dT%H:%M:%S'
for i in "${arr[@]}"
do
    file=$folder/$i.txt
    command time -o $file.time \
    cargo run --release --bin $solution data/$i.txt > $file
    # echo cargo run --release --bin $solution data/$i.txt
    echo $i `cat $file.time`
done

rm -rf /tmp/pgo-data
RUSTFLAGS="-C llvm-args=-vp-counters-per-site=16 -Cprofile-generate=/tmp/pgo-data" cargo build --release --target=aarch64-apple-darwin --bin 5_multi_thread
./target/aarch64-apple-darwin/release/5_multi_thread ./data/1m.txt 1>/dev/null
./target/aarch64-apple-darwin/release/5_multi_thread ./data/10m.txt 1>/dev/null
./target/aarch64-apple-darwin/release/5_multi_thread ./data/100m.txt 1>/dev/null
~/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/bin/llvm-profdata merge -o /tmp/pgo-data/merged.profdata /tmp/pgo-data
RUSTFLAGS="-C llvm-args=-vp-counters-per-site=16 -Cprofile-use=/tmp/pgo-data/merged.profdata" cargo build --release --target=aarch64-apple-darwin --bin 5_multi_thread
echo \n\n
echo "PGO"
time ./target/aarch64-apple-darwin/release/5_multi_thread ./data/1m.txt 1>/dev/null
time ./target/aarch64-apple-darwin/release/5_multi_thread ./data/10m.txt 1>/dev/null
time ./target/aarch64-apple-darwin/release/5_multi_thread ./data/100m.txt 1>/dev/null
time ./target/aarch64-apple-darwin/release/5_multi_thread ./data/1b.txt 1>/dev/null