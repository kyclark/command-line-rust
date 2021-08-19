#!/usr/bin/env bash

set -u

EMPTY="tests/inputs/empty.txt"
ONE="tests/inputs/one.txt"
TWO="tests/inputs/two.txt"
THREE="tests/inputs/three.txt"
NAMES="tests/inputs/names.txt"
DEADLIFTS="tests/inputs/deadlifts.txt"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

rm $OUT_DIR/*

paste $EMPTY                   > $OUT_DIR/empty.out
paste $ONE                     > $OUT_DIR/one.out
paste $TWO                     > $OUT_DIR/two.out
paste $EMPTY $TWO              > $OUT_DIR/empty_two.out
paste -d "," $TWO $EMPTY       > $OUT_DIR/two_empty_d_comma.out
paste $ONE $TWO                > $OUT_DIR/one_two.out
paste $TWO $THREE              > $OUT_DIR/two_three.out
paste $EMPTY $ONE $TWO $THREE  > $OUT_DIR/empty_one_two_three.out
paste $NAMES                   > $OUT_DIR/names.out
paste -d ":" $NAMES $DEADLIFTS > $OUT_DIR/names_deadlifts_d_colon.out
paste -s -d '\t\n' $NAMES      > $OUT_DIR/names_serial_delims.out
