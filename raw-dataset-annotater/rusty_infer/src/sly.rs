extern crate image;
use crate::anns::{Anns, Bitmap};
use crate::util::*;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fs;
use std::path::Path;
use std::rc::Rc;

// ---- META -------
#[derive(Serialize, Deserialize, Debug)]
pub struct SlyMeta {
    classes: Vec<SlyAnnMetaInner>,
    tags_images: Vec<String>,
    tags_objects: Vec<String>,
}

enum SlyAnnMeta {
    Bitmap,
    Bbox,
}

impl SlyAnnMeta {
    fn new(&self, label: &String) -> SlyAnnMetaInner {
        match self {
            SlyAnnMeta::Bitmap => SlyAnnMetaInner {
                title: format!("{}_bitmap", label),
                shape: "bitmap".to_string(),
                color: "#ae5311".to_string(),
            },
            SlyAnnMeta::Bbox => SlyAnnMetaInner {
                title: format!("{}_bbox", label),
                shape: "rectangle".to_string(),
                color: "8faa12".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlyAnnMetaInner {
    title: String,
    shape: String,
    color: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SlyAnns {
    tags: Vec<String>,
    description: String,
    objects: Vec<SlyAnn>,
    size: ImSize,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct SlyAnn {
    description: String,
    bitmap: Option<Bitmap>,
    tags: Vec<String>,
    classTitle: String,
    points: SlyPoints,
}

impl SlyAnn {
    fn new(bitmap: Option<Bitmap>, bbox: Option<Bbox>, label: String) -> SlyAnn {
        let suffix = match bitmap {
            None => String::from("bitmap"),
            _ => String::from("bbox"),
        };

        let exterior = match bbox {
            Some(b) => b.to_vec(),
            None => vec![],
        };

        SlyAnn {
            description: String::from(""),
            bitmap,
            tags: vec![],
            classTitle: format!("{}_{}", label, suffix),
            points: SlyPoints {
                exterior,
                interior: [],
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SlyPoints {
    exterior: Vec<Pos>,
    interior: [u32; 0],
}

// ------ CREATE FUNCS ---------
pub fn create_meta(label: String, output_dir: String) -> () {
    let meta_p = format!("{}/meta.json", output_dir);
    let meta = SlyMeta {
        classes: vec![SlyAnnMeta::Bitmap.new(&label), SlyAnnMeta::Bbox.new(&label)],
        tags_images: vec![],
        tags_objects: vec![],
    };
    fs::write(meta_p, to_string_pretty(&meta).unwrap()).expect("Couldn't write Supervisely meta.");
}

pub fn create_ann(in_p: &Path, out_p: &Path, label: String) -> () {
    let anns = Anns::new(&in_p);
    let anns = &anns;
    println!("Writing to {}", out_p.to_str().unwrap());

    let sly_anns = SlyAnns {
        tags: vec![String::from("train")],
        description: String::from(""),
        objects: to_slyanns(&anns, label),
        size: anns.size.clone(),
    };

    fs::write(out_p, to_string_pretty(&sly_anns).unwrap())
        .expect("Couldn't write Supervisely anns.");
}

fn to_slyanns(anns: &Anns, label: String) -> Vec<SlyAnn> {
    let mut out = vec![];
    let label = Rc::new(label);
    for (_, ann) in &anns.anns {
        out.push(SlyAnn::new(ann.bitmap.clone(), None, (&label).to_string()));
        out.push(SlyAnn::new(None, Some(ann.bbox), (&label).to_string()));
    }
    out
}
