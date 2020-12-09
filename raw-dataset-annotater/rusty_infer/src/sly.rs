extern crate image;
use crate::util::*;
use image::{GenericImageView, Rgba};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

// ---- META -------
#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    classes: Vec<AnnMetaInner>,
    tags_images: Vec<String>,
    tags_objects: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnnMetaInner {
    title: String,
    shape: String,
    color: String,
}

enum AnnMeta {
    Bitmap,
    Bbox,
}

impl AnnMeta {
    fn new(&self, label: &String) -> AnnMetaInner {
        match self {
            AnnMeta::Bitmap => AnnMetaInner {
                title: format!("{}_bitmap", label),
                shape: "bitmap".to_string(),
                color: "#ae5311".to_string(),
            },
            AnnMeta::Bbox => AnnMetaInner {
                title: format!("{}_bbox", label),
                shape: "rectangle".to_string(),
                color: "8faa12".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Anns {
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
        let black_pixel = Rgba([0 as u8, 0, 0, 255]);
        let mut img = image::open(path).unwrap();
        let (h, w) = img.dimensions();
        let mut anns: HashMap<String, Bbox> = HashMap::new();
        for (x, y, pixel) in img.pixels() {
            if pixel == black_pixel {
                continue;
            };
            let colour = pixel.to_str();
            if let Entry::Occupied(ann_colour) = anns.entry(colour.clone()) {
                let ann = ann_colour.into_mut();

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
            } else {
                anns.insert(colour, [[x, y], [x, y]]);
            }
        }

        let size = ImSize {
            height: h,
            width: w,
        };

        let mut actual_anns: HashMap<String, Ann> = HashMap::new();
        for (colour, ann) in anns {
            if too_tiny(ann, &size) {
                continue;
            }
            actual_anns.insert(
                colour,
                Ann {
                    bbox: [ann[0], ann[1]],
                    bitmap: Some(Bitmap {
                        origin: ann[0],
                        data: crop_b64(&mut img, ann),
                    }),
                },
            );
        }

        Anns {
            size,
            anns: actual_anns,
        }
    }

    fn to_fullanns(&self, label: String) -> Vec<FullAnn> {
        let mut out = vec![];
        let label = Rc::new(label);
        for (_, ann) in &self.anns {
            out.push(FullAnn::new(ann.bitmap.clone(), None, (&label).to_string()));
            out.push(FullAnn::new(None, Some(ann.bbox), (&label).to_string()));
        }
        out
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Bitmap {
    origin: Pos,
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Ann {
    bbox: Bbox,
    bitmap: Option<Bitmap>,
}

// ------ OUTER SLY ---------
#[derive(Serialize, Deserialize, Debug)]
struct Points {
    exterior: Vec<Pos>,
    interior: [u32; 0],
}

#[derive(Serialize, Deserialize, Debug)]
struct FullAnn {
    description: String,
    bitmap: Option<Bitmap>,
    tags: Vec<String>,
    classTitle: String,
    points: Points,
}

impl FullAnn {
    fn new(bitmap: Option<Bitmap>, bbox: Option<Bbox>, label: String) -> FullAnn {
        let suffix = match bitmap {
            None => String::from("bitmap"),
            _ => String::from("bbox"),
        };

        let exterior = match bbox {
            Some(b) => b.to_vec(),
            None => vec![],
        };

        FullAnn {
            description: String::from(""),
            bitmap,
            tags: vec![],
            classTitle: format!("{}_{}", label, suffix),
            points: Points {
                exterior,
                interior: [],
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FullAnns {
    tags: Vec<String>,
    description: String,
    objects: Vec<FullAnn>,
    size: ImSize,
}

// ------ CREATE FUNCS ---------
pub fn create_meta(label: String, output_dir: String) -> () {
    let meta_p = format!("{}/meta.json", output_dir);
    let meta = Meta {
        classes: vec![AnnMeta::Bitmap.new(&label), AnnMeta::Bbox.new(&label)],
        tags_images: vec![],
        tags_objects: vec![],
    };
    fs::write(meta_p, to_string_pretty(&meta).unwrap()).expect("Couldn't write Supervisely meta.");
}

pub fn create_ann(in_p: &Path, out_p: &Path, label: String) -> () {
    let anns = Anns::new(&in_p);
    let anns = &anns;
    println!("Writing to {}", out_p.to_str().unwrap());

    let full_anns = FullAnns {
        tags: vec![String::from("train")],
        description: String::from(""),
        objects: anns.to_fullanns(label),
        size: anns.size.clone(),
    };

    fs::write(out_p, to_string_pretty(&full_anns).unwrap())
        .expect("Couldn't write Supervisely anns.");
}
