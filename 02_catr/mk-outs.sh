#!/usr/bin/env bash

ROOT="tests/inputs"
for FILE in $ROOT/*.txt; do
    cat    $FILE > $FILE.out
    cat -n $FILE > $FILE.n.out
    cat -b $FILE > $FILE.b.out
done

BUSTLE=$ROOT/the-bustle.txt 
cat    < $BUSTLE > ${BUSTLE}.stdin.out
cat -n < $BUSTLE > ${BUSTLE}.n.stdin.out
cat -b < $BUSTLE > ${BUSTLE}.b.stdin.out

ALL="$ROOT/empty.txt $ROOT/fox.txt $ROOT/spiders.txt $ROOT/the-bustle.txt"
cat    $ALL > $ROOT/all.out
cat -n $ALL > $ROOT/all.n.out
cat -b $ALL > $ROOT/all.b.out
