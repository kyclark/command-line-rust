#!/usr/bin/env bash

CSV="tests/inputs/movies1.csv"
TSV="tests/inputs/movies1.tsv"

for FLD in 1 2 3 1-2 2-3 1-3; do
    cut -f $FLD      $TSV > "${TSV}.f${FLD}.out"
    cut -f $FLD -d , $CSV > "${CSV}.f${FLD}.dcomma.out"
done

for POS in 1 2 8 1-2 2-3 1-8; do
    cut -b $POS $TSV > "${TSV}.b${POS}.out"
    cut -b $POS $CSV > "${CSV}.b${POS}.out"
done

for POS in 1 2 8 1-2 2-3 1-8; do
    cut -c $POS $TSV > "${TSV}.c${POS}.out"
    cut -c $POS $CSV > "${CSV}.c${POS}.out"
done
