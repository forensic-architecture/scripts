# src

This repository contains scripts to run necessary transforms and
post-processing for the data produced from UE4 in preparation for training ML
classifiers.

## setup
From a bash or zsh shell inside this folder ('src'), run:
```
source setup.sh
```
This command will create a Docker image with the necessary dependencies
installed, and spin up a bash shell inside a Docker container created from 
that image. The Docker image will not rebuild if it already exists in your 
local image list.

### From 'raw' to annotated dataset
Annotations are exported 'raw' from a game engine simply as an image that
consists of another render pass, where pixels that include an object of
interested are a non-black colour. Each separate object is coded with
a different colour.  There is currently no semantic to encode multiple
different objects of interest in a raw annotation.

Ensure that the exported images are organised with the following folder
structure:

.
+-- img
|   +-- img1.png
|   +-- img2.png
+-- raw_ann
|   +-- img1.png
|   +-- img2.png


The original image should be in the 'img' folder, and the mask should be a file
of the same name in the 'raw_ann' folder.

Once you have confirmed that the folder is organised correctly, adjust the
variables that are templated in `run_annotater`. In particular, make sure that
the `DATASET_VOLUMES` is appropriate to where datasets are stored on your local
disk.

Once this is in order, run:
```
./run_annotater
```

This will build a Docker container with all the required dependencies and run
the conversion scripts. Note that once the container is built, it will prompt
you for the folder name where your images are, and the label you want to give.

The scripts are currently hardcoded to produce [Supervisely's annotation format](https://docs.supervise.ly/import/local_files/supervisely/), but the structure is set up to provide alternative formats in the future.
