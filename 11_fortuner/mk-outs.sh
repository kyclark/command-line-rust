#!/usr/bin/env bash

IN_DIR="$PWD/tests/inputs"
OUT_DIR="$PWD/tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

FILES="fortunes humorists"

cd $IN_DIR

fortune -m 'Yogi Berra' $FILES \
    1>$OUT_DIR/berra_cap.out 2>$OUT_DIR/berra_cap.err
fortune -m 'Will Rogers' $FILES \
    1>$OUT_DIR/rogers_cap.out 2>$OUT_DIR/rogers_cap.err

fortune -m 'yogi berra' $FILES \
    1>$OUT_DIR/berra_lower.out 2>$OUT_DIR/berra_lower.err
fortune -m 'will rogers' $FILES \
    1>$OUT_DIR/rogers_lower.out 2>$OUT_DIR/rogers_lower.err

fortune -i -m 'yogi berra' $FILES \
    1>$OUT_DIR/berra_lower_i.out 2>$OUT_DIR/berra_lower_i.err
fortune -i -m 'will rogers' $FILES \
    1>$OUT_DIR/rogers_lower_i.out 2>$OUT_DIR/rogers_lower_i.err
