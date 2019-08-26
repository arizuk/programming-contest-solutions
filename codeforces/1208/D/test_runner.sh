rustc -o main main.rs -C opt-level=3
for i in `seq 1 100`; do
    ruby case.rb
    res=$(cat input_test|./main)
    ans=$(cat output_test)
    if [ "${res}" != "${ans}" ]; then
        break
    else
        echo "ok ... "
    fi
done
