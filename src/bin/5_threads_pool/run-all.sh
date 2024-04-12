# declare -a arr=("1k" "10k" "100k" "1m" "10m" "100m" "1b")
declare -a arr=("1m" "10m" "100m" "1b")

solution=4_better_ds
folder=src/bin/$solution/output
mkdir -p $folder

rm -rf target
time cargo build --release --bin 4_better_ds
for i in "${arr[@]}"
do
    file=$folder/$i.txt
    command time -o $file.time \
    cargo run --release --bin $solution data/$i.txt > $file
    echo $i `cat $file.time`
done