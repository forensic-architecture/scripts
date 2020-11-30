import argparse
import re
import shutil
from os import listdir, mkdir, rmdir
from os.path import isfile, join, normpath, basename

def reorder_unreal(fname):
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


def reorder_unity(fname):
    bl = ['_READ-ME.txt', 'config.json']
    imgs = [f for f in listdir(fname) if isfile(join(fname, f)) and f not in bl]
    masks_dir = join(fname, '_masks')
    masks = listdir(masks_dir)

    img_folder = join(fname, 'img')
    raw_ann_folder = join(fname, 'raw_ann')
    try:
        mkdir(img_folder)
        mkdir(raw_ann_folder)
    except:
        pass

    for f in imgs:
        shutil.move(join(fname, f), join(img_folder, f))

    for f in listdir(join(fname, '_masks')):
        newname = re.search(r'(.+)\_mask', f).group(1)
        shutil.move(join(masks_dir, f), f"{join(raw_ann_folder, newname)}.jpg")

    try:
        rmdir(masks_dir)
    except:
        pass


if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    parser.add_argument("--folder", help="folder to reorder in prep for convert_masks.py")
    parser.add_argument("--format", help="format to convert from: either 'unity' or 'unreal'")

    args = parser.parse_args()

    if args.format == 'unity':
        reorder_unity(args.folder)
    elif args.format == 'unreal':
        reorder_unreal(args.folder)


