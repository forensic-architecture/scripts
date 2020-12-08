mod sly;
use crate::sly::sly_create_meta;

use std::env;
use std::fs;

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

pub fn gen_anns(cfg: &Config) -> () {
    println!("{:?}", cfg);
    println!("generating anns...");

    let ann_p = format!("{}/{}/ann", cfg.output_dir, cfg.dataset_name);
    let img_p = format!("{}/{}/img", cfg.output_dir, cfg.dataset_name);

    fs::create_dir_all(ann_p);
    fs::create_dir_all(img_p);

    sly_create_meta(cfg.label.clone(), cfg.output_dir.clone());
}
