use crate::errors::Error;
use crate::util::*;
use image::{GenericImageView, Rgba};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

pub trait AnnotatedDataset {
    // fn new(label: &String, dataset_root: &Path) -> Self;
    fn write_item(&self, anns: &Anns, mask: &Path) -> Result<(), Error>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bitmap {
    pub origin: Pos,
    pub data: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ann {
    pub bbox: Bbox,
    pub bitmap: Option<Bitmap>,
}

pub struct Anns {
    pub size: ImSize,
    pub anns: HashMap<String, Ann>,
}

impl Anns {
    // `msk_p` is a path to a mask generated from a game engine. Each non-black colour in the image is
    // taken to be a distinct annotation.
    //
    // For the time being, the assumption is that a `msk_p` only contains one label, which pertains to
    // all annotations in the mask.
    pub fn new(path: &Path) -> Anns {
        let black_pixel = Rgba([0 as u8, 0, 0, 255]);
        let mut img = image::open(path).unwrap();
        let (w, h) = img.dimensions();
        let mut anns: HashMap<String, Bbox> = HashMap::new();
        for (x, y, pixel) in img.pixels() {
            if pixel == black_pixel {
                continue;
            };
            let colour = pixel.to_str();
            match anns.get_mut(&colour) {
                Some(ann) => {
                    // update top right point
                    if x < ann[0][0] {
                        ann[0][0] = x;
                    }
                    if y < ann[0][1] {
                        ann[0][1] = y;
                    }

                    // update bottom left point
                    if x > ann[1][0] {
                        ann[1][0] = x;
                    }
                    if y > ann[1][1] {
                        ann[1][1] = y;
                    }
                }
                None => {
                    anns.insert(colour, [[x, y], [x, y]]);
                }
            }
        }

        let size = ImSize {
            height: h,
            width: w,
        };

        let mut actual_anns: HashMap<String, Ann> = HashMap::new();
        for (colour, ann) in anns {
            if too_tiny(&ann, &size) {
                continue;
            }
            actual_anns.insert(
                colour,
                Ann {
                    bbox: [ann[0], ann[1]],
                    bitmap: Some(Bitmap {
                        origin: ann[0],
                        data: crop_black_and_white_b64(&mut img, ann),
                    }),
                },
            );
        }

        Anns {
            size,
            anns: actual_anns,
        }
    }
}
