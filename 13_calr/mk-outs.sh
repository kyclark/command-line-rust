#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

cal 2020 > $OUTDIR/2020.txt
cal 2 2020 > $OUTDIR/2-2020.txt
cal 4 2020 > $OUTDIR/4-2020.txt
cal 5 2020 > $OUTDIR/5-2020.txt
