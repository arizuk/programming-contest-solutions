set -e
rustc -o main ${1} -C opt-level=3
for i in `seq 1 100`; do
    ruby case.rb
    res=$(cat input_test|./main)
    ans=$(cat output_test)
    if [ "${res}" != "${ans}" ]; then
        echo "# test failed!"
        echo "----------------- res"
        echo ${res}
        echo "----------------- ans"
        echo ${ans}
        break
    else
        echo "${i}: ok ... "
    fi
done
