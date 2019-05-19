mod util;

use image::ImageBuffer;
use image::Pixel;
use image::Rgba;
use image::RgbaImage;

pub struct MirageTank {
    wimage: RgbaImage,
    bimage: RgbaImage,
    width: u32,
    height: u32,
}

// getter
impl MirageTank {
    #[inline]
    pub fn wimage(&self) -> &RgbaImage {
        &self.wimage
    }

    #[inline]
    pub fn bimage(&self) -> &RgbaImage {
        &self.bimage
    }

    #[inline]
    pub fn size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

// core
impl MirageTank {
    // wlight: [0,1], blight: [0,1]
    fn grey(wpixel: Rgba<u8>, bpixel: Rgba<u8>, wlight: f32, blight: f32) -> Rgba<u8> {
        let wc: f32 = f32::from(wpixel.to_luma().data[0]) * wlight;
        let bc: f32 = f32::from(bpixel.to_luma().data[0]) * blight;

        let a = (255.0 - wc + bc).min(255.0).max(0.0);
        let r = (bc / a * 255.0).min(255.0);

        let a = a.round() as u8;
        let r = r.round() as u8;
        Rgba { data: [r, r, r, a] }
    }

    // wlight: [0,1], blight: [0,1], wcolor: [0, 1], bcolor: [0, 1]
    fn colorful(
        wpixel: Rgba<u8>,
        bpixel: Rgba<u8>,
        wlight: f32,
        blight: f32,
        wcolor: f32,
        bcolor: f32,
    ) -> Rgba<u8> {
        let mut wrgb: [f32; 3] = [0., 0., 0.];
        let data = wpixel.to_rgb().data;
        for i in 0..3 {
            wrgb[i] = f32::from(data[i]) / 255.0 * wlight;
        }

        let mut brgb: [f32; 3] = [0., 0., 0.];
        let data = bpixel.to_rgb().data;
        for i in 0..3 {
            brgb[i] = f32::from(data[i]) / 255.0 * blight;
        }

        let wgrey = ((wrgb[0] + wrgb[1] + wrgb[2]) / 3.0).min(1.0);
        let bgrey = ((brgb[0] + brgb[1] + brgb[2]) / 3.0).min(1.0);

        for c in wrgb.iter_mut() {
            *c = *c * wcolor + wgrey * (1.0 - wcolor);
        }

        for c in brgb.iter_mut() {
            *c = *c * bcolor + bgrey * (1.0 - bcolor);
        }

        let mut drgb: [f32; 3] = [0., 0., 0.];
        for i in 0..3 {
            drgb[i] = 1.0 - wrgb[i] + brgb[i];
        }

        let maxc = brgb[0].max(brgb[1]).max(brgb[2]);
        let a = (drgb[0] * 0.222 + drgb[1] * 0.707 + drgb[2] * 0.071)
            .max(maxc)
            .min(1.0);

        let r = ((brgb[0] / a).min(1.0) * 255.0).round() as u8;
        let g = ((brgb[1] / a).min(1.0) * 255.0).round() as u8;
        let b = ((brgb[2] / a).min(1.0) * 255.0).round() as u8;
        let a = (a * 255.0).round() as u8;

        Rgba { data: [r, g, b, a] }
    }
}

// pub
impl MirageTank {
    #[inline]
    pub fn new(wimage: RgbaImage, bimage: RgbaImage, width: u32, height: u32) -> Self {
        use util::resized;

        Self {
            wimage: resized(wimage, width, height),
            bimage: resized(bimage, width, height),
            width,
            height,
        }
    }

    pub fn checkerboarded(mut self) -> Self {
        for w in 0..self.width {
            for h in 0..self.height {
                if (w + h) % 2 == 0 {
                    self.bimage.put_pixel(
                        w,
                        h,
                        Rgba {
                            data: [255, 255, 255, 255],
                        },
                    )
                } else {
                    self.wimage.put_pixel(w, h, Rgba { data: [0, 0, 0, 0] })
                }
            }
        }
        self
    }

    fn output<F>(&self, merge: F) -> Option<RgbaImage>
    where
        F: Fn(Rgba<u8>, Rgba<u8>) -> Rgba<u8>,
    {
        let pixels = self
            .wimage
            .pixels()
            .zip(self.bimage.pixels())
            .map(|(&wp, &bp)| merge(wp, bp));

        let mut output: Vec<u8> = Vec::new();
        for p in pixels {
            output.extend(p.data.iter());
        }

        ImageBuffer::from_raw(self.width, self.height, output)
    }

    #[inline]
    pub fn grey_output(&self, wlight: f32, blight: f32) -> RgbaImage {
        self.output(|wp, bp| MirageTank::grey(wp, bp, wlight, blight))
            .expect("fail to create output image from pixels")
    }

    #[inline]
    pub fn colorful_output(&self, wlight: f32, blight: f32, wcolor: f32, bcolor: f32) -> RgbaImage {
        self.output(|wp, bp| MirageTank::colorful(wp, bp, wlight, blight, wcolor, bcolor))
            .expect("fail to create output image from pixels")
    }
}
