#!/usr/bin/env bash

DIR="./tests/inputs"
cd $DIR
rm -f *.dat

for FILE in *; do
    if [[ -f $FILE ]]; then
        echo $FILE
        strfile -c % $FILE $FILE.dat > /dev/null
    fi
done

echo "Done."
