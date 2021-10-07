#!/usr/bin/env bash

IN_DIR="$PWD/tests/inputs"
OUT_DIR="$PWD/tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

rm -f $OUT_DIR/*

FILES="literature quotes"

cd $IN_DIR

fortune -m 'Yogi Berra' $FILES \
    1>$OUT_DIR/berra_cap.out 2>$OUT_DIR/berra_cap.err
fortune -m 'Mark Twain' $FILES \
    1>$OUT_DIR/twain_cap.out 2>$OUT_DIR/twain_cap.err

fortune -m 'yogi berra' $FILES \
    1>$OUT_DIR/berra_lower.out 2>$OUT_DIR/berra_lower.err
fortune -m 'mark twain' $FILES \
    1>$OUT_DIR/twain_lower.out 2>$OUT_DIR/twain_lower.err

fortune -i -m 'yogi berra' $FILES \
    1>$OUT_DIR/berra_lower_i.out 2>$OUT_DIR/berra_lower_i.err
fortune -i -m 'mark twain' $FILES \
    1>$OUT_DIR/twain_lower_i.out 2>$OUT_DIR/twain_lower_i.err
