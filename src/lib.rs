#![allow(non_snake_case)]
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
mod algorithms;

pub fn convert(img: DynamicImage) -> DynamicImage {
    let mut converted_img = DynamicImage::new_rgb8(img.width() * 2, img.height() * 2);

    let pixels = img.pixels();
    let expansions = pixels.map(|(x, y, px)| (x, y, crate::algorithms::scale_2x(x, y, px, &img)));

    for (x, y, expansion) in expansions {
        // Put the resulting pixels where they belong
        let (x, y) = (x * 2, y * 2);
        converted_img.put_pixel(x, y + 1, expansion.0);
        converted_img.put_pixel(x + 1, y + 1, expansion.1);
        converted_img.put_pixel(x, y, expansion.2);
        converted_img.put_pixel(x + 1, y, expansion.3);
    }

    converted_img
}

//   ┌─────┐     ┌──┬──┐
//   │     │     │0 │1 │
//   │pixel├────►├──┼──┤
//   │     │     │2 │3 │
//   └─────┘     └──┴──┘
// todo: better way to define this?
pub struct PixelExpansion(Rgba<u8>, Rgba<u8>, Rgba<u8>, Rgba<u8>);

impl PixelExpansion {
    fn new(px_color: Rgba<u8>) -> Self {
        PixelExpansion(px_color, px_color, px_color, px_color)
    }
}

pub fn get_pixel_or_nearest(x: i32, y: i32, img: &DynamicImage) -> Rgba<u8> {
    let bounds = img.bounds(); // x, y, width, height
    let mut coords: (u32, u32) = (0, 0);

    // make sure x is within bounds
    if x < bounds.0 as i32 {
        coords.0 = bounds.0;
    } else if x >= bounds.2 as i32 {
        coords.0 = bounds.2 - 1;
    } else {
        coords.0 = x as u32;
    };

    // make sure y is within bounds
    if y < bounds.1 as i32 {
        coords.1 = bounds.1;
    } else if y >= bounds.3 as i32 {
        coords.1 = bounds.3 - 1;
    } else {
        coords.1 = y as u32;
    }

    img.get_pixel(coords.0, coords.1)
}
