type Image = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
use image::GenericImageView;

fn main() {
    let loaded_img = image::open("test_pic.jpg").unwrap();
    let (width, height) = loaded_img.dimensions();

    let mut turtler = Turtler::new(width, height);

    for x in 0..width {
        let mut flapper = false;
        for y in 0..height {
            flapper = !flapper;
            if flapper {
                let color = loaded_img.get_pixel(x, y);

                turtler.draw_line(color, x, y, 1, 1);
            } else {
                let color = color((x % 255) as u8, (y % 255) as u8, 200);
                turtler.draw_line(color, x, y, 1, 1);
            }
        }
    }

    // Compositor - this should enable creating 'fractally' images
    /*
    * Load image
    * * Sample N pixels in a line, or turtle
    * * Then avg those pixels
    * * Take avg pixels and map to palette
    * * Write to image
    * Save image
    * Finally, display that on canvas using projector and paint it

    */

    turtler.save("test.png");
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Turtler {
    output_buffer: Image,
    width: u32,
    height: u32,
}

impl Turtler {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            height,
            output_buffer: Image::new(width, height),
            width,
        }
    }

    pub fn draw_line(&mut self, color: Color, x0: u32, y0: u32, slope: u32, len: u32) {
        let mut x = x0;
        let mut y = y0;
        for l in 0..len {
            for _ in 0..slope {
                *self.output_buffer.get_pixel_mut(x, y) = color;

                y = self.increment_y(y);
            }

            x = self.increment_x(x);
        }
    }

    fn increment_x(&self, x: u32) -> u32 {
        let x = x.wrapping_add(1);

        if x >= self.width {
            0
        } else {
            x
        }
    }

    fn increment_y(&self, y: u32) -> u32 {
        let y = y.wrapping_add(1);

        if y >= self.height {
            0
        } else {
            y
        }
    }

    pub fn save(&self, file: &str) {
        self.output_buffer.save(file).unwrap();
    }
}

type Color = image::Rgba<u8>;

fn color(r: u8, g: u8, b: u8) -> Color {
    image::Rgba([r, g, b, 255])
}

fn paint(direction: Direction, width: u32, height: u32, len: u32, img: &mut Image) {
    let mut x = 0;
    let mut y = 0;

    let colors = vec![color(255, 0, 0), color(0, 255, 0), color(0, 0, 255)];

    for (i, color) in colors.iter().enumerate() {
        let mut len = len;
        let mut direction = direction;

        let mut should_continue = true;

        // TODO: add a stack to do more recursive things

        while len != 0 && should_continue {
            // Draw initial turtle
            *img.get_pixel_mut(x, y) = *color;

            // Do turtle
            for _ in 0..len {
                // TODO: wraps
                match direction {
                    Direction::Up => y = y.wrapping_sub(1).clamp(0, height - 1),
                    Direction::Down => y = y.wrapping_add(1).clamp(0, height - 1),
                    Direction::Left => x = x.wrapping_sub(1).clamp(0, width - 1),
                    Direction::Right => x = x.wrapping_add(1).clamp(0, width - 1),
                }

                *img.get_pixel_mut(x, y) = *color;
            }

            // Do children
            let delta = len / 4;
            if delta == 0 {
                should_continue = false;
            }

            len = len - len / 4;

            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };

            if len == 1 {
                should_continue = false;
            }
        }
    }
}
