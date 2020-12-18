import os
import io
import base64
import zlib
import shutil
import numpy as np
from PIL import Image

GREYSCALE_MODE = "L"

def copy_folder(from_path, to_path):
    if os.path.exists(to_path):
        shutil.rmtree(to_path)
    shutil.copytree(from_path, to_path)


def save_pil_image(img, path):
    img.convert("RGB").save(path)


def save_pil_crops(img_path, anns):
    image = Image.open(img_path)
    # image = image.convert("P")

    for idx, (k, v) in enumerate(anns.items()):
        cropped = crop_by_bbox(image, v["bbox"])
        img_name = f"crop_{idx}.jpg"
        save_pil_image(cropped, img_name)
        print(f"Saved {img_name}.")


def base64_2_mask(s):
    """Convert from base64 annotation format to an image"""
    z = zlib.decompress(base64.b64decode(s))
    n = np.fromstring(z, np.uint8)
    mask = cv2.imdecode(n, cv2.IMREAD_UNCHANGED)[:, :, 3].astype(bool)
    return mask


def mask_2_base64(mask):
    """Convert from semseg image to a base64 annotation"""
    img_pil = Image.fromarray(np.array(mask, dtype=np.uint8))
    img_pil.putpalette([0, 0, 0, 255, 255, 255])
    bytes_io = io.BytesIO()
    img_pil.save(bytes_io, format="PNG", transparency=0, optimize=0)
    bytes = bytes_io.getvalue()
    return base64.b64encode(zlib.compress(bytes)).decode("utf-8")


def rgb_2_base64(rgbimage, colour):
    data = np.array(rgbimage)

    # corner case for empty anns
    if len(data.shape) is not 3:
        return None

    b1, g1, r1 = colour
    red, green, blue = data[:,:,0], data[:,:,1], data[:,:,2]
    mask = (red == r1) & (green == g1) & (blue == b1)
    data[:,:,:3][~mask] = [0,0,0]

    im2 = Image.fromarray(data).convert(GREYSCALE_MODE)

    return greyscale_2_base64(im2)


def greyscale_2_base64(greyscale):
    """Convert from greyscale mask to a base64 annotation"""
    greyscale.putpalette([0, 0, 0, 255, 255, 255])
    bytes_io = io.BytesIO()
    greyscale.save(bytes_io, format="PNG", transparency=0, optimize=0)
    bytes = bytes_io.getvalue()
    return base64.b64encode(zlib.compress(bytes)).decode("utf-8")


def base64_from_grayscale(path):
    image = Image.open(path)
    greyscale = image.convert("L")  #  make grayscale
    return greyscale_2_base64(greyscale)


def rgb_to_str(arr):
    return f"{arr[0]},{arr[1]},{arr[2]}"


def str_to_rgb(_str):
    rgb = [x.strip() for x in _str.split(",")]
    return [int(x) for x in rgb]
