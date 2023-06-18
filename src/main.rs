use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use image::io::Reader as ImageReader;

    let img = ImageReader::open("src/tests/images/tilemap_color_packed.png")?.decode()?;

    for (x, y, px) in img.pixels() {
        let (x, y) = (x as i32, y as i32);
        // ┌──┬──┬──┐
        // │  │A │  │
        // ├──┼──┼──┤
        // │C │px│B │
        // ├──┼──┼──┤
        // │  │D │  │
        // └──┴──┴──┘
        // By ChrisNF - Own work, CC BY-SA 4.0, https://commons.wikimedia.org/w/index.php?curid=71857945
        let A = get_pixel_or_nearest(x, y + 1, &img);
        let B = get_pixel_or_nearest(x + 1, y, &img);
        let C = get_pixel_or_nearest(x, y - 1, &img);
        let D = get_pixel_or_nearest(x - 1, y, &img);

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
    // img.save("src/tests/images/tilemap_color_converted.png");
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

fn get_pixel_or_nearest(x: i32, y: i32, img: &DynamicImage) -> Rgba<u8> {
    let bounds = img.bounds(); // x, y, width, height

    // todo: tidy this up (extract into function perhamps?)
    // Is x in bounds?
    let x: u32 = if x >= 0 && img.in_bounds(x as u32, bounds.1) {
        x as u32 // it is, keep it
    } else {
        closest_to(x, bounds.0, bounds.2) // it is not, change it
    };

    // Is y in bounds?
    let y: u32 = if y >= 0 && img.in_bounds(bounds.0, y as u32) {
        y as u32 // it is, keep it
    } else {
        closest_to(y, bounds.1, bounds.3) // it is not, change it
    };

    img.get_pixel(x, y)
}

fn closest_to(num: i32, first: u32, second: u32) -> u32 {
    let dist_to_first = (num - first as i32).abs();
    let dist_to_second = (num - second as i32).abs();

    if dist_to_first < dist_to_second {
        first
    } else {
        second
    }
}
