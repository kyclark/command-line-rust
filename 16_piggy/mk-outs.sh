#!/usr/bin/env bash

PRG="cargo run --"

for FILE in tests/inputs/*.txt; do
    echo "$(basename "$FILE")"
    $PRG $FILE > $FILE.out 
done

echo "Done."
