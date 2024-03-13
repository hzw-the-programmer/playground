use epub::doc::EpubDoc;
use std::fs;
use std::io::Write;

fn main() {
    let doc = EpubDoc::new("One man's view of the world.epub");
    assert!(doc.is_ok());
    let mut doc = doc.unwrap();

    let cover_data = doc.get_cover().unwrap();

    let f = fs::File::create("cover.png");
    assert!(f.is_ok());
    let mut f = f.unwrap();
    let resp = f.write_all(&cover_data);
}
