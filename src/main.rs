mod pixel_machine;

use image::imageops::FilterType;
use pixel_machine::*;
use rayon::prelude::*;
use std::sync::Arc;

type Texture = Arc<image::DynamicImage>;

#[derive(serde::Deserialize)]
pub struct Cfg {
    pub width: u32,
    pub height: u32,
    pub inputs: Vec<String>,
    pub output: String,
    pub program: String,
}

fn main() -> Result<(), Error> {
    let start = std::time::Instant::now();

    // Load config
    let (directory, cfg) = {
        let mut file_path = None;
        for (idx, arg) in std::env::args().enumerate() {
            if idx == 1 {
                file_path = Some(arg);
            }
        }

        match file_path {
            Some(file_path) => {
                println!("Loading cfg from {}", file_path);
                let path = std::path::Path::new(&file_path);
                let working_dir = std::env::current_dir().unwrap().to_path_buf();
                let mut p = working_dir.clone();
                p.push(path);
                let path = p;
                let contents = std::fs::read_to_string(&path).unwrap();
                let cfg: Cfg = serde_json::from_str(&contents).unwrap();

                let directory = {
                    let mut directory = working_dir.clone();
                    match &path.parent() {
                        Some(parent) => {
                            directory.push(parent);
                        }
                        None => {}
                    }
                    directory
                };

                (directory, cfg)
            }
            None => panic!("Required config JSON file!"),
        }
    };

    // Set working values
    let width = cfg.width;
    let height = cfg.height;

    let output_file = {
        let mut file = directory.clone();
        file.push(cfg.output);
        file.to_str().unwrap().to_string()
    };
    let program = {
        let mut file = directory.clone();
        file.push(cfg.program);

        std::fs::read_to_string(file).unwrap()
    };

    // Load all textures
    let textures = {
        let mut textures = vec![];
        for tex in cfg.inputs.iter() {
            let mut file = directory.clone();
            file.push(tex);
            textures.push(file.to_str().unwrap().to_string());
        }
        textures
    };

    let textures: Vec<Texture> = {
        let mut t = vec![];
        textures
            .par_iter()
            .map(|texture| {
                let loaded_img = image::open(texture).unwrap().resize_to_fill(
                    width,
                    height,
                    FilterType::Nearest,
                );
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
            let color = PixelMachine::new(*x, *y, width, height, textures.clone())
                .interpret(&program)
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
    new_image.save(output_file).unwrap();

    println!("DURATION: {:?}", std::time::Instant::now() - start);
    Ok(())
}
