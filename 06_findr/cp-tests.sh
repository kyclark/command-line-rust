#!/usr/bin/env bash

if [[ $# -ne 1 ]]; then
    printf "Usage: %s DEST_DIR\n" $(basename "$0")
    exit 1
fi

DEST=$1

if [[ ! -d "$DEST" ]]; then
    echo "\"$DEST\" is not a directory"
    exit 1
fi

echo "Copying \"tests\" to \"$DEST\""
cp -r tests "$DEST"
cd "$DEST"

echo "Fixing symlink"
rm tests/inputs/d/b.csv
ln -s tests/inputs/a/b.csv tests/inputs/d/b.csv

echo "Done."
