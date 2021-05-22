#!/usr/bin/env bash

ROOT="tests/inputs"
for FILE in $ROOT/*.txt; do
    head      $FILE > $FILE.out
    head -n 2 $FILE > $FILE.n2.out
    head -n 4 $FILE > $FILE.n4.out
    head -c 1 $FILE > $FILE.c1.out
    head -c 2 $FILE > $FILE.c2.out
    head -c 4 $FILE > $FILE.c4.out
done

ALL="$ROOT/empty.txt $ROOT/one.txt $ROOT/three.txt $ROOT/two.txt"
head      $ALL > $ROOT/all.out
head -n 2 $ALL > $ROOT/all.n2.out
head -n 4 $ALL > $ROOT/all.n4.out
head -c 1 $ALL > $ROOT/all.c1.out
head -c 2 $ALL > $ROOT/all.c2.out
head -c 4 $ALL > $ROOT/all.c4.out
