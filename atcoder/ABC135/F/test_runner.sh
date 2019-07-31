set -e

p1=main
p2=accepted

rustc -o ${p1} ${p1}.rs -C opt-level=3
rustc -o ${p2} ${p2}.rs -C opt-level=3

for i in `seq 1 1000`; do
    ruby case.rb > case.dat
    ans_p1=$(cat case.dat|./${p1})
    ans_p2=$(cat case.dat|./${p2})
    if [ "$ans_p1" != "$ans_p2" ]; then
        echo "ng ... test failed"
        echo ${p1}: $ans_p1
        echo ${p2}: $ans_p2
        exit 1
    else
        echo "${i} ok ... ${ans_p1} ${ans_p2}"
    fi
done
rm ${p1} ${p2}
rm case.dat