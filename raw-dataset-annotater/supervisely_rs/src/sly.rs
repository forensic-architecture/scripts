extern crate image;
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

// ---- UTIL -------
type Pos = [u32; 2];
type Mask = Vec<Pos>;
type Bbox = [Pos; 2];
type Pixel = image::Rgba<u8>;

trait PixelMethods {
    fn to_str(&self) -> String;
}

impl PixelMethods for Pixel {
    fn to_str(&self) -> String {
        format!("{},{},{}", self[0], self[1], self[2])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ImSize {
    height: u32,
    width: u32,
}

fn too_tiny(pxls: &Mask, size: &ImSize) -> bool {
    let msk = pxls.len() as f64;
    let img = (size.width * size.height) as f64;
    (msk / img) < 0.002
}

// TODO: actually infer the mask
fn mask_as_b64(mask: &Mask, origin: &Pos) -> String {
    "TODO".to_string()
}

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

struct AnnRepr {
    pixels: Mask,
    tr_pt: Pos,
    bl_pt: Pos,
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
        let black_pixel = image::Rgba([0 as u8, 0, 0, 255]);
        let img = image::open(path).unwrap();
        let (h, w) = img.dimensions();
        let mut anns: HashMap<String, AnnRepr> = HashMap::new();
        for (x, y, pixel) in img.pixels() {
            if pixel == black_pixel {
                continue;
            };
            let colour = pixel.to_str();
            if let Entry::Occupied(ann_colour) = anns.entry(colour.clone()) {
                let ann = ann_colour.into_mut();

                ann.pixels.push([x, y]);

                // update tr_pt
                if x < ann.tr_pt[0] {
                    ann.tr_pt[0] = x;
                }
                if y < ann.tr_pt[1] {
                    ann.tr_pt[1] = y;
                }

                // update bl_pt
                if x > ann.bl_pt[0] {
                    ann.bl_pt[0] = x;
                }
                if y > ann.bl_pt[1] {
                    ann.bl_pt[1] = y;
                }
            } else {
                anns.insert(
                    colour,
                    AnnRepr {
                        pixels: vec![[x, y]],
                        tr_pt: [x, y],
                        bl_pt: [x, y],
                    },
                );
            }
        }

        let size = ImSize {
            height: h,
            width: w,
        };

        let mut actual_anns: HashMap<String, Ann> = HashMap::new();
        for (colour, ann) in anns {
            if too_tiny(&ann.pixels, &size) {
                continue;
            }
            actual_anns.insert(
                colour,
                Ann {
                    bbox: [ann.tr_pt, ann.bl_pt],
                    bitmap: Some(Bitmap {
                        origin: ann.tr_pt,
                        data: mask_as_b64(&ann.pixels, &ann.tr_pt),
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
