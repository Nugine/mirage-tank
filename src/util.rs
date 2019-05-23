use image::imageops;
use image::DynamicImage;
use image::RgbaImage;

#[inline]
pub fn resized(src: RgbaImage, nwidth: u32, nheight: u32) -> RgbaImage {
    let size = src.dimensions();
    if size.0 == nwidth && size.1 == nheight {
        return src;
    }

    DynamicImage::ImageRgba8(src)
        .resize_exact(nwidth, nheight, imageops::CatmullRom)
        .to_rgba()
}
