use crate::*;
use image::Rgba;

pub fn eagle(x: u32, y: u32, px: Rgba<u8>, img: &DynamicImage) -> PixelExpansion {
    let mut expansion = PixelExpansion::new(2, px, (x, y));
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

        if A == B && B == D {
            // top left
            expansion.pixels[0] = A;
        }

        if B == C && C == F {
            // top right
            expansion.pixels[1] = B;
        }

        if D == G && G == H {
            // bottom left
            expansion.pixels[2] = D;
        }

        if F == I && I == H {
            // bottom right
            expansion.pixels[3] = F;
        }
    }

    expansion
}
