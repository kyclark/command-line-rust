#!/usr/bin/env bash

DIRS=$(find . -mindepth 1 -maxdepth 1 -type d -name [01][0-9]_\* | sort)
for DIR in $DIRS; do
    DIRNAME=$(basename "$DIR")
    echo "==> $DIRNAME <=="
    (cd $DIR && cargo test -q > /dev/null && cargo clippy)
    [[ $DIRNAME == "10_tailr" ]] && break
done

echo "Done."
