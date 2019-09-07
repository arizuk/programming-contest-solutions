#!/bin/bash -eu

prog=${1}
rustc ${prog} -o out -C opt-level=3 -C debug_assertions=no -C link-args=-Wl,-S
source=$(gzip -c out | base64 -w 0)
cat << EOS > main.bash
echo '${source}' | base64 -d | gunzip > /tmp/a.out && chmod +x /tmp/a.out && /tmp/a.out
EOS

echo $(ls -lh main.bash)