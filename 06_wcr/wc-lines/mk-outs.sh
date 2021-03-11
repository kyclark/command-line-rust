#!/usr/bin/env bash

ROOT="tests/inputs"
for FILE in $ROOT/*.txt; do
    wc     $FILE > $FILE.out
    wc -w  $FILE > $FILE.w.out
    wc -c  $FILE > $FILE.c.out
    wc -l  $FILE > $FILE.l.out
    wc -wc $FILE > $FILE.wc.out
    wc -wl $FILE > $FILE.wl.out
    wc -cl $FILE > $FILE.cl.out
done

ALL="$ROOT/empty.txt $ROOT/one.txt $ROOT/two.txt $ROOT/three.txt"
wc     $ALL > tests/inputs/all.out
wc -w  $ALL > tests/inputs/all.w.out
wc -c  $ALL > tests/inputs/all.c.out
wc -l  $ALL > tests/inputs/all.l.out
wc -wc $ALL > tests/inputs/all.wc.out
wc -wl $ALL > tests/inputs/all.wl.out
wc -cl $ALL > tests/inputs/all.cl.out
