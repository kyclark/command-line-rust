#!/usr/bin/env bash

ROOT="tests/inputs"
FILES="$ROOT/empty.txt $ROOT/fox.txt $ROOT/atlamal.txt"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

for FILE in $FILES; do
    BASENAME=$(basename "$FILE")
    wc      $FILE > ${OUT_DIR}/${BASENAME}.out
    wc -l   $FILE > ${OUT_DIR}/${BASENAME}.l.out
    wc -w   $FILE > ${OUT_DIR}/${BASENAME}.w.out
    wc -c   $FILE > ${OUT_DIR}/${BASENAME}.c.out
    wc -m   $FILE > ${OUT_DIR}/${BASENAME}.m.out
    wc -lwm $FILE > ${OUT_DIR}/${BASENAME}.lwm.out
    wc -wc  $FILE > ${OUT_DIR}/${BASENAME}.wc.out
    wc -wm  $FILE > ${OUT_DIR}/${BASENAME}.wm.out
    wc -wl  $FILE > ${OUT_DIR}/${BASENAME}.wl.out
    wc -cl  $FILE > ${OUT_DIR}/${BASENAME}.cl.out
    wc -ml  $FILE > ${OUT_DIR}/${BASENAME}.ml.out
done

wc < "$ROOT/atlamal.txt" > "$OUT_DIR/atlamal.txt.stdin.out"

wc      $FILES > $OUT_DIR/all.out
wc -l   $FILES > $OUT_DIR/all.l.out
wc -w   $FILES > $OUT_DIR/all.w.out
wc -c   $FILES > $OUT_DIR/all.c.out
wc -m   $FILES > $OUT_DIR/all.m.out
wc -lwm $FILES > $OUT_DIR/all.lwm.out
wc -wc  $FILES > $OUT_DIR/all.wc.out
wc -wm  $FILES > $OUT_DIR/all.wm.out
wc -wl  $FILES > $OUT_DIR/all.wl.out
wc -cl  $FILES > $OUT_DIR/all.cl.out
wc -ml  $FILES > $OUT_DIR/all.ml.out
