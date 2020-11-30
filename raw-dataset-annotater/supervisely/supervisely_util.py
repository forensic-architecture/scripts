import json


def format_supervisely_bitmap(ann, classTitle):
    return {
        "description": "",
        "bitmap": ann["bitmap"],
        "tags": [],
        "classTitle": f"{classTitle}_bitmap",
        "points": {"exterior": [], "interior": []},
    }


def format_supervisely_bbox(ann, classTitle):
    return {
        "description": "",
        "bitmap": None,
        "tags": [],
        "classTitle": f"{classTitle}_bbox",
        "points": {"exterior": ann["bbox"], "interior": []},
    }


def format_ann(obj, classTitle):
    all_anns = obj["anns"].items()
    return {
        "tags": ["train"],
        "description": "",
        "objects": [format_supervisely_bitmap(ann, classTitle) for _, ann in all_anns]
        + [format_supervisely_bbox(ann, classTitle) for _, ann in all_anns],
        "size": obj["size"],
    }


def save_meta(title, outpath):
    with open(f"{outpath}/meta.json", "w") as outfile:
        json.dump(
            {
                "classes": [
                    {"title": f"{title}_bitmap", "shape": "bitmap", "color": "#ae5311"},
                    {
                        "title": f"{title}_bbox",
                        "shape": "rectangle",
                        "color": "#8faa12",
                    },
                ],
                "tags_images": ["train"],
                "tags_objects": [],
            },
            outfile,
            indent=2,
        )


def save_as_ann(json_path, obj, classTitle):
    super_obj = format_ann(obj, classTitle)

    with open(json_path, "w") as outfile:
        json.dump(super_obj, outfile, indent=2)
