#[macro_use]
extern crate erased_serde;

mod anns;
pub mod errors;
mod util;
use crate::anns::Anns;
use crate::errors::Error;

mod sly;
use crate::sly::SlyDataset;

use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub input_dir: String,
    pub output_dir: String,
    pub dataset_name: String,
    pub label: String,
    pub img_dir: String,
    pub msk_dir: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, Error> {
        args.next(); // skip program name

        let input_dir = match args.next() {
            Some(a) => a,
            None => return Err(Error::from("No input directory provided.")),
        };

        let output_dir = match args.next() {
            Some(a) => a,
            None => return Err(Error::from("No output directory provided.")),
        };

        let dataset_name = match args.next() {
            Some(a) => a,
            None => return Err(Error::from("No dataset name provided.")),
        };

        let label = match args.next() {
            Some(a) => a,
            None => return Err(Error::from("No label (for found objects) provided.")),
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

pub fn gen_anns(cfg: &Config) -> Result<(), Error> {
    let ann_p = format!("{}/{}/ann", cfg.output_dir, cfg.dataset_name);
    let img_p = format!("{}/{}/img", cfg.output_dir, cfg.dataset_name);

    fs::create_dir_all(ann_p)?;
    fs::create_dir_all(img_p)?;

    let dataset_root = Path::new(&cfg.output_dir);

    // TODO: initialise all datasets
    let ds = SlyDataset::new(&cfg.label, dataset_root);

    let msk_p = format!("{}/{}", cfg.input_dir, cfg.msk_dir);

    if !exists(&msk_p) {
        let err_msg = format!("Input masks do not exist in: {}", msk_p);
        return Err(Error::General(err_msg));
    }

    // TODO: in parallel?
    for msk_path in fs::read_dir(&msk_p)? {
        let entry = msk_path?;
        let msk_p = entry.path();

        println!("Reading mask from {:?}...", msk_p);
        let fname = msk_p
            .file_stem()
            .unwrap()
            .to_str()
            .ok_or("Couldn't calculate mask")?;

        let msk_p = msk_p.as_path();
        let anns = Anns::new(msk_p);
        let anns = &anns;

        let ann_p = format!(
            "{}/{}/ann/{}.jpg.json",
            cfg.output_dir, cfg.dataset_name, &fname
        );
        let ann_p = Path::new(&ann_p);

        // TODO: write all items
        ds.write_item(&anns, ann_p)?;
        println!("Item written to {:?}", ann_p);
    }

    return Ok(());
}

pub fn copy_imgs(cfg: &Config) -> () {
    let img_p = format!("{}/{}", cfg.input_dir, cfg.img_dir);
    let new_img_p = format!("{}/{}/img", cfg.output_dir, cfg.dataset_name);
    let cmd = format!("cp -r {}/*.jpg {}/", img_p, new_img_p);
    std::process::Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("failed to copy images over");
    println!("Original images copied to {:?}.", new_img_p)
}
