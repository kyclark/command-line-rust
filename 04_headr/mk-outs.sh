#!/usr/bin/env bash

INPUTS="./tests/inputs"
OUT_DIR="./tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

for FILE in $INPUTS/*.txt; do
    BASENAME=$(basename "$FILE")
    head      $FILE > ${OUT_DIR}/${BASENAME}.out
    head -n 2 $FILE > ${OUT_DIR}/${BASENAME}.n2.out
    head -n 4 $FILE > ${OUT_DIR}/${BASENAME}.n4.out
    head -c 1 $FILE > ${OUT_DIR}/${BASENAME}.c1.out
    head -c 2 $FILE > ${OUT_DIR}/${BASENAME}.c2.out
    head -c 4 $FILE > ${OUT_DIR}/${BASENAME}.c4.out
done

ALL="$INPUTS/empty.txt $INPUTS/one.txt $INPUTS/two.txt $INPUTS/three.txt \
    $INPUTS/ten.txt"
head      $ALL > $OUT_DIR/all.out
head -n 2 $ALL > $OUT_DIR/all.n2.out
head -n 4 $ALL > $OUT_DIR/all.n4.out
head -c 1 $ALL > $OUT_DIR/all.c1.out
head -c 2 $ALL > $OUT_DIR/all.c2.out
head -c 4 $ALL > $OUT_DIR/all.c4.out
