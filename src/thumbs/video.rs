use crate::thumbnail::*;
use anyhow::Context;
use image::{DynamicImage, ImageBuffer};
use std::path::Path;
use video_rs::decode::Decoder;

pub struct Video;

impl Thumbnailable for Video {
    const MIME_TYPES: &'static [&'static str] =
        &["video/mp4", "video/webm", "application/x-matroska"];

    fn run(thumbnailer: &Thumbnailer, path: &Path) -> anyhow::Result<DynamicImage> {
        let mut decoder = Decoder::new(path)?;

        let (width, height) = decoder.size();
        let frame = decoder.decode()?.1;

        let buf = frame
            .as_slice()
            .context("Failed to turn frame into slice.")?;

        let img: ImageBuffer<image::Rgb<u8>, Vec<u8>> =
            ImageBuffer::from_raw(width, height, buf.to_vec())
                .context("Failed to construct image buffer.")?;

        let thumb = DynamicImage::ImageRgb8(img).thumbnail(thumbnailer.width, thumbnailer.height);
        return Ok(thumb);
    }
}
