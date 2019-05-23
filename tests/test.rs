use image::RgbaImage;

use mirage_tank::MirageTank;

fn sample_path(idx: u32) -> (String, String, String, String) {
    (
        format!("./tests/sample-{}/w.jpg", idx),
        format!("./tests/sample-{}/b.jpg", idx),
        format!("./tests/sample-{}/o.grey.png", idx),
        format!("./tests/sample-{}/o.colorful.png", idx),
    )
}

fn open_images(wpath: &str, bpath: &str) -> (RgbaImage, RgbaImage) {
    let wimg = image::open(wpath)
        .expect(&format!("fail to open image: {}", wpath))
        .to_rgba();
    let bimg = image::open(bpath)
        .expect(&format!("fail to open image: {}", bpath))
        .to_rgba();
    (wimg, bimg)
}

fn test_sample(idx: u32) {
    let (wpath, bpath, gpath, cpath) = sample_path(idx);

    let (wimg, bimg) = open_images(&wpath, &bpath);
    let mt = MirageTank::new(wimg, bimg, 300, 400).checkerboarded();

    mt.grey_output(1.0, 0.2)
        .expect("fail to create output image")
        .save(&gpath)
        .expect(&format!("fail to save output image to {}", gpath));

    mt.colorful_output(1.0, 0.2, 0.5, 0.7)
        .expect("fail to create output image")
        .save(&cpath)
        .expect(&format!("fail to save output image to {}", cpath));
}

#[test]
fn test_sample_1() {
    test_sample(1);
}

#[test]
fn test_sample_2() {
    test_sample(2);
}
