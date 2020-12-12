// use flate2::write::ZlibEncoder;
// use flate2::Compression;
use image::{DynamicImage, GrayImage, Luma, Rgba};
use serde::Serialize;
use std::cmp::{max, min};
// use std::io::prelude::*;

pub type Pos = [u32; 2];
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

#[derive(Serialize, Debug, Clone)]
pub struct ImSize {
    pub height: u32,
    pub width: u32,
}

// TODO
pub fn too_tiny(pxls: &Bbox, size: &ImSize) -> bool {
    false
    // match pxls {
    //     Some(_) => false,
    //     None => true,
    // }
}

fn abs_diff(x: u32, y: u32) -> u32 {
    max(x, y) - min(x, y)
}

pub fn crop_black_and_white_b64(im: &mut DynamicImage, bounds: [Pos; 2]) -> String {
    let [tr, bl] = bounds;
    let w = abs_diff(tr[0], bl[0]);
    let h = abs_diff(tr[1], bl[1]);

    let im = im.crop(tr[0], tr[1], w, h);
    let crop = im.to_luma8();
    let mut mask = GrayImage::new(w, h);

    let black_px = Luma([0]);
    let white_px = Luma([255]);

    for y in 0..h {
        for x in 0..w {
            let p = crop.get_pixel(x, y);
            mask.put_pixel(
                x,
                y,
                match p[0] {
                    0 => black_px,
                    _ => white_px,
                },
            );
        }
    }

    mask.save("/tmp/testmask.png").unwrap();
    let mut buf = vec![];
    let mask = DynamicImage::ImageLuma8(mask);
    mask.write_to(&mut buf, image::ImageOutputFormat::Png)
        .unwrap();

    // let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    // e.write_all(mask.as_bytes()).unwrap();
    // let compressed_bytes = e.finish().unwrap();
    // base64::encode(&compressed_bytes)

    base64::encode(&buf)
}
