use image::{DynamicImage, ImageReader};

use crate::thumbnail::*;
use std::path::Path;

pub struct Img;

impl Thumbnailable for Img {
    const MIME_TYPES: &'static [&'static str] = &["image/jpeg", "image/png", "image/gif"];

    fn run(thumbnailer: &Thumbnailer, path: &Path) -> anyhow::Result<DynamicImage> {
        Ok(ImageReader::open(path)?
            .with_guessed_format()?
            .decode()?
            .thumbnail(thumbnailer.width, thumbnailer.height))
    }
}
