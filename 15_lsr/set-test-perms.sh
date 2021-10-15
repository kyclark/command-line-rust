#!/usr/bin/env bash

set -u
DIR=${1:-$PWD}

if [[ $DIR == "-h" ]] || [[ $DIR == "--help" ]]; then
    printf "Usage: %s DIR\n" $(basename "$0")
    exit 0
fi

chmod 755 ${DIR}/tests/inputs/dir
chmod 600 ${DIR}/tests/inputs/fox.txt
chmod 644 ${DIR}/tests/inputs/.hidden ${DIR}/tests/inputs/empty.txt \
    ${DIR}/tests/inputs/bustle.txt ${DIR}/tests/inputs/dir/.gitkeep \
    ${DIR}/tests/inputs/dir/spiders.txt

echo "Done, fixed files in \"$DIR\"."
