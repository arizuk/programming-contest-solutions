rustc -o seg seg.rs -C opt-level=3
rustc -o dp dp.rs -C opt-level=3
for i in `seq 1 100`; do
    ruby case.rb > case
    seg=$(cat case|./seg)
    dp=$(cat case|./dp)
    if [ "$seg" != "$dp" ]; then
        echo $seg
        echo $dp
        break
    else
        echo "ok ... ${seg} ${dp}"
    fi
done
rm seg dp
rm case