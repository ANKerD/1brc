echo $1 $2 $3
f1="./src/bin/$1/output/$3"
f2="./src/bin/$2/output/$3"
diff $f1 $f2