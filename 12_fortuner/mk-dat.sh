#!/usr/bin/env bash

DIR="./tests/inputs"
cd $DIR
rm -f *.dat

for FILE in *; do
    echo $FILE
    strfile -c % $FILE $FILE.dat > /dev/null
done

echo "Done."
