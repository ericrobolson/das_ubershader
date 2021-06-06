mod pixel_machine;
use pixel_machine::*;
use rayon::prelude::*;
use std::sync::Arc;

type Texture = Arc<image::DynamicImage>;

fn main() -> Result<(), Error> {
    // TODO: Make these things configuration
    let width = 128;
    let height = 128;

    let output_name = "output.png";
    let program = "
    fragPos 1 texturePixel          
    fragPos 3 texturePixel  
    ";
    let textures = vec!["test.jpg", "test1.png"];

    // END TODO
    let start = std::time::Instant::now();

    let textures: Vec<Texture> = {
        let mut t = vec![];
        textures
            .par_iter()
            .map(|texture| {
                let mut loaded_img = image::open(texture).unwrap();

                let crop_x = 0;
                let crop_y = 0;

                // TODO: deal with smaller images than boundary
                // TODO: crop from center of img?
                // TODO: scale image?
                loaded_img.crop(crop_x, crop_y, width, height);

                Arc::new(loaded_img)
            })
            .collect_into_vec(&mut t);
        t
    };

    // Build up pixels
    let pixels = {
        let mut pixels = vec![];
        for x in 0..width {
            for y in 0..height {
                pixels.push((x, y));
            }
        }
        pixels
    };

    // Process pixels
    let new_pixels: Vec<(u32, u32, Color)> = pixels
        .par_iter()
        .map(|(x, y)| {
            let color = PixelMachine::new(*x, *y, textures.clone())
                .interpret(program)
                .unwrap();

            (*x, *y, color)
        })
        .collect();

    // Write calculated pixels to image
    let mut new_image: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        image::ImageBuffer::new(width, height);

    for (x, y, color) in new_pixels {
        *new_image.get_pixel_mut(x, y) = image::Rgba::<u8>([color.r, color.g, color.b, color.a]);
    }

    // Save and return
    new_image.save(output_name).unwrap();

    println!("DURATION: {:?}", std::time::Instant::now() - start);
    Ok(())
}
