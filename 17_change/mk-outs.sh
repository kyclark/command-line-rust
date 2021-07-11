#!/usr/bin/env bash

PRG="cargo run --"
OUTDIR="tests/expected"

[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

for NUM in $(seq 1 27); do
    echo $NUM
    $PRG $NUM > $OUTDIR/$NUM.out 
done

echo "Done."
