use crate::anns::{Anns, Bitmap};
use crate::errors::Error;
use crate::util::*;
use serde::Serialize;
use serde_json::to_string_pretty;
use std::fs;
use std::path::Path;

// ---- META -------
#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
pub struct SlyMeta {
    classes: Vec<SlyAnnMetaInner>,
    tags: Vec<String>,
    projectType: String,
}

impl From<&String> for SlyMeta {
    fn from(label: &String) -> SlyMeta {
        SlyMeta {
            classes: vec![SlyAnnMeta::Bitmap.new(label), SlyAnnMeta::Bbox.new(label)],
            tags: vec![],
            projectType: "images".to_string(),
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

#[derive(Serialize, Debug)]
pub struct SlyAnnMetaInner {
    title: String,
    shape: String,
    color: String,
}

#[derive(Serialize)]
struct SlyAnns {
    tags: Vec<String>,
    description: String,
    objects: Vec<Box<dyn SlyAnn>>,
    size: ImSize,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct SlyAnnBitmap {
    description: String,
    bitmap: Option<Bitmap>,
    tags: Vec<String>,
    classTitle: String,
}

#[derive(Serialize, Debug)]
#[allow(non_snake_case)]
struct SlyAnnBbox {
    description: String,
    tags: Vec<String>,
    geometryType: String,
    classTitle: String,
    points: SlyPoints,
}

// NOTE: erased_serde is necessary as SlyAnn contains a Boxed trait. See
// https://stackoverflow.com/questions/50021897/how-to-implement-serdeserialize-for-a-boxed-trait-object
// for more information.
trait SlyAnn: erased_serde::Serialize {}
impl SlyAnn for SlyAnnBbox {}
impl SlyAnn for SlyAnnBitmap {}
serialize_trait_object!(SlyAnn);

fn create_slyann(bitmap: Option<Bitmap>, bbox: Option<Bbox>, label: &String) -> Box<dyn SlyAnn> {
    let description = String::from("");
    let tags = vec![];
    match bitmap {
        None => Box::new(SlyAnnBbox {
            description,
            tags,
            classTitle: format!("{}_bbox", label),
            geometryType: "rectangle".to_string(),
            points: SlyPoints {
                exterior: match bbox {
                    Some(b) => b.to_vec(),
                    _ => vec![],
                },
                interior: [],
            },
        }),
        _ => Box::new(SlyAnnBitmap {
            description,
            tags,
            classTitle: format!("{}_bitmap", label),
            bitmap,
        }),
    }
}

fn vec_from_anns(anns: &Anns, label: &String) -> Vec<Box<dyn SlyAnn>> {
    let mut out = vec![];
    for (_, ann) in &anns.anns {
        // out.push(create_slyann(ann.bitmap.clone(), None, &label));
        out.push(create_slyann(None, Some(ann.bbox), &label));
    }
    out
}

#[derive(Serialize, Debug)]
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
            tags: vec![],
            description: String::from(""),
            objects: vec_from_anns(&anns, &self.label),
            size: anns.size.clone(),
        };

        let sly_anns = to_string_pretty(&sly_anns)?;
        fs::write(dest, &sly_anns)?;

        Ok(())
    }
}
