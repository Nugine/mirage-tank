use image::RgbaImage;

use mirage_tank::MirageTank;

const WPATH: &'static str = "./tests/w.jpg";
const BPATH: &'static str = "./tests/b.jpg";
const GREY_OPATH: &'static str = "./tests/o.grey.png";
const COLORFUL_OPATH: &'static str = "./tests/o.colorful.png";

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
fn test() {
    let (wimg, bimg) = open_images();
    let mt = MirageTank::new(wimg, bimg, 300, 400).checkerboarded();

    mt.grey_output(1.0, 0.2)
        .save(GREY_OPATH)
        .expect(&format!("fail to save output image to {}", GREY_OPATH));

    mt.colorful_output(1.0, 0.2, 0.5, 0.7)
        .save(COLORFUL_OPATH)
        .expect(&format!("fail to save output image to {}", COLORFUL_OPATH));
}
