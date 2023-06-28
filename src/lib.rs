#![allow(non_snake_case)]
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
mod algorithms;
pub use algorithms::Algorithm;

pub fn convert(img: DynamicImage, algorithm: algorithms::Algorithm) -> DynamicImage {
    // do the filter
    let algorithm_fn = match algorithm {
        // this function will get executed to expand the image
        Algorithm::Scale2X => algorithms::scale_2x,
        Algorithm::Scale3X => algorithms::scale_3x,
        Algorithm::Eagle => algorithms::eagle,
    };
    let expansion_size: u32 = match algorithm {
        // 2x2 algorithms
        Algorithm::Scale2X => 2,
        Algorithm::Eagle => 2,
        // 3x3 algorithms
        Algorithm::Scale3X => 3,
    };

    let pixels = img.pixels();
    let expansions = pixels.map(|(x, y, px)| (algorithm_fn(x, y, px, &img)));

    // put the pixels into the result image
    let mut converted_img =
        DynamicImage::new_rgb8(img.width() * expansion_size, img.height() * expansion_size);
    for expansion in expansions {
        expansion.put_into_image(&mut converted_img);
    }

    converted_img
}

// the filter expands a pixel in an image into more pixels -> this struct
//   ┌─────┐     ┌──┬──┐
//   │     │     │0 │1 │
//   │pixel├────►├──┼──┤
//   │     │     │2 │3 │
//   └─────┘     └──┴──┘
pub struct PixelExpansion {
    size: u8,
    pub pixels: Vec<Rgba<u8>>,
    original_coords: (u32, u32), // (x, y) of point which is being expanded
}

impl PixelExpansion {
    pub fn new(size: u8, px_color: Rgba<u8>, original_coords: (u32, u32)) -> Self {
        PixelExpansion {
            size,
            pixels: vec![px_color; size as usize * size as usize],
            original_coords,
        }
    }

    // puts the pixel expansion into the Image, assuming it's the correct size
    pub fn put_into_image(&self, img: &mut DynamicImage) {
        let (x, y) = self.original_coords;
        for row in 0..self.size {
            for column in 0..self.size {
                let pixel = self.pixels[(self.size * row + column) as usize];

                let x = x * self.size as u32 + column as u32;
                // y coords get flipped, hence the (self.size - 1 - row)
                let y = y * self.size as u32 + (self.size - 1 - row) as u32;
                img.put_pixel(x, y, pixel);
            }
        }
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
