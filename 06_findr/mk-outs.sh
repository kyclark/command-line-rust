#!/usr/bin/env bash

IN_DIR="tests/inputs"
OUT_DIR="tests/expected"

if [[ ! -d "$IN_DIR" ]]; then
    echo "Missing IN_DIR \"$IN_DIR\""
    exit 1
fi

[[ -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"
rm $OUT_DIR/*.txt

find "$IN_DIR" > "$OUT_DIR/path1.txt"
find "$IN_DIR/a" > "$OUT_DIR/path_a.txt"
find "$IN_DIR/a/b" > "$OUT_DIR/path_a_b.txt"
find "$IN_DIR/d" > "$OUT_DIR/path_d.txt"
find "$IN_DIR/a/b" "$IN_DIR/d" > "$OUT_DIR/path_a_b_d.txt"

find "$IN_DIR" -type f > "$OUT_DIR/type_f.txt"
find "$IN_DIR/a" -type f > "$OUT_DIR/type_f_path_a.txt"
find "$IN_DIR/a/b" -type f > "$OUT_DIR/type_f_path_a_b.txt"
find "$IN_DIR/d" -type f > "$OUT_DIR/type_f_path_d.txt"
find "$IN_DIR/a/b" "$IN_DIR/d" -type f > "$OUT_DIR/type_f_path_a_b_d.txt"

find "$IN_DIR" -type d > "$OUT_DIR/type_d.txt"
find "$IN_DIR/a" -type d > "$OUT_DIR/type_d_path_a.txt"
find "$IN_DIR/a/b" -type d > "$OUT_DIR/type_d_path_a_b.txt"
find "$IN_DIR/d" -type d > "$OUT_DIR/type_d_path_d.txt"
find "$IN_DIR/a/b" "$IN_DIR/d" -type d > "$OUT_DIR/type_d_path_a_b_d.txt"

find "$IN_DIR" -type l > "$OUT_DIR/type_l.txt"
find "$IN_DIR" -type f -o -type l > "$OUT_DIR/type_f_l.txt"

find "$IN_DIR" -name \*.csv > "$OUT_DIR/name_csv.txt"
find "$IN_DIR" -name \*.csv -o -name \*.mp3 > "$OUT_DIR/name_csv_mp3.txt"
find "$IN_DIR/a" "$IN_DIR/d" -name \*.txt > "$OUT_DIR/name_txt_path_a_d.txt"

find "$IN_DIR" -name a* > "$OUT_DIR/name_a.txt"
find "$IN_DIR" -type f -name a* > "$OUT_DIR/type_f_name_a.txt"
find "$IN_DIR" -type d -name a* > "$OUT_DIR/type_d_name_a.txt"
