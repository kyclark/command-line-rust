#!/usr/bin/env bash

ROOT="tests/inputs"
for FILE in $ROOT/*.txt; do
    tail       $FILE > $FILE.out
    tail -n 2  $FILE > $FILE.n2.out
    tail -n 4  $FILE > $FILE.n4.out
    tail -c 8  $FILE > $FILE.c8.out
    tail -c 12 $FILE > $FILE.c12.out
done

ALL="$ROOT/10.txt $ROOT/empty.txt $ROOT/one.txt $ROOT/three.txt $ROOT/two.txt"
tail         $ALL > $ROOT/all.out
tail -n 1    $ALL > $ROOT/all.n1.out
tail -n 2    $ALL > $ROOT/all.n2.out
tail -c 8    $ALL > $ROOT/all.c8.out
tail -c 12   $ALL > $ROOT/all.c12.out
tail -n 2 -q $ALL > $ROOT/all.n2.q.out
