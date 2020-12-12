import cv2
import os
import sys
import argparse
from os import listdir
from os.path import isfile, join
from PIL import Image
from enum import Enum

import supervisely_util
import convert_util


class AnnFormat(Enum):
    SUPERVISELY = 1


def crop_by_bbox(image, bbox):
    left = bbox[0][0]
    top = bbox[0][1]
    right = bbox[1][0]
    bottom = bbox[1][1]
    return image.crop((left, top, right, bottom))


def infer_anns(img_path):
    """Each non-black colour in the image is considered a separate ann.

    Iterate through all pixels in a mask produced from UE4. Each time a new
    colour is encountered (that is not black), keep track of all the pixels in
    that colour, creating the ann's bitmap. At each pixel, calculate whether or
    not the corresponding colour's bbox needs to be updated.

    Returns a dict whose keys are string representations of the RGB colour in
    the original image, and whose values are a dict with:
        bbox: [[topmost, leftmost][bottommost, rightmost]]
        bitmap: Supervisely format, { "origin": [2,3], "data": "base64..." }
    """
    baseimage = Image.open(img_path)
    rgbimage = baseimage.convert("RGB")
    npimage = cv2.imread(img_path)

    # first pass: find bbox's by separate colours
    tempAnns = {}
    for row_idx, row in enumerate(npimage):
        for col_idx, col in enumerate(row):
            if any(col):  # skip black pixels
                colour = convert_util.rgb_to_str(col)
                ref = [row_idx, col_idx]
                if colour in tempAnns:
                    # update bbox points where necessary
                    bbox = list(tempAnns[colour]["bbox"])
                    if row_idx < bbox[0][0]:
                        tempAnns[colour]["bbox"][0][0] = row_idx
                    if col_idx < bbox[0][1]:
                        tempAnns[colour]["bbox"][0][1] = col_idx
                    if row_idx > bbox[1][0]:
                        tempAnns[colour]["bbox"][1][0] = row_idx
                    if col_idx > bbox[1][1]:
                        tempAnns[colour]["bbox"][1][1] = col_idx

                else:
                    tempAnns[colour] = {"bbox": [list(ref), list(ref)]}

    # second pass: calculate bitmaps from bboxs
    anns = {}
    for (colour, v) in tempAnns.items():
        # NOTE: reverse bbox due to the way I calculate in loop above.
        reversed_bbox = list(map(lambda x: x[::-1], v["bbox"]))
        cropped = crop_by_bbox(rgbimage, reversed_bbox)
        _bitmap = {
            "origin": reversed_bbox[0],
            "data": convert_util.rgb_2_base64(cropped, convert_util.str_to_rgb(colour)),
        }

        # NOTE: bitmap is None if there is only one pixel for a colour
        # TODO: add some pixel minimum of thresholds for which to produce an annotation
        if not _bitmap["data"] is None:
            anns[colour] = {"bitmap": _bitmap, "bbox": reversed_bbox}

    width, height = rgbimage.size

    return {"size": {"height": height, "width": width}, "anns": anns}


def gen_and_save_anns(args):
    """Generate annotations from UE4 exports, saving at the specified path.

    This function is currently tightly coupled to the Supervisely format, but
    is loosely archicted with modular annotation formats in mind. Note that the
    Supervisely annotation format operates as a sort of mother format, as there
    exist open source scripts to convert to other common object detection
    dataset formats on their Github, and via their platform.
    """
    # setup
    switcher = {AnnFormat.SUPERVISELY: supervisely_util}
    # NOTE: add other ann formats if necessary by adding a 'util' module with
    # the same interface as supervisely_util.
    utils = switcher.get(AnnFormat.SUPERVISELY)
    if utils is None:
        raise Exception("You need to pass a valid annotation format.")

    OUT_ANN_PATH = f"{args.outfolder}/{args.name}/ann"
    OUT_IMG_PATH = f"{args.outfolder}/{args.name}/img"
    # make necessary folders
    if not os.path.exists(OUT_ANN_PATH):
        os.makedirs(OUT_ANN_PATH)

    # meta.json describes the datasets at the top level
    utils.save_meta(args.label, args.outfolder)

    print(args.imagefoldername)
    IN_ANN_PATH = f"{args.infolder}/{args.imagefoldername}"
    if not os.path.isdir(IN_ANN_PATH):
        raise Exception(
            f"A directory '{args.imagefoldername}' needs to exist in the directory."
        )

    ann_paths = [
        f
        for f in listdir(IN_ANN_PATH)
        if isfile(join(IN_ANN_PATH, f)) and (f.endswith(".png") or f.endswith(".jpg"))
    ]

    for ann_path in ann_paths:
        print(f"Calculating {IN_ANN_PATH}/{ann_path}...")
        obj = infer_anns(f"{IN_ANN_PATH}/{ann_path}")
        utils.save_as_ann(
            f"{OUT_ANN_PATH}/{ann_path.replace('.png', '').replace('.jpg', '')}.json",
            obj,
            args.label,
        )
        print(f"Inferred and saved annotation: {ann_path}")

    # copy original images over
    convert_util.copy_folder(f"{args.infolder}/{args.imagefoldername}", OUT_IMG_PATH)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    parser.add_argument("--infolder", help="path to input folder")
    parser.add_argument("--outfolder", help="path to output folder")
    parser.add_argument("--name", help="name for the generated dataset")
    parser.add_argument(
        "--label", help="the name of the label to attach to generated masks"
    )
    parser.add_argument(
        "--imagefoldername",
        nargs="?",
        default="X",
        help="the name of the nested folder with the actual images",
    )
    parser.add_argument(
        "--maskfoldername",
        nargs="?",
        default="Y",
        help="the name of the nested folder with the masks",
    )

    gen_and_save_anns(parser.parse_args())
