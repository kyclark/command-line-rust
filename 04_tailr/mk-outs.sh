#!/usr/bin/env bash

ROOT="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

for FILE in $ROOT/*.txt; do
    BASENAME=$(basename "$FILE")
    tail       $FILE > ${OUT_DIR}/${BASENAME}.out
    tail -n 3  $FILE > ${OUT_DIR}/${BASENAME}.n3.out
    tail -n 4  $FILE > ${OUT_DIR}/${BASENAME}.n4.out
    tail -c 8  $FILE > ${OUT_DIR}/${BASENAME}.c8.out
    tail -c 12 $FILE > ${OUT_DIR}/${BASENAME}.c12.out
done

ALL="$ROOT/10.txt $ROOT/empty.txt $ROOT/one.txt $ROOT/three.txt $ROOT/two.txt"

tail         $ALL > $OUT_DIR/all.out
tail -n 1    $ALL > $OUT_DIR/all.n1.out
tail -n 3    $ALL > $OUT_DIR/all.n3.out
tail -c 8    $ALL > $OUT_DIR/all.c8.out
tail -c 12   $ALL > $OUT_DIR/all.c12.out
tail -n 3 -q $ALL > $OUT_DIR/all.n3.q.out
