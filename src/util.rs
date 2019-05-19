use image::imageops;
use image::DynamicImage;
use image::RgbaImage;

pub fn resized(src: RgbaImage, nwidth: u32, nheight: u32) -> RgbaImage {
    DynamicImage::ImageRgba8(src)
        .resize(nwidth, nheight, imageops::CatmullRom)
        .to_rgba()
}
