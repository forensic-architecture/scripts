import argparse
from os import listdir, mkdir
import shutil
from os.path import isfile, join, normpath, basename

def reorder(fname):
    files = [f for f in listdir(fname) if isfile(join(fname, f))]
    folder_name = basename(normpath(fname))
    img_folder = join(fname, 'img')
    raw_ann_folder = join(fname, 'raw_ann')

    mkdir(img_folder)
    mkdir(raw_ann_folder)

    for f in files:
        if "CustomStencil" in f:
            shutil.move(join(fname, f), join(raw_ann_folder, f.replace("{}CustomStencil.".format(folder_name), "")))
        elif "FinalImage" in f:
            shutil.move(join(fname, f), join(img_folder, f.replace("{}FinalImage.".format(folder_name), "")))


if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    parser.add_argument("--folder", help="folder to reorder in prep for convert_masks.py")

    args = parser.parse_args()

    reorder(args.folder)
