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
    fn greyize(wpixel: Rgba<u8>, bpixel: Rgba<u8>, wlight: f32, blight: f32) -> Rgba<u8> {
        let wc: f32 = f32::from(wpixel.to_luma().data[0]) * wlight;
        let bc: f32 = f32::from(bpixel.to_luma().data[0]) * blight;

        let a = (255.0 - wc + bc).min(255.0).max(0.0);
        let r = (bc / a * 255.0).min(255.0).max(0.0);

        let a = a.round() as u8;
        let r = r.round() as u8;
        Rgba { data: [r, r, r, a] }
    }

    fn output(
        &self,
        wlight: f32,
        blight: f32,
        merge: fn(Rgba<u8>, Rgba<u8>, f32, f32) -> Rgba<u8>,
    ) -> Option<RgbaImage> {
        let pixels = self
            .wimage
            .pixels()
            .zip(self.bimage.pixels())
            .map(|(&wp, &bp)| merge(wp, bp, wlight, blight));

        let mut output: Vec<u8> = Vec::new();
        for p in pixels {
            output.extend(p.data.iter());
        }

        ImageBuffer::from_raw(self.width, self.height, output)
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

    #[inline]
    pub fn grey_output(&self, wlight: f32, blight: f32) -> RgbaImage {
        self.output(wlight, blight, MirageTank::greyize)
            .expect("fail to create output image from pixels")
    }
}
