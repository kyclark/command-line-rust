#!/usr/bin/env bash

set -u

ROOT="tests/inputs"
EMPTY="$ROOT/empty.txt"
FOX="$ROOT/fox.txt"
SPIDERS="$ROOT/spiders.txt"
BUSTLE="$ROOT/the-bustle.txt"
ALL="$EMPTY $FOX $SPIDERS $BUSTLE"

for FILE in $ALL; do
    cat    $FILE > $FILE.out
    cat -n $FILE > $FILE.n.out
    cat -b $FILE > $FILE.b.out
done

cat    $ALL > $ROOT/all.out
cat -n $ALL > $ROOT/all.n.out
cat -b $ALL > $ROOT/all.b.out

cat    < $BUSTLE > ${BUSTLE}.stdin.out
cat -n < $BUSTLE > ${BUSTLE}.n.stdin.out
cat -b < $BUSTLE > ${BUSTLE}.b.stdin.out

