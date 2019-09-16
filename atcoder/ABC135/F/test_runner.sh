set -e

p1=rolling_hash
p2=kmp
e1=exe_${p1}
e2=exe_${p2}

rustc -o ${e1} ${p1}.rs -C opt-level=3
rustc -o ${e2} ${p2}.rs -C opt-level=3

for i in `seq 1 1000`; do
    ruby case.rb > input_case
    ans_p1=$(cat input_case|./${e1})
    ans_p2=$(cat input_case|./${e2})
    if [ "$ans_p1" != "$ans_p2" ]; then
        echo "ng ... test failed"
        echo ${e1}: $ans_p1
        echo ${e2}: $ans_p2
        exit 1
    else
        echo "${i} ok ... ${ans_p1} ${ans_p2}"
    fi
done
rm ${e1} ${e2}
rm input_case