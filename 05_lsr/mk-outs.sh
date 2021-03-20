#!/usr/bin/env bash

PRG="cargo run --"
IN_DIR="tests/inputs"
OUT_DIR="tests/expected"
for FILE in $IN_DIR/*.txt; do
    BASENAME=$(basename "$FILE")
    $PRG $FILE > $OUT_DIR/$BASENAME.out
    $PRG -l $FILE > $OUT_DIR/$BASENAME.l.out
done

$PRG    $IN_DIR > $OUT_DIR/dir.out
$PRG -l $IN_DIR > $OUT_DIR/dir.l.out

ALL="$IN_DIR/empty.txt $IN_DIR/one.txt $IN_DIR/two.txt $IN_DIR/three.txt"
$PRG    $ALL > $OUT_DIR/all.out
$PRG -l $ALL > $OUT_DIR/all.l.out
