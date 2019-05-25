use mirage_tank::MirageTank;

fn sample_path(idx: u32) -> (String, String, String, String) {
    (
        format!("./tests/sample-{}/w.jpg", idx),
        format!("./tests/sample-{}/b.jpg", idx),
        format!("./tests/sample-{}/o.grey.png", idx),
        format!("./tests/sample-{}/o.colorful.png", idx),
    )
}

#[test]
fn test_sample_1() {
    let (wpath, bpath, gpath, cpath) = sample_path(1);
    let wimg = image::open(&wpath).expect(&format!("fail to open image: {}", wpath));
    let bimg = image::open(&bpath).expect(&format!("fail to open image: {}", bpath));

    let mt = MirageTank::new(wimg, bimg);

    let gimg = mt.grey_output(300, 400, true, 1.0, 0.2);
    let cimg = mt.colorful_output(300, 400, false, 1.0, 0.2, 0.5, 0.7);
    gimg.save(&gpath)
        .expect(&format!("fail to save output image to {}", gpath));
    cimg.save(&cpath)
        .expect(&format!("fail to save output image to {}", cpath));
}

#[test]
fn test_sample_2() {
    let (wpath, bpath, gpath, cpath) = sample_path(1);
    let wbuf = std::fs::read(&wpath).expect(&format!("fail to open image: {}", wpath));
    let bbuf = std::fs::read(&bpath).expect(&format!("fail to open image: {}", bpath));

    let mt = MirageTank::from_raw(&wbuf, &bbuf).expect("fail to load image");

    let gimg = mt.grey_output(300, 400, true, 1.0, 0.2);
    let cimg = mt.colorful_output(300, 400, false, 1.0, 0.2, 0.5, 0.7);
    gimg.save(&gpath)
        .expect(&format!("fail to save output image to {}", gpath));
    cimg.save(&cpath)
        .expect(&format!("fail to save output image to {}", cpath));
}
