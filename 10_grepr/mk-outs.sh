#!/usr/bin/env bash

set -u

DIR="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

rm -f "$OUT_DIR/*"

grep foo $DIR/empty.txt > "$OUT_DIR/empty.foo"
grep The tests/inputs/bustle.txt > "$OUT_DIR/bustle.txt.the.capitalized"
grep the tests/inputs/bustle.txt > "$OUT_DIR/bustle.txt.the.lowercase"
grep -i the tests/inputs/bustle.txt > "$OUT_DIR/bustle.txt.the.lowercase.insensitive"

grep nobody tests/inputs/nobody.txt > "$OUT_DIR/nobody.txt"
grep -i nobody tests/inputs/nobody.txt > "$OUT_DIR/nobody.txt.insensitive"

grep The $DIR/*.txt > "$OUT_DIR/all.the.capitalized"
grep -i the $DIR/*.txt > "$OUT_DIR/all.the.lowercase.insensitive"

grep -r dog tests/inputs > "$OUT_DIR/dog.recursive"
grep -ri the tests/inputs > "$OUT_DIR/the.recursive.insensitive"
