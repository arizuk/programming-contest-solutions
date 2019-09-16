set -e
rustc -o main test.rs -C opt-level=3
for i in `seq 1 100`; do
    ruby case.rb > input_case
    cat input_case|./main
    echo "${i}: ok ..."
done
rm input_case