use image::{GenericImage, GenericImageView, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use image::io::Reader as ImageReader;

    let img = ImageReader::open("src/tests/images/tilemap_color_packed.png")?.decode()?;
    // Add 1px padding on all sides
    let pad_amount: u32 = 1;

    for (x, y, px) in img.pixels().filter(|(x, y, px)| {
        (x < &pad_amount
            || y < &pad_amount
            || x > &(img.width() - pad_amount)
            || y > &(img.width() - pad_amount))
    }) {
        // ┌──┬──┬──┐
        // │  │A │  │
        // ├──┼──┼──┤
        // │C │px│B │
        // ├──┼──┼──┤
        // │  │D │  │
        // └──┴──┴──┘
        // By ChrisNF - Own work, CC BY-SA 4.0, https://commons.wikimedia.org/w/index.php?curid=71857945
        let A = img.get_pixel(x, y + 1);
        let B = img.get_pixel(x + 1, y);
        let C = img.get_pixel(x, y - 1);
        let D = img.get_pixel(x - 1, y);

        let mut expansion = PixelExpansion::new(px);

        if (C == A && C != D && A != B) {
            expansion.0 = A;
        }
        if (A == B && A != C && B != D) {
            expansion.1 = B;
        }
        if (D == C && D != B && C != A) {
            expansion.2 = C;
        }
        if (B == D && B != A && D != C) {
            expansion.3 = D;
        }
    }

    Ok(())
}

//   ┌─────┐     ┌──┬──┐
//   │     │     │0 │1 │
//   │pixel├────►├──┼──┤
//   │     │     │2 │3 │
//   └─────┘     └──┴──┘
// todo: better way to define this?
struct PixelExpansion(Rgba<u8>, Rgba<u8>, Rgba<u8>, Rgba<u8>);

impl PixelExpansion {
    fn new(px_color: Rgba<u8>) -> Self {
        PixelExpansion(px_color, px_color, px_color, px_color)
    }
}
