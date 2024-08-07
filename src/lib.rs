#![allow(non_snake_case)]
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};
mod algorithms;
pub use algorithms::Algorithm;

pub fn convert(img: DynamicImage, algorithm: algorithms::Algorithm) -> DynamicImage {
    // choose the algorithm to use to expand the image
    // it's a boxed closure so I can pass the size to the arbitrary-size algorithm(s)
    let algorithm_fn: Box<dyn Fn(u32, u32, Rgba<u8>, &DynamicImage) -> PixelExpansion> =
        match algorithm {
            Algorithm::Scale2X => Box::new(algorithms::scale_2x),
            Algorithm::Eagle => Box::new(algorithms::eagle),
            Algorithm::Scale3X => Box::new(algorithms::scale_3x),
            // Arbitrary-sized
            Algorithm::NearestNeighbor { size } => Box::new(move |x, y, px, img: &DynamicImage| {
                algorithms::nearest_neighbor(size, x, y, px, img)
            }),
        };
    let expansion_size: u32 = match algorithm {
        // 2x2 algorithms
        Algorithm::Scale2X => 2,
        Algorithm::Eagle => 2,
        // 3x3 algorithms
        Algorithm::Scale3X => 3,
        // Arbitrary-size algorithms
        Algorithm::NearestNeighbor { size } => size,
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
pub(crate) struct PixelExpansion {
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

pub(crate) fn get_pixel_or_nearest(x: i32, y: i32, img: &DynamicImage) -> Rgba<u8> {
    let (sub_w, sub_h) = img.dimensions();
    let mut coords: (u32, u32) = (0, 0);

    // make sure x is within bounds
    if x < 0 {
        coords.0 = 0;
    } else if x >= sub_w as i32 {
        coords.0 = sub_w - 1;
    } else {
        coords.0 = x as u32;
    };

    // make sure y is within bounds
    if y < 0 {
        coords.1 = 0;
    } else if y >= sub_h as i32 {
        coords.1 = sub_h - 1;
    } else {
        coords.1 = y as u32;
    }

    img.get_pixel(coords.0, coords.1)
}
