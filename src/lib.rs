mod util;

use image::Rgba;
use image::RgbaImage;

pub struct MirageTank {
    wimage: RgbaImage,
    bimage: RgbaImage,
    size: (u32, u32),
}

impl MirageTank {
    pub fn new(wimage: RgbaImage, bimage: RgbaImage, width: u32, height: u32) -> Self {
        use util::resized;

        // ensure the same size
        Self {
            wimage: resized(wimage, width, height),
            bimage: resized(bimage, width, height),
            size: (width, height),
        }
    }

    pub fn checkerboarded(mut self) -> Self {
        let (width, height) = self.size;
        for w in 0..width {
            for h in 0..height {
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
}
