# src

This repository contains scripts to run necessary transforms and
post-processing for the data produced from UE4 in preparation for training ML
classifiers.

## setup
From a bash or zsh shell (no fish support) inside this folder ('src'), run:
```
source setup.sh
```
This command will create a Docker image with the necessary dependencies
installed, and spin up a bash shell inside a Docker container created from 
that image. The Docker image will not rebuild if it already exists in your 
local image list.

### UE4 to Supervisely Workflow
Annotations are exported from UE4 simply as an image that consists of another
render pass, where pixels that include an object of interested are a non-black
colour. Each separate object is coded with a different colour. There is
currently no semantic to encode multiple different objects of interest in an
annotation that comes from UE4.

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
variables that are templated in `run.sh` for input and output paths
appropriately. Then, inside the docker container (entered via the command
above), run: 
```
sh run.sh
```

This will convert the masks that UE4 produces to [Supervisely's annotation
format](https://docs.supervise.ly/import/local_files/supervisely/). See the
variables that are templated inside `run.sh` and adjust accordingly for
different input and output paths, dataset names, and label names.
