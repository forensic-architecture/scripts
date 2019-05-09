# Video Pixel Profile

![Pixel Profile Example](/Temp_Out/pixel-profile-example.png)

These scripts index an MP4 video to produce an image that visually expresses
what the video contains by sampling some pixels from each frame, and 
a 'timeline' of the video. The following heuristics are implemented:

| Title | Description |
| ----- | ----------- |
| MidPointX | Sample a one-pixel-wide column in the center of each frame (the X midpoint), stacking the sample from each frame horizontally in the output JPG |
| MidPointY | Sample a one-pixel-high row in the center of each frame ( the Y midpoint), stacking the sample from each frame vertically in the output JPG |


### Running
* Install the dependencies in your local version of python 3:

    `python3 -m pip install -r requirements.txt`

* Copy the videos you want to process in the ‘Temp_In’ folder.
* On the terminal, run the following script:

    `python3 framerMidPointX.py`

After the script has finished running, the resulting JPEGs can be found in
'Temp_Out'.

#### Running on Windows
Note that on Windows, the executable for the Python interpreter is usually
downloaded as `py.exe`, and so you will need to use that to run commands
instead of `python3`:
    `py.exe framerMidPointX.py`
