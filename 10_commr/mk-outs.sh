#!/usr/bin/env bash

set -u
IN_DIR="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

rm -f $OUT_DIR/*

comm         $IN_DIR/empty.txt $IN_DIR/empty.txt > $OUT_DIR/empty_empty.out
comm         $IN_DIR/file1.txt $IN_DIR/empty.txt > $OUT_DIR/file1_empty.out
comm         $IN_DIR/empty.txt $IN_DIR/file2.txt > $OUT_DIR/empty_file2.out
comm         $IN_DIR/file1.txt $IN_DIR/file1.txt > $OUT_DIR/file1_file1.out

comm         $IN_DIR/blank.txt $IN_DIR/file1.txt > $OUT_DIR/blank_file1.out
comm         $IN_DIR/file1.txt $IN_DIR/blank.txt > $OUT_DIR/file1_blank.out
comm -1      $IN_DIR/file1.txt $IN_DIR/blank.txt > $OUT_DIR/file1_blank.1.out
comm -2      $IN_DIR/file1.txt $IN_DIR/blank.txt > $OUT_DIR/file1_blank.2.out
comm -3      $IN_DIR/file1.txt $IN_DIR/blank.txt > $OUT_DIR/file1_blank.3.out

comm         $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.out
comm -1      $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.1.out
comm -2      $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.2.out
comm -3      $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.3.out

comm -12     $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.12.out
comm -23     $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.23.out
comm -13     $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.13.out
comm -123    $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.123.out

comm -i -1   $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.1.i.out
comm -i -2   $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.2.i.out
comm -i -3   $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.3.i.out

comm -i -12  $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.12.i.out
comm -i -23  $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.23.i.out
comm -i -13  $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.13.i.out
comm -i -123 $IN_DIR/file1.txt $IN_DIR/file2.txt > $OUT_DIR/file1_file2.123.i.out

comm         $IN_DIR/file1.txt $IN_DIR/file2.txt | sed "s/\t/:/g" > $OUT_DIR/file1_file2.delim.out
comm -1      $IN_DIR/file1.txt $IN_DIR/file2.txt | sed "s/\t/:/g" > $OUT_DIR/file1_file2.1.delim.out
comm -2      $IN_DIR/file1.txt $IN_DIR/file2.txt | sed "s/\t/:/g" > $OUT_DIR/file1_file2.2.delim.out
comm -3      $IN_DIR/file1.txt $IN_DIR/file2.txt | sed "s/\t/:/g" > $OUT_DIR/file1_file2.3.delim.out
comm -12     $IN_DIR/file1.txt $IN_DIR/file2.txt | sed "s/\t/:/g" > $OUT_DIR/file1_file2.12.delim.out
comm -23     $IN_DIR/file1.txt $IN_DIR/file2.txt | sed "s/\t/:/g" > $OUT_DIR/file1_file2.23.delim.out
comm -13     $IN_DIR/file1.txt $IN_DIR/file2.txt | sed "s/\t/:/g" > $OUT_DIR/file1_file2.13.delim.out
comm -123    $IN_DIR/file1.txt $IN_DIR/file2.txt | sed "s/\t/:/g" > $OUT_DIR/file1_file2.123.delim.out
