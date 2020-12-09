extern crate image;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

enum AnnType {
    Bitmap,
    Bbox,
}

type Pixel = image::Rgba<u8>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Anns {
    size: ImSize,
    anns: HashMap<String, Ann>,
}

impl Anns {
    fn new(h: u32, w: u32) -> Anns {
        Anns {
            size: ImSize {
                height: h,
                width: w,
            },
            anns: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct BitmapAnn {
    origin: [u32; 2],
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ann {
    bbox: [[u32; 2]; 2],
    bitmap: Option<BitmapAnn>,
}

impl AnnType {
    fn new_meta(&self, label: &String) -> AnnMeta {
        match self {
            AnnType::Bitmap => AnnMeta {
                title: format!("{}_bitmap", label),
                shape: "bitmap".to_string(),
                color: "#ae5311".to_string(),
            },
            AnnType::Bbox => AnnMeta {
                title: format!("{}_bbox", label),
                shape: "rectangle".to_string(),
                color: "8faa12".to_string(),
            },
        }
    }

    fn new_ann(&self, path: &Path) -> Result<Anns, String> {
        let black_pixel = image::Rgba([0 as u8, 0, 0, 255]);
        let img = image::open(path).unwrap();
        let (h, w) = img.dimensions();
        let mut store = Anns::new(h, w);
        for (x, y, pixel) in img.pixels() {
            if pixel == black_pixel {
                continue;
            };
            let colour = pixel.to_str();
            if let Entry::Occupied(cur_bbox) = store.anns.entry(colour.clone()) {
                let ann = cur_bbox.into_mut();
                println!("{:?}", ann.bbox);
            } else {
                store.anns.insert(
                    colour,
                    Ann {
                        bbox: [[x, y], [x, y]],
                        bitmap: None,
                    },
                );
            }

            println!("{:?}", pixel.to_str());
            // println!("{}, {}", x, y);
        }

        Ok(Anns::new(h, w))
    }
}

trait PixelMethods {
    fn to_str(&self) -> String;
}

impl PixelMethods for Pixel {
    fn to_str(&self) -> String {
        format!("{},{},{}", self[0], self[1], self[2])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnMeta {
    title: String,
    shape: String,
    color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImSize {
    height: u32,
    width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlyMeta {
    classes: Vec<AnnMeta>,
    tags_images: Vec<String>,
    tags_objects: Vec<String>,
}

pub fn sly_create_meta(label: String, output_dir: String) -> () {
    let meta_p = format!("{}/meta.json", output_dir);
    let meta = SlyMeta {
        classes: vec![
            AnnType::Bitmap.new_meta(&label),
            AnnType::Bbox.new_meta(&label),
        ],
        tags_images: vec![],
        tags_objects: vec![],
    };
    fs::write(meta_p, to_string(&meta).unwrap()).expect("Couldn't write Supervisely meta.");
}

// `msk_p` is a path to a mask generated from a game engine. Each non-black colour in the image is
// taken to be a distinct annotation.
//
// For the time being, the assumption is that a `msk_p` only contains one label, which pertains to
// all annotations in the mask.
pub fn infer_anns(msk_p: &Path) -> Anns {
    println!("TODO: {:?}", msk_p);
    AnnType::Bbox.new_ann(msk_p).unwrap()
}
