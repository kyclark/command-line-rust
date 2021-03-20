#!/usr/bin/env bash

ROOT="tests/inputs"
for FILE in $ROOT/*.txt; do
    tail      $FILE > $FILE.out
    tail -n 2 $FILE > $FILE.n2.out
    tail -n 4 $FILE > $FILE.n4.out
    tail -c 2 $FILE > $FILE.c2.out
    tail -c 4 $FILE > $FILE.c4.out
done

ALL="$ROOT/10.txt $ROOT/empty.txt $ROOT/one.txt $ROOT/three.txt $ROOT/two.txt"
tail      $ALL > $ROOT/all.out
tail -n 1 $ALL > $ROOT/all.n1.out
tail -n 2 $ALL > $ROOT/all.n2.out
tail -c 1 $ALL > $ROOT/all.c1.out
tail -c 2 $ALL > $ROOT/all.c2.out
