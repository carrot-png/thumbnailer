use std::{env::args, path::Path};

use thumbnailer::thumbnail::Thumbnailer;

fn main() {
    let thumbnailer = Thumbnailer::new(250, 250);

    let path = args().last().unwrap();
    let path = Path::new(&path);

    let thumb = thumbnailer.get(&path).unwrap();
    thumb.save("read.png").unwrap();
    println!("Saved");
}
