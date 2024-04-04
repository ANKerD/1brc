declare -a arr=("1k" "10k" "100k" "1m" "10m" "100m" "1b")

solution=2_optimize_build
folder=src/bin/$solution/output
mkdir -p $folder

rm -rf target
cargo build --release
for i in "${arr[@]}"
do
    file=$folder/$i.txt
    command time -o $file.time cargo run --release --bin $solution data/$i.txt > $file
    echo $i `cat $file.time`
done