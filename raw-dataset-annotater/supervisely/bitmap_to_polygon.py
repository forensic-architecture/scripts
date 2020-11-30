import numpy as np
from skimage import measure
from shapely.geometry import Polygon

from .convert_util import base64_2_mask

from numpy.lib.stride_tricks import as_strided


def _tile_array(a, b0, b1):
    """
    Fast nearest-neighbour upsampling.

    from: https://stackoverflow.com/a/32848377/6194274
    """
    r, c = a.shape  # number of rows/columns
    rs, cs = a.strides  # row/column strides
    x = as_strided(a, (r, b0, c, b1), (rs, 0, cs, 0))  # view as large 4D array
    return x.reshape(r * b0, c * b1)  # create new 2D array


def mask_to_polygon(mask: np.ndarray, origin, tolerance: int = 3):
    """Convert a (boolean) numpy array into a polygon.

    Parameters
    ----------
    mask : np.ndarray
        The mask to polygonise
    origin : sequnce
        The xy coordinate of the top-left corner of the mask in the larger
        image
    tolerance : int, optional
        How many pixels the polygon corners can be from the true polygon
        (the default is 3)

    Returns
    -------
    list of np.ndarray
        A list of polygons, nx2, with the x and y coordinates for each point
        in the polygon.
    total_size
        The number of pixels occupied in the bitmap, so you can discard tiny
        polygons.
    """

    # upsample and pad the array
    total_size = mask.sum()
    mask = np.pad(_tile_array(mask, 2, 2), 2, "constant", constant_values=0)
    mask = mask.astype(np.uint8)
    contours = measure.find_contours(mask.astype(np.uint8), 0.9)

    contours_ = []

    for contour in contours:
        polygon = Polygon(contour).simplify(tolerance)
        y, x = polygon.exterior.xy
        # undo padding and upsampling
        y = origin[1] + (np.array(y) - 2) / 2
        x = origin[0] + (np.array(x) - 2) / 2

        contours_.append(np.stack((x, y), axis=1))

    return contours_, total_size


def bitmap_to_poly(bitmap_object: dict, tolerance: int = 3):
    """[summary]

    Parameters
    ----------
    bitmap_object : dict
        A dictionary for a bitmap, obtained from a supervisely json annotation
        file.
    tolerance : int, optional
        How many pixels the polygon corners can be from the true polygon
        (the default is 3).

    Returns
    -------
    list of np.ndarray
        A list of polygons, nx2, with the x and y coordinates for each point
        in the polygon.
    total_size
        The number of pixels occupied in the bitmap, so you can discard tiny
        polygons.
    """

    mask = base64_2_mask(bytes(bitmap_object["data"], "utf8"))
    return mask_to_polygon(mask, bitmap_object["origin"])
