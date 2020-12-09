use serde::{Deserialize, Serialize};
use serde_json::{to_string, to_writer};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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

    fn new_ann(&self) -> Ann {
        Ann {}
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
    height: i32,
    width: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ann {
    // size: ImSize,
// anns: HashMap<String, String>,
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

// `msk_p` is a path to a mask generated from a game engine, which is an image containing black
// pixels where there is not an annotation, and colored pixels where there are annotations.
//
// For the time being, the assumption is that a `msk_p` only contains one label, which pertains to
// all annotations in the mask.
pub fn infer_sly_anns(msk_p: &Path) -> Ann {
    println!("TODO: {:?}", msk_p);
    AnnType::Bbox.new_ann()
}
