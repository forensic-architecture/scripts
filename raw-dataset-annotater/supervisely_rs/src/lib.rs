mod errors;
use crate::errors::*;

mod sly;
use crate::sly::{create_ann, create_meta};

use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    input_dir: String,
    output_dir: String,
    dataset_name: String,
    label: String,
    img_dir: String,
    msk_dir: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // skip program name

        let input_dir = match args.next() {
            Some(a) => a,
            None => return Err("No input directory provided."),
        };

        let output_dir = match args.next() {
            Some(a) => a,
            None => return Err("No output directory provided."),
        };

        let dataset_name = match args.next() {
            Some(a) => a,
            None => return Err("No dataset name provided."),
        };

        let label = match args.next() {
            Some(a) => a,
            None => return Err("No label (for found objects) provided."),
        };

        Ok(Config {
            input_dir,
            output_dir,
            dataset_name,
            label,
            img_dir: "X".to_string(),
            msk_dir: "Y".to_string(),
        })
    }
}

fn exists(pth: &String) -> bool {
    fs::metadata(pth).is_ok()
}

pub fn gen_anns(cfg: &Config) -> Result<(), GeneralError> {
    println!("{:?}", cfg);
    println!("generating anns...");

    let ann_p = format!("{}/{}/ann", cfg.output_dir, cfg.dataset_name);
    let img_p = format!("{}/{}/img", cfg.output_dir, cfg.dataset_name);

    fs::create_dir_all(ann_p)?;
    fs::create_dir_all(img_p)?;

    create_meta(cfg.label.clone(), cfg.output_dir.clone());

    let msk_p = format!("{}/{}", cfg.input_dir, cfg.msk_dir);

    if !exists(&msk_p) {
        let err_msg = format!("Masks do not exist in: {}", msk_p);
        return Err(GeneralError::new(&err_msg));
    }

    // TODO: in parallel?
    for msk_path in fs::read_dir(&msk_p)? {
        let entry = msk_path?;
        let msk_p = entry.path();

        println!("Calculating {}...", msk_p.to_str().unwrap());
        let fname = msk_p
            .file_stem()
            .unwrap()
            .to_str()
            .ok_or_else(|| GeneralError::new("Couldn't calculate mask."))?
            .to_string();
        let ann_p = format!("{}/{}.json", cfg.output_dir, &fname);

        create_ann(msk_p.as_path(), Path::new(&ann_p), cfg.label.clone());
    }

    return Ok(());
}
