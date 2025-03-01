use std::env::args;
use thumbnails::Thumbnailer;

fn main() {
    let thumbnailer = Thumbnailer::new(250, 250);

    let path = args().next_back().unwrap();
    let thumb = thumbnailer.get(&path).unwrap();
    thumb.save("read.png").unwrap();
    println!("Saved");
}
