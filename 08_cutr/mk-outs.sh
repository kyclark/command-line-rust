#!/usr/bin/env bash

DIR="tests/inputs/"

for FLD in 1 2 3 1-2 2-3 1-3; do
    cut -f $FLD      "$DIR/1.tsv" > "$DIR/1.tsv.f${FLD}.out"
    cut -f $FLD -d , "$DIR/1.csv" > "$DIR/1.csv.f${FLD}.dcomma.out"
done

for BYTES in 1 2 3 1-2 2-3 1-3; do
    cut -b $FLD "$DIR/1.tsv" > "$DIR/1.tsv.b${BYTES}.out"
    cut -b $FLD "$DIR/1.csv" > "$DIR/1.csv.b${BYTES}.out"
done
