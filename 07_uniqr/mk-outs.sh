#!/usr/bin/env bash

ROOT="tests/inputs"

# Cf https://github.com/coreutils/coreutils/blob/master/tests/misc/uniq.pl
echo -ne "a\na\n"    > $ROOT/t1.txt
echo -ne "a\na"      > $ROOT/t2.txt
echo -ne "a\nb"      > $ROOT/t3.txt
echo -ne "a\na\nb"   > $ROOT/t4.txt
echo -ne "b\na\na\n" > $ROOT/t5.txt
echo -ne "a\nb\nc\n" > $ROOT/t6.txt

for FILE in $ROOT/*.txt; do
    uniq $FILE > $FILE.out
    uniq -c $FILE > $FILE.c.out
    uniq < $FILE > $FILE.stdin.out
    uniq -c < $FILE > $FILE.stdin.c.out
done
