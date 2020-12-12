This repo includes script to get a dataset labelled with supervise.ly into a Pascal VOC format, as well as the format for training YOLO based detectors.

## Installation
pip install pascal-voc-writer

## Commands
Assuming your dataset in supervise.ly format is in this repo, change the directories and other variables in `config.py`. 

If your dataset is not in this repo, make sure to change the path to absolute paths in `config.py`, mainly `json_path_pattern` and `img_patterns`.

Run `python rename_file.py` to clean image and annotation jsons file paths into more consistent names.

Run `python supervisely_to_pascal_voc.py` to get the dataset in PASCAL VOC format.