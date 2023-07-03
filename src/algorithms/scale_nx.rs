use crate::*;
use image::Rgba;

pub(crate) fn scale_2x(x: u32, y: u32, px: Rgba<u8>, img: &DynamicImage) -> PixelExpansion {
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

pub(crate) fn scale_3x(x: u32, y: u32, px: Rgba<u8>, img: &DynamicImage) -> PixelExpansion {
    let mut expansion = PixelExpansion::new(3, px, (x, y));
    {
        let (x, y) = (x as i32, y as i32);
        // ┌──┬──┬──┐
        // │A │B │C │
        // ├──┼──┼──┤
        // │D │px│F │
        // ├──┼──┼──┤
        // │G │H │I │
        // └──┴──┴──┘
        let A = get_pixel_or_nearest(x - 1, y + 1, &img);
        let B = get_pixel_or_nearest(x, y + 1, &img);
        let C = get_pixel_or_nearest(x + 1, y + 1, &img);
        let D = get_pixel_or_nearest(x - 1, y, &img);
        let F = get_pixel_or_nearest(x + 1, y, &img);
        let G = get_pixel_or_nearest(x - 1, y - 1, &img);
        let H = get_pixel_or_nearest(x, y - 1, &img);
        let I = get_pixel_or_nearest(x + 1, y - 1, &img);

        if D == B && D != H && B != F {
            expansion.pixels[0] = D;
        }
        if (D == B && D != H && B != F && px != C) || (B == F && B != D && F != H && px != A) {
            expansion.pixels[1] = B;
        }
        if B == F && B != D && F != H {
            expansion.pixels[2] = F;
        }
        if (H == D && H != F && D != B && px != A) || (D == B && D != H && B != F && px != G) {
            expansion.pixels[3] = D;
        }
        // expansion.pixels[4] is already equal to px
        if (B == F && B != D && F != H && px != I) || (F == H && F != B && H != D && px != C) {
            expansion.pixels[5] = F;
        }
        if H == D && H != F && D != B {
            expansion.pixels[6] = D;
        }
        if (F == H && F != B && H != D && px != G) || (H == D && H != F && D != B && px != I) {
            expansion.pixels[7] = H;
        }
        if F == H && F != B && H != D {
            expansion.pixels[8] = F;
        }
    }

    expansion
}
