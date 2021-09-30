#!/usr/bin/env bash

ROOT="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

rm -f $OUT_DIR/*

for FILE in $ROOT/*.txt; do
    BASENAME=$(basename "$FILE")
    tail        $FILE > ${OUT_DIR}/${BASENAME}.out
    tail -n 0   $FILE > ${OUT_DIR}/${BASENAME}.n0.out
    tail -n 1   $FILE > ${OUT_DIR}/${BASENAME}.n1.out
    tail -n 3   $FILE > ${OUT_DIR}/${BASENAME}.n3.out
    tail -n 4   $FILE > ${OUT_DIR}/${BASENAME}.n4.out
    tail -n 200 $FILE > ${OUT_DIR}/${BASENAME}.n200.out
    tail -c 3   $FILE > ${OUT_DIR}/${BASENAME}.c3.out
    tail -c 8   $FILE > ${OUT_DIR}/${BASENAME}.c8.out
    tail -c 12  $FILE > ${OUT_DIR}/${BASENAME}.c12.out
    tail -c 200 $FILE > ${OUT_DIR}/${BASENAME}.c200.out

    tail -n +0  $FILE > ${OUT_DIR}/${BASENAME}.n+0.out
    tail -n +1  $FILE > ${OUT_DIR}/${BASENAME}.n+1.out
    tail -n +2  $FILE > ${OUT_DIR}/${BASENAME}.n+2.out
    tail -c +0  $FILE > ${OUT_DIR}/${BASENAME}.c+0.out
    tail -c +1  $FILE > ${OUT_DIR}/${BASENAME}.c+1.out
    tail -c +2  $FILE > ${OUT_DIR}/${BASENAME}.c+2.out
done

ALL="$ROOT/ten.txt $ROOT/empty.txt $ROOT/one.txt $ROOT/three.txt $ROOT/two.txt"

tail         $ALL > $OUT_DIR/all.out
tail -n 0    $ALL > $OUT_DIR/all.n0.out
tail -n 1    $ALL > $OUT_DIR/all.n1.out
tail -n 1 -q $ALL > $OUT_DIR/all.n1.q.out
tail -n 3    $ALL > $OUT_DIR/all.n3.out
tail -c 0    $ALL > $OUT_DIR/all.c0.out
tail -c 3    $ALL > $OUT_DIR/all.c3.out
tail -c 8    $ALL > $OUT_DIR/all.c8.out
tail -c 12   $ALL > $OUT_DIR/all.c12.out
tail -n 3 -q $ALL > $OUT_DIR/all.n3.q.out

tail -n +1    $ALL > $OUT_DIR/all.n+1.out
tail -n +3    $ALL > $OUT_DIR/all.n+3.out
tail -c +3    $ALL > $OUT_DIR/all.c+3.out
tail -c +8    $ALL > $OUT_DIR/all.c+8.out
tail -c +12   $ALL > $OUT_DIR/all.c+12.out
tail -n +3 -q $ALL > $OUT_DIR/all.n+3.q.out
