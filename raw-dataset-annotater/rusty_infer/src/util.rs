extern crate base64;
use image::{DynamicImage, Rgba};
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};

pub type Pos = [u32; 2];
pub type Mask = Vec<Pos>;
pub type Bbox = [Pos; 2];
pub type Pixel = Rgba<u8>;

pub trait PixelMethods {
    fn to_str(&self) -> String;
}

impl PixelMethods for Pixel {
    fn to_str(&self) -> String {
        format!("{},{},{}", self[0], self[1], self[2])
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImSize {
    pub height: u32,
    pub width: u32,
}

// TODO
pub fn too_tiny(pxls: Bbox, size: &ImSize) -> bool {
    false
}

fn abs_diff(x: u32, y: u32) -> u32 {
    max(x, y) - min(x, y)
}

pub fn crop_b64(im: &mut DynamicImage, bounds: [Pos; 2]) -> String {
    let [tr, bl] = bounds;
    let w = abs_diff(tr[0], bl[0]);
    let h = abs_diff(tr[1], bl[1]);
    let im = im.crop(tr[0], tr[1], w, h);
    let mut buf = vec![];
    im.write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();
    base64::encode(&buf)
}
