#!/usr/bin/env bash

BASE_PATH=/datasets/synthetic-datasets/Triple-Chaser
folders=( "2019.04.15-FA-002" "2019.04.20-FA-003" "2019.04.23-FA-004" "2019.04.24-FA-001" "2019.04.25-FA-003" "2019.05.02-FA-002" )


for FOLD in "${folders[@]}"
do
	INPATH="$BASE_PATH/$FOLD"
	OUTPATH="$INPATH/supervisely"
	DATASET_NAME="UE4 Triple Chasers (top)"
	LABEL="triplecanister_top"

	# NOTE: this line is only necessary if working straight from AA's exports.
	# python lib/reorder.py --folder "$INPATH"
	python lib/convert_masks.py \
		--infolder "$INPATH" \
		--outfolder "$OUTPATH" \
		--name "$DATASET_NAME" \
		--label "$LABEL"
done
