use image::RgbaImage;

use mirage_tank::MirageTank;

const WPATH: &'static str = "./tests/w.jpg";
const BPATH: &'static str = "./tests/b.jpg";
const OPATH: &'static str = "./tests/o.png";

fn open_images() -> (RgbaImage, RgbaImage) {
    let wimg = image::open(WPATH)
        .expect(&format!("fail to open image: {}", WPATH))
        .to_rgba();
    let bimg = image::open(BPATH)
        .expect(&format!("fail to open image: {}", BPATH))
        .to_rgba();
    (wimg, bimg)
}

#[test]
fn grey() {
    let (wimg, bimg) = open_images();
    dbg!(wimg.dimensions());
    dbg!(bimg.dimensions());
    let mt = MirageTank::new(wimg, bimg, 300, 400);
    dbg!(mt.size());

    let output = mt.checkerboarded().grey_output(1.0, 0.2);

    output
        .save(OPATH)
        .expect(&format!("fail to save output image to {}", OPATH));
}
