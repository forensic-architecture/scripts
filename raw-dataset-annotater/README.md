# Raw Dataset Annotater
These are scripts designed to take sets of images rendered from game engines
(one of which represents the source image, and the other the annotations on
that source image of objects of interest)-- and produce annotated datasets in
various formats for ML training.

There are two versions of these scripts: one in Python, and the other in Rust.

## Python
Works in a finicky way. You can convert from the raw data to [Supervisely](https://docs.supervise.ly/data-organization/00_ann_format_navi), and then with a separate set of scripts convert from Supervisely to the [Pascal VOC format](https://towardsdatascience.com/coco-data-format-for-object-detection-a4c5eaf518c5).

## Rust
Currently works minimally to produce annotated datasets with just Supervisely
bounding boxes (no bitmap annotations yet). These can then be further converted
to Pascal VOC with the Python scripts.

Running details for each of the scripts are in the respective folders. 
