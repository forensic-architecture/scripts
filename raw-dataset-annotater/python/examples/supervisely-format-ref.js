/**
 * See https://docs.supervise.ly/ann_format/ for more detail.
 */

const superviselyExampleJsonStructure = {
  "tags": [
    // if the data is test, it should be tagged as such here
    "train"
  ],
  "description": "",
  "objects": [
    {
      "description": "",
      // if the annotation is a bounding box, this object should be 'null'
      "bitmap": {
        // x and y coordinates at the left top corner of the bitmap.
        // NOTE: if the bitmap spans the entire image, is the origin just always (0,0)?
        "origin": [ 0, 0 ],
        "data": "the_base64_mask_annotation_extended_from_origin_goes_here"
      },
      // annotation-specific tags go here
      "tags": [],
      "classTitle": "triple-top-bitmap",
      "points": {
        "interior": [],
        // if the ann is a bounding box, this is a 2d array with corner points
        "exterior": []
      }
    }
  ]
}
