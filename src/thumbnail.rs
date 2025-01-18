use std::{collections::HashMap, fs::File, path::Path};

use anyhow::Context;
use image::DynamicImage;

use crate::thumbs::*;

pub struct Thumbnailer<'a> {
    pub width: u32,
    pub height: u32,
    mappings: HashMap<&'a str, fn(&Thumbnailer, &Path) -> anyhow::Result<DynamicImage>>,
}

impl Thumbnailer<'_> {
    pub fn new(width: u32, height: u32) -> Self {
        let mut mappings: HashMap<&str, fn(&Thumbnailer, &Path) -> anyhow::Result<DynamicImage>> =
            HashMap::new();

        #[cfg(feature = "img")]
        img::Img::load(&mut mappings);

        #[cfg(feature = "zip")]
        zip::Zip::load(&mut mappings);

        #[cfg(feature = "ffmpeg")]
        if video_rs::init().is_ok() {
            video::Video::load(&mut mappings);
        }

        #[cfg(feature = "pdf")]
        pdf::Pdf::load(&mut mappings);

        Thumbnailer {
            width,
            height,
            mappings,
        }
    }

    pub fn get<T: AsRef<Path>>(&self, path: T) -> anyhow::Result<DynamicImage> {
        let path = path.as_ref();
        let file = File::open(path)?;
        let mime = tree_magic_mini::from_file(&file).context("Failed to find MIME type.")?;

        let func = self
            .mappings
            .get(mime)
            .context(format!("Unsupported MIME type: {mime}"))?;
        func(&self, path)
    }
}

pub trait Thumbnailable {
    const MIME_TYPES: &'static [&'static str];
    fn run(thumbnailer: &Thumbnailer, path: &Path) -> anyhow::Result<DynamicImage>;

    fn load(mappings: &mut HashMap<&str, fn(&Thumbnailer, &Path) -> anyhow::Result<DynamicImage>>) {
        for mime in Self::MIME_TYPES {
            mappings.insert(mime, Self::run);
        }
    }
}
