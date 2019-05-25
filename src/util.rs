use image::imageops;
use image::DynamicImage;
use image::GenericImageView;
use image::Rgba;

#[inline]
pub fn resized(src: &DynamicImage, nwidth: u32, nheight: u32) -> DynamicImage {
    let (width, height) = src.dimensions();
    if nwidth < width && nheight < height {
        src.thumbnail_exact(nwidth, nheight)
    } else if nwidth != width && nheight != height {
        src.resize_exact(nwidth, nheight, imageops::CatmullRom)
    } else {
        src.clone()
    }
}

pub const WHITE_RGBA: Rgba<u8> = Rgba {
    data: [0xff, 0xff, 0xff, 0xff],
};

pub const BLACK_RGBA: Rgba<u8> = Rgba {
    data: [0x00, 0x00, 0x00, 0x00],
};
