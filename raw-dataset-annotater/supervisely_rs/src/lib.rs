mod sly;
use crate::sly::{infer_anns, sly_create_meta};

use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;

#[derive(Debug)]
pub struct GeneralError {
    pub msg: String,
}

impl GeneralError {
    fn new(msg: &str) -> GeneralError {
        GeneralError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for GeneralError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<io::Error> for GeneralError {
    fn from(err: io::Error) -> GeneralError {
        GeneralError::new(&err.to_string())
    }
}

impl Error for GeneralError {
    fn description(&self) -> &str {
        &self.msg
    }
}

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

    fs::create_dir_all(ann_p);
    fs::create_dir_all(img_p);

    sly_create_meta(cfg.label.clone(), cfg.output_dir.clone());

    let msk_p = format!("{}/{}", cfg.input_dir, cfg.msk_dir);

    if !exists(&msk_p) {
        let err_msg = format!("Masks do not exist in: {}", msk_p);
        return Err(GeneralError::new(&err_msg));
    }

    let msk_paths: Vec<_> = fs::read_dir(&msk_p)?
        .map(|res| res.map(|e| e.path()))
        .collect();

    // TODO: in parallel?
    for msk_path in fs::read_dir(&msk_p)? {
        let entry = msk_path?;
        let pathbuf = entry.path();

        println!("Calculating {}...", pathbuf.to_str().unwrap());
        let ann = infer_anns(pathbuf.as_path());
    }

    return Ok(());
}
