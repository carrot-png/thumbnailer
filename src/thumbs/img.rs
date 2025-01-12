use image::{DynamicImage, ImageReader};

use crate::thumbnail::*;
use std::path::Path;

pub struct Img;

impl Thumbnailable for Img {
    const MIME_TYPES: &'static [&'static str] = &["image/jpeg", "image/png", "image/gif"];

    fn run(thumbnailer: &Thumbnailer, path: &Path) -> Option<DynamicImage> {
        Some(ImageReader::open(path)
            .ok()?
            .decode()
            .ok()?
            .thumbnail(thumbnailer.width, thumbnailer.height))
    }
}
