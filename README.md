# Magnify-rs
This is a rust library implementing some simple [Pixel-art scaling algorithms](https://en.wikipedia.org/wiki/Pixel-art_scaling_algorithms).

## Currently supported algorithms
- Scale2x, Scale3x
- Eagle
- Nearest neighbor scaling

## Example
This code scales `image.bmp` using the Scale3X algorithms and then saves the result into `converted.bmp`. 
```rs
use image::io::Reader as ImageReader;
use magnify::Algorithm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open("image.bmp")?.decode()?;

    let converted_img = magnify::convert(img, Algorithm::Scale3X);
    converted_img.save("converted.bmp")?;

    Ok(())
}
```