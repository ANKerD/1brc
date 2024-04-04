declare -a arr=("1k" "10k" "100k" "1m" "10m" "100m" "1b")

solution=1_naive_implementation
folder=src/bin/$solution/output
mkdir -p $folder

for i in "${arr[@]}"
do
    file=$folder/$i.txt
    command time -o $file.time cargo run --bin $solution data/$i.txt > $file
    echo $i `cat $file.time`
done