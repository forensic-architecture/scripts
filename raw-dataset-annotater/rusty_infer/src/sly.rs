use crate::anns::{Anns, Bitmap};
use crate::errors::Error;
use crate::util::*;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fs;
use std::path::Path;

// ---- META -------
#[derive(Serialize, Deserialize, Debug)]
pub struct SlyMeta {
    classes: Vec<SlyAnnMetaInner>,
    tags_images: Vec<String>,
    tags_objects: Vec<String>,
}

impl From<&String> for SlyMeta {
    fn from(label: &String) -> SlyMeta {
        SlyMeta {
            classes: vec![SlyAnnMeta::Bitmap.new(label), SlyAnnMeta::Bbox.new(label)],
            tags_images: vec![],
            tags_objects: vec![],
        }
    }
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
                color: "#8faa12".to_string(),
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
    fn new(bitmap: Option<Bitmap>, bbox: Option<Bbox>, label: &String) -> SlyAnn {
        let suffix = match bitmap {
            None => "bbox",
            _ => "bitmap",
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

    fn vec_from_anns(anns: &Anns, label: &String) -> Vec<SlyAnn> {
        let mut out = vec![];
        for (_, ann) in &anns.anns {
            out.push(SlyAnn::new(ann.bitmap.clone(), None, &label));
            out.push(SlyAnn::new(None, Some(ann.bbox), &label));
        }
        out
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SlyPoints {
    exterior: Vec<Pos>,
    interior: [u32; 0],
}

pub struct SlyDataset {
    pub label: String,
    pub dataset_root: String,
}

impl SlyDataset {
    pub fn new(label: &String, dataset_root: &Path) -> Self {
        let label = label.clone();
        let dataset_root = String::from(dataset_root.to_str().unwrap());

        let meta_p = format!("{}/meta.json", &dataset_root);
        let meta = SlyMeta::from(&label);
        fs::write(meta_p, to_string_pretty(&meta).unwrap())
            .expect("Couldn't write Supervisely meta.");

        SlyDataset {
            label,
            dataset_root,
        }
    }

    pub fn write_item(&self, anns: &Anns, dest: &Path) -> Result<(), Error> {
        let sly_anns = SlyAnns {
            tags: vec![String::from("train")],
            description: String::from(""),
            objects: SlyAnn::vec_from_anns(&anns, &self.label),
            size: anns.size.clone(),
        };

        let sly_anns = to_string_pretty(&sly_anns)?;
        fs::write(dest, &sly_anns)?;

        Ok(())
    }
}
