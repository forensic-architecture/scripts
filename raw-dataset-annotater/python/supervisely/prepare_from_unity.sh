#!/usr/bin/env bash
BASE=$(realpath $1)

mv $BASE/X $BASE/img
mv $BASE/Y $BASE/raw_ann
