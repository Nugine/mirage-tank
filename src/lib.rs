mod util;

use image::load_from_memory;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageResult;
use image::Pixel;
use image::Rgba;
use image::RgbaImage;

use util::resized;
use util::BLACK_RGBA;
use util::WHITE_RGBA;

pub struct MirageTank {
    wimage: DynamicImage,
    bimage: DynamicImage,
}

impl MirageTank {
    #[inline]
    pub fn new(wimage: DynamicImage, bimage: DynamicImage) -> Self {
        Self { wimage, bimage }
    }

    pub fn from_raw(wbuffer: &[u8], bbuffer: &[u8]) -> ImageResult<Self> {
        let wimage = load_from_memory(wbuffer)?;
        let bimage = load_from_memory(bbuffer)?;
        Ok(Self { wimage, bimage })
    }

    #[inline]
    pub fn wimage_size(&self) -> (u32, u32) {
        self.wimage.dimensions()
    }

    #[inline]
    pub fn bimage_size(&self) -> (u32, u32) {
        self.bimage.dimensions()
    }
}

impl MirageTank {
    pub fn grey_output(
        &self,
        width: u32,
        height: u32,
        checkerboarded: bool,
        wlight: f32,
        blight: f32,
    ) -> RgbaImage {
        let wimage = resized(&self.wimage, width, height).to_rgba();
        let bimage = resized(&self.bimage, width, height).to_rgba();
        let mut oimage = RgbaImage::new(width, height);

        for (((w, h, mut wpixel), mut bpixel), opixel) in wimage
            .enumerate_pixels()
            .zip(bimage.pixels())
            .zip(oimage.pixels_mut())
        {
            if checkerboarded {
                if (w + h) % 2 == 0 {
                    bpixel = &WHITE_RGBA;
                } else {
                    wpixel = &BLACK_RGBA;
                }
            }

            let wc: f32 = f32::from(wpixel.to_luma().data[0]) * wlight;
            let bc: f32 = f32::from(bpixel.to_luma().data[0]) * blight;

            let a = (255.0 - wc + bc).min(255.0).max(0.0);
            let r = (bc / a * 255.0).min(255.0);

            let a = a.round() as u8;
            let r = r.round() as u8;
            *opixel = Rgba { data: [r, r, r, a] }
        }
        oimage
    }

    pub fn colorful_output(
        &self,
        width: u32,
        height: u32,
        checkerboarded: bool,
        wlight: f32,
        blight: f32,
        wcolor: f32,
        bcolor: f32,
    ) -> RgbaImage {
        let wimage = resized(&self.wimage, width, height).to_rgba();
        let bimage = resized(&self.bimage, width, height).to_rgba();
        let mut oimage = RgbaImage::new(width, height);

        let mut wrgb: [f32; 3] = [0., 0., 0.];
        let mut brgb: [f32; 3] = [0., 0., 0.];
        let mut drgb: [f32; 3] = [0., 0., 0.];

        for (((w, h, mut wpixel), mut bpixel), opixel) in wimage
            .enumerate_pixels()
            .zip(bimage.pixels())
            .zip(oimage.pixels_mut())
        {
            if checkerboarded {
                if (w + h) % 2 == 0 {
                    bpixel = &WHITE_RGBA;
                } else {
                    wpixel = &BLACK_RGBA;
                }
            }

            for i in 0..3 {
                wrgb[i] = f32::from(wpixel.data[i]) / 255.0 * wlight;
            }

            for i in 0..3 {
                brgb[i] = f32::from(bpixel.data[i]) / 255.0 * blight;
            }

            let wgrey = ((wrgb[0] + wrgb[1] + wrgb[2]) / 3.0).min(1.0);
            let bgrey = ((brgb[0] + brgb[1] + brgb[2]) / 3.0).min(1.0);

            for c in wrgb.iter_mut() {
                *c = *c * wcolor + wgrey * (1.0 - wcolor);
            }

            for c in brgb.iter_mut() {
                *c = *c * bcolor + bgrey * (1.0 - bcolor);
            }

            for i in 0..3 {
                drgb[i] = 1.0 - wrgb[i] + brgb[i];
            }

            let a = (drgb[0] * 0.2126 + drgb[1] * 0.7152 + drgb[2] * 0.0722)
                .max(brgb[0].max(brgb[1]).max(brgb[2]))
                .min(1.0);

            let r = ((brgb[0] / a).min(1.0) * 255.0).round() as u8;
            let g = ((brgb[1] / a).min(1.0) * 255.0).round() as u8;
            let b = ((brgb[2] / a).min(1.0) * 255.0).round() as u8;
            let a = (a * 255.0).round() as u8;

            *opixel = Rgba { data: [r, g, b, a] }
        }
        oimage
    }
}
