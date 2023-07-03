use crate::*;
use image::Rgba;

pub(crate) fn nearest_neighbor(
    size: u32,
    x: u32,
    y: u32,
    px: Rgba<u8>,
    _img: &DynamicImage,
) -> PixelExpansion {
    let expansion = PixelExpansion::new(size as u8, px, (x, y));
    expansion
}
