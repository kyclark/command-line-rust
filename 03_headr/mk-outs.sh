#!/usr/bin/env bash

ROOT="tests/inputs"
for FILE in $ROOT/*.txt; do
    head      $FILE > $FILE.out
    head -n 2 $FILE > $FILE.n2.out
    head -n 4 $FILE > $FILE.n4.out
done

#ALL="$ROOT/foo.txt $ROOT/fox.txt"
#cat    $ALL > tests/inputs/all.out
#cat -n $ALL > tests/inputs/all.n.out
