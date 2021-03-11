#!/usr/bin/env bash

ROOT="tests/inputs"
for FILE in $ROOT/*.txt; do
    cat    $FILE > $FILE.out
    cat -n $FILE > $FILE.n.out
done

ALL="$ROOT/foo.txt $ROOT/fox.txt"
cat    $ALL > tests/inputs/all.out
cat -n $ALL > tests/inputs/all.n.out
