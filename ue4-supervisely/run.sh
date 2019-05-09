#!/usr/bin/env bash
INPATH="/datasets/synthetic/UE4/generated_synthetic1"
OUTPATH="/datasets/synthetic/supervisely/generated_synthetic1"
DATASET_NAME="UE4 Triple Chasers (top)"
LABEL="triplecanister_top"

python lib/convert_masks.py --in "$INPATH" --out "$OUTPATH" --name "$DATASET_NAME" --label "$LABEL"
