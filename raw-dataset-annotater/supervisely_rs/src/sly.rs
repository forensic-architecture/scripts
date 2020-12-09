extern crate image;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    classes: Vec<AnnMeta>,
    tags_images: Vec<String>,
    tags_objects: Vec<String>,
}

pub fn create_meta(label: String, output_dir: String) -> () {
    let meta_p = format!("{}/meta.json", output_dir);
    let meta = Meta {
        classes: vec![
            AnnType::Bitmap.new_meta(&label),
            AnnType::Bbox.new_meta(&label),
        ],
        tags_images: vec![],
        tags_objects: vec![],
    };
    fs::write(meta_p, to_string(&meta).unwrap()).expect("Couldn't write Supervisely meta.");
}

pub fn create_ann(in_p: &Path, out_p: &Path) -> () {
    // println!("inpath: {}", in_p.to_str().unwrap());
    // println!("outpath: {}", out_p.to_str().unwrap());
    let _ = Anns::new(in_p);
    ()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnMeta {
    title: String,
    shape: String,
    color: String,
}

enum AnnType {
    Bitmap,
    Bbox,
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
}

type Pixel = image::Rgba<u8>;

trait PixelMethods {
    fn to_str(&self) -> String;
}

impl PixelMethods for Pixel {
    fn to_str(&self) -> String {
        format!("{},{},{}", self[0], self[1], self[2])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImSize {
    height: u32,
    width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Anns {
    size: ImSize,
    anns: HashMap<String, Ann>,
}

impl Anns {
    // `msk_p` is a path to a mask generated from a game engine. Each non-black colour in the image is
    // taken to be a distinct annotation.
    //
    // For the time being, the assumption is that a `msk_p` only contains one label, which pertains to
    // all annotations in the mask.
    fn new(path: &Path) -> Anns {
        let black_pixel = image::Rgba([0 as u8, 0, 0, 255]);
        let img = image::open(path).unwrap();
        let (h, w) = img.dimensions();
        let mut anns: HashMap<String, Vec<[u32; 2]>> = HashMap::new();
        for (x, y, pixel) in img.pixels() {
            if pixel == black_pixel {
                continue;
            };
            let colour = pixel.to_str();
            if let Entry::Occupied(ann_colour) = anns.entry(colour.clone()) {
                let ann = ann_colour.into_mut();
                ann.push([x, y]);
            } else {
                anns.insert(colour, vec![[x, y]]);
            }
        }

        let actual_anns: HashMap<String, Ann> = HashMap::new();
        for (col, pxls) in anns {}

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
