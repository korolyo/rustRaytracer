use image::{ImageBuffer, RgbImage};

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    let mut image: RgbImage = ImageBuffer::new(image_width, image_height);

    for i in 0..image_width {
        for j in 0..image_height {
            let ir = (i) as u8;
            let ig = (j) as u8;
            let ib = 0;
            image.put_pixel(i, j, image::Rgb([ir, ig, ib]));
        }
    }
    image.save("ColourPalette.png").unwrap();
}
