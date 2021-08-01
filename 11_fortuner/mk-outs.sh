#!/usr/bin/env bash

IN_DIR="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

fortune -m 'Yogi Berra' $IN_DIR \
    1>$OUT_DIR/berra_cap.out 2>$OUT_DIR/berra_cap.err
fortune -m 'Will Rogers' $IN_DIR \
    1>$OUT_DIR/rogers_cap.out 2>$OUT_DIR/rogers_cap.err

fortune -m 'yogi berra' $IN_DIR \
    1>$OUT_DIR/berra_lower.out 2>$OUT_DIR/berra_lower.err
fortune -m 'will rogers' \
    $IN_DIR 1>$OUT_DIR/rogers_lower.out 2>$OUT_DIR/rogers_lower.err

fortune -i -m 'yogi berra' $IN_DIR \
    1>$OUT_DIR/berra_lower_i.out 2>$OUT_DIR/berra_lower_i.err
fortune -i -m 'will rogers' $IN_DIR \
    1>$OUT_DIR/rogers_lower_i.out 2>$OUT_DIR/rogers_lower_i.err
