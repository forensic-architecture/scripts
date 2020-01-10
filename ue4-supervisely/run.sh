#!/usr/bin/env bash

BASE_PATH=/workspace/datasets/_sy

for FOLD in 2019.04.24-FA-001 2019.04.25-FA-003 2019.05.02-FA-002
do
	INPATH="$BASE_PATH/$FOLD"
	OUTPATH="$INPATH/supervisely"
	DATASET_NAME="UE4 Triple Chasers (top)"
	LABEL="triplecanister_top"
	FORMAT="unreal"

	# NOTE: this line is only necessary if working straight from unreal or unity exports.
	# python lib/reorder.py --folder "$INPATH" --format "$FORMAT"
	python lib/convert_masks.py \
		--infolder "$INPATH" \
		--outfolder "$OUTPATH" \
		--name "$DATASET_NAME" \
		--label "$LABEL"
done
