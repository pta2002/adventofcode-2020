#!/usr/bin/env nix-shell
#!nix-shell -p cargo -p rustc -p hyperfine -p jq -i bash

rm -f report.txt
for d in */; do
  cd $d

  if [ -f "input.txt" ]; then
    echo "--- Benchmarking $d ---"
    cargo build --release
    hyperfine --export-json benchmark.json target/release/aoc

    TIME=$(printf "%.5f" $(jq .results[0].mean benchmark.json))

    echo "Took ${TIME}s"

    echo "${d}: ${TIME}s" >> ../report.txt
  else
    echo "[!] Skipping $d, no input"
  fi

  cd ..
done
