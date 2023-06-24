use crate::*;
use image::Rgba;

pub fn scale_2x(x: u32, y: u32, px: Rgba<u8>, img: &DynamicImage) -> PixelExpansion {
    let mut expansion = PixelExpansion::new(2, px, (x, y));
    {
        let (x, y) = (x as i32, y as i32);
        // ┌──┬──┬──┐
        // │  │A │  │
        // ├──┼──┼──┤
        // │C │px│B │
        // ├──┼──┼──┤
        // │  │D │  │
        // └──┴──┴──┘
        let A = get_pixel_or_nearest(x, y + 1, &img);
        let B = get_pixel_or_nearest(x + 1, y, &img);
        let C = get_pixel_or_nearest(x - 1, y, &img);
        let D = get_pixel_or_nearest(x, y - 1, &img);

        if C == A && C != D && A != B {
            expansion.pixels[0] = A;
        }
        if A == B && A != C && B != D {
            expansion.pixels[1] = B;
        }
        if D == C && D != B && C != A {
            expansion.pixels[2] = C;
        }
        if B == D && B != A && D != C {
            expansion.pixels[3] = D;
        }
    }

    expansion
}
