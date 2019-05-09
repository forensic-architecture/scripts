"""
Prints an image with a color profile of each frame,
averaging the color of each row in the frame.
Left to right, each pixel-column corresponds to the horizontal color average of a frame.
"""
import os
import cv2
import numpy as np
import math
from pathlib import Path

in_folder = "Temp_In"
out_folder = "Temp_Out"
files = [f for f in os.listdir(in_folder) if os.path.isfile(os.path.join(in_folder, f))]


def output_file(orig_filename):
    return f"{out_folder}/profile_sampleX_{filename}.jpg"


for filename in files:
    # use openCV to capture
    mov = Path(os.path.join(in_folder, filename))
    if mov.is_file():
        cap = cv2.VideoCapture(os.path.join(in_folder, filename))
        print("Working on file %s" % filename)
    else:
        print("Sorry, I could not open that file.")
        exit()

    # get some video properties
    frameTotal = int(cap.get(cv2.CAP_PROP_FRAME_COUNT))
    width = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
    height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))
    fps = cap.get(cv2.CAP_PROP_FPS)

    if cap.isOpened() is False:
        print("I found the file, but was unable to open it.")
        continue
    else:
        print("Video opened. Number of frames: ", frameTotal, ", fps: ", fps)

    # create a {height}x{frameTotal}x3 tensor of 8 bit unsigned integers
    newImg = np.zeros((height, frameTotal, 3), dtype=np.uint8)
    success = True
    frameCount = 0
    col = math.floor(width / 2)

    print(f"filename: {filename}")
    print(f"frameTotal: {frameTotal}")
    print(f"width: {width}")
    print(f"height: {height}")
    print(f"fps: {fps}")
    print(f"sampling column: {col}")

    print("... ...processing... ...")
    while frameCount < frameTotal and success:
        row = 0
        success, image = cap.read()

        # transplant the center column of pixels into the newImage
        while row < height and success:
            blue = image[row, col][0]
            green = image[row, col][1]
            red = image[row, col][2]
            newImg.itemset((row, frameCount, 0), blue)
            newImg.itemset((row, frameCount, 1), green)
            newImg.itemset((row, frameCount, 2), red)
            row += 1

        frameCount += 1

        # Print the thing
    cv2.imwrite(output_file(filename), newImg)
    print(f"{output_file(filename)} created.")
    print("---------------------------------")
