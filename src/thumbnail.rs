use std::{collections::HashMap, fs::File, path::Path};

use image::DynamicImage;

use crate::thumbs::*;

pub struct Thumbnailer<'a> {
    pub width: u32,
    pub height: u32,
    mappings: HashMap<&'a str, fn(&Thumbnailer, &Path) -> Option<DynamicImage>>,
}

impl Thumbnailer<'_> {
    pub fn new(width: u32, height: u32) -> Self {
        let mut mappings: HashMap<&str, fn(&Thumbnailer, &Path) -> Option<DynamicImage>> =
            HashMap::new();

        img::Img::load(&mut mappings);

        Thumbnailer {
            width,
            height,
            mappings,
        }
    }

    pub fn get(&self, path: &Path) -> Option<DynamicImage> {
        let file = File::open(path).ok()?;
        let mime = tree_magic_mini::from_file(&file)?;
        println!("Reading path with MIME: {}", mime);

        let func = self.mappings.get(mime)?;
        func(&self, path)
    }
}

pub trait Thumbnailable {
    const MIME_TYPES: &'static [&'static str];
    fn run(thumbnailer: &Thumbnailer, path: &Path) -> Option<DynamicImage>;

    fn load(mappings: &mut HashMap<&str, fn(&Thumbnailer, &Path) -> Option<DynamicImage>>) {
        for mime in Self::MIME_TYPES {
            mappings.insert(mime, Self::run);
        }
    }
}
