set -e
file=${1:-main.rs}
rustc -o main ${file} -C opt-level=3
rustc -o judge judge.rs -C opt-level=3
for f in `ls cases/input*`; do
    cat $f|./main > output
    res=$(cat $f output|./judge)
    if [ "${res}" != "ok" ]; then
        echo "# test failed! ${f}"
        echo "----------------- res"
        echo $(cat output)
        echo "----------------- ans"
        echo ${res}
        break
    else
        echo "${f}: ok ... "
    fi
done
