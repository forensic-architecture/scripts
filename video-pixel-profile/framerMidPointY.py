"""
Prints an image with a color profile of each frame,
averaging the color of each row in the frame.
Left to right, each pixel-column corresponds to the horizontal color average of a frame.
"""

import cv2
import numpy as np
import math
from pathlib import Path

# Prompt the user for a movie file
filename = input('Enter the movie file: ')

# Use openCV to capture
mov = Path(filename)
if mov.is_file():
	cap = cv2.VideoCapture(filename)
else:
	print('Sorry, I could not open that file.')
	exit()

# Get some video properties
frameTotal = int(cap.get(cv2.CAP_PROP_FRAME_COUNT))
width  = int(cap.get(cv2.CAP_PROP_FRAME_WIDTH))
height = int(cap.get(cv2.CAP_PROP_FRAME_HEIGHT))
fps    = cap.get(cv2.CAP_PROP_FPS)

if cap.isOpened() is False:
	print("I found the file, but was unable to open it.")
else:
	print("Video opened. Number of frames: ", frameTotal, ", fps: ", fps)

# Create a 1024x1024x3 array of 8 bit unsigned integers
newImg = np.zeros( (frameTotal, 1920 ,3), dtype=np.uint8 )

# Iterate over each frame
success = True
frameCount = 0
while frameCount < frameTotal and success:
	success,image = cap.read()
	row = math.floor(height / 2)
	col = 0
	# color info is b, g, r
	while col < width:
		blue = image[row, col][0]
		green = image[row, col][1]
		red = image[row, col][2]
		newImg.itemset((frameCount, col, 0), blue)
		newImg.itemset((frameCount, col, 1), green)
		newImg.itemset((frameCount, col, 2), red)
		col += 1

	print('Processing frame %d...' % frameCount)
	frameCount += 1

# Print the thing
cv2.imwrite("profileMidPointY.jpg", newImg)
