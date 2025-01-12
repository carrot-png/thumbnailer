use image::DynamicImage;
use tempfile::NamedTempFile;

use crate::thumbnail::*;
use std::{fs::File, io, path::Path};

pub struct Zip;

impl Thumbnailable for Zip {
    const MIME_TYPES: &'static [&'static str] = &["application/zip"];

    fn run(thumbnailer: &Thumbnailer, path: &Path) -> anyhow::Result<DynamicImage> {
        let archive_file = File::open(path)?;
        let mut archive = zip::ZipArchive::new(archive_file)?;

        let mut file = archive.by_index(0)?;
        let mut temp = NamedTempFile::new()?;
        io::copy(&mut file, &mut temp)?;

        thumbnailer.get(temp.path())
    }
}
