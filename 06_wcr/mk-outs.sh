#!/usr/bin/env bash

ROOT="tests/inputs"
FILES="$ROOT/empty.txt $ROOT/fox.txt $ROOT/atlamal.txt"

for FILE in $FILES; do
    wc      $FILE > $FILE.out
    wc -l   $FILE > $FILE.l.out
    wc -w   $FILE > $FILE.w.out
    wc -c   $FILE > $FILE.c.out
    wc -m   $FILE > $FILE.m.out
    wc -lwm $FILE > $FILE.lwm.out
    wc -wc  $FILE > $FILE.wc.out
    wc -wm  $FILE > $FILE.wm.out
    wc -wl  $FILE > $FILE.wl.out
    wc -cl  $FILE > $FILE.cl.out
    wc -ml  $FILE > $FILE.ml.out
done

wc < "$ROOT/atlamal.txt" > "$ROOT/atlamal.txt.stdin.out"

wc      $FILES > tests/inputs/all.out
wc -l   $FILES > tests/inputs/all.l.out
wc -w   $FILES > tests/inputs/all.w.out
wc -c   $FILES > tests/inputs/all.c.out
wc -m   $FILES > tests/inputs/all.m.out
wc -lwm $FILES > tests/inputs/all.lwm.out
wc -wc  $FILES > tests/inputs/all.wc.out
wc -wm  $FILES > tests/inputs/all.wm.out
wc -wl  $FILES > tests/inputs/all.wl.out
wc -cl  $FILES > tests/inputs/all.cl.out
wc -ml  $FILES > tests/inputs/all.ml.out
