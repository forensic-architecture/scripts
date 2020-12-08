use serde::{Deserialize, Serialize};
use serde_json::{to_string, to_writer};
use std::fs;

enum SlyAnnType {
    Bitmap,
    Bbox,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlyAnn {
    title: String,
    shape: String,
    color: String,
}

fn build_slyann(label: &String, t: SlyAnnType) -> SlyAnn {
    match t {
        SlyAnnType::Bitmap => SlyAnn {
            title: format!("{}_bitmap", label),
            shape: "bitmap".to_string(),
            color: "#ae5311".to_string(),
        },
        SlyAnnType::Bbox => SlyAnn {
            title: format!("{}_bbox", label),
            shape: "rectangle".to_string(),
            color: "8faa12".to_string(),
        },
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SlyMeta {
    classes: Vec<SlyAnn>,
    tags_images: Vec<String>,
    tags_objects: Vec<String>,
}

pub fn sly_create_meta(label: String, output_dir: String) -> () {
    let meta_p = format!("{}/meta.json", output_dir);
    let meta = SlyMeta {
        classes: vec![
            build_slyann(&label, SlyAnnType::Bitmap),
            build_slyann(&label, SlyAnnType::Bbox),
        ],
        tags_images: vec![],
        tags_objects: vec![],
    };
    fs::write(meta_p, to_string(&meta).unwrap()).expect("Couldn't write Supervisely meta.");
}
