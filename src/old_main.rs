type Image = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
use core::num;

use image::GenericImageView;
mod pixel_machine;

fn old_main() {
    let loaded_img = image::open("test.jpg").unwrap();
    let (width, height) = loaded_img.dimensions();

    let mut turtler = Turtler::new(width, height);

    let line_w = 10;
    let line_h = 20;

    let x_cutoff = 4;
    let y_cutoff = 2;

    // Must always be above 0 due to division
    let steps = 5;

    // Layer 0
    for x in 0..width {
        for y in 0..height {
            turtler.draw_pixel(color(0, 0, 0), x, y);
        }
    }

    // Layer 1
    for x in 0..width {
        if x % line_w == 0 {
            for y in 0..height {
                if y % line_h == 0 {
                    let mut count = 0;
                    let mut avg_r = 0;
                    let mut avg_g = 0;
                    let mut avg_b = 0;

                    // Calculate avg

                    let x_range = x..x + line_w;
                    let y_range = y..y + line_h;

                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            let x = x % turtler.width;
                            let y = y % turtler.height;

                            let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                            avg_r += r as u32;
                            avg_g += g as u32;
                            avg_b += b as u32;
                            count += 1;
                        }
                    }

                    // Get average
                    let avg_color = color(
                        (avg_r / count) as u8,
                        (avg_g / count) as u8,
                        (avg_b / count) as u8,
                    );

                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            if x % line_w < x_cutoff || y % line_h < y_cutoff {
                                continue;
                            }

                            turtler.draw_pixel(avg_color, x, y);
                        }
                    }
                }
            }
        }
        // Do some really wack stuff now
        for x in 0..width {
            for y in 0..height {
                let [r, g, b, a] = turtler.output_buffer.get_pixel(x, y).0;
                let step_conversion = steps * (u8::MAX / steps.max(1));
                let converted_r = u8::MAX - b;
                let converted_g = u8::MAX - r;
                let converted_b = u8::MAX - b;

                if r > 0 {
                    turtler.draw_pixel(color(converted_r, converted_g, converted_b), x, y);
                }
            }
        }

        // Do some other stuff
        for x in 0..width {
            if x % 2 != 0 {
                for y in 0..height {
                    if y % 2 != 0 {
                        let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                        let converted_r = u8::MAX - b;
                        let converted_g = u8::MAX - r;
                        let converted_b = u8::MAX - b;

                        if r > 0 {
                            turtler.draw_pixel(color(converted_r, converted_g, converted_b), x, y);
                        }
                    }
                }
            } else {
                for y in 0..height {
                    if y % 3 == 0 && y % 2 != 0 {
                        let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                        let converted_r = u8::MAX - b;
                        let converted_g = u8::MAX - r;
                        let converted_b = u8::MAX - b;
                        turtler.draw_pixel(color(converted_r, converted_g, converted_b), x, y);
                    }
                }
            }

            for y in 0..height {
                let [r, g, b, a] = turtler.output_buffer.get_pixel(x, y).0;

                if r == 0 && (g == 0 || b == 0) {
                    turtler.draw_pixel(color(255 / 3, 255 / 3, 255), x, y);
                }
            }
        }

        for y in 0..height {
            let [r0, g0, b0, a0] = loaded_img.get_pixel(x, y).0;
            let [r1, g1, b1, a1] = turtler.output_buffer.get_pixel(x, y).0;

            let r = ((r0 as u32 + r1 as u32) / 2) as u8;
            let g = ((g0 as u32 + g1 as u32) / 2) as u8;
            let b = ((b0 as u32 + b1 as u32) / 2) as u8;
            turtler.draw_pixel(color(r, g, b), x, y);
        }

        for y in 0..height {
            let [r0, g0, b0, a0] = loaded_img.get_pixel(x, y).0;
            let [r1, g1, b1, a1] = turtler.output_buffer.get_pixel(x, y).0;

            let r = (((r0 as u32) * 5 + r1 as u32) / 5) as u8;
            let g = (((g0 as u32) * 5 + g1 as u32) / 5) as u8;
            let b = (((b0 as u32) * 5 + b1 as u32) / 5) as u8;
            turtler.draw_pixel(color(r, g, b), x, y);
        }
    }

    // Normalish picture
    for x in 0..width {
        if x % line_w == 0 {
            for y in 0..height {
                if y % line_h == 0 {
                    let mut count = 0;
                    let mut avg_r = 0;
                    let mut avg_g = 0;
                    let mut avg_b = 0;

                    // Calculate avg

                    let x_range = x..x + line_w;
                    let y_range = y..y + line_h;

                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            let x = x % turtler.width;
                            let y = y % turtler.height;

                            let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                            avg_r += r as u32;
                            avg_g += g as u32;
                            avg_b += b as u32;
                            count += 1;
                        }
                    }

                    // Get average
                    let avg_color = color(
                        (avg_r / count) as u8,
                        (avg_g / count) as u8,
                        (avg_b / count) as u8,
                    );

                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            if x % line_w < x_cutoff || y % line_h < y_cutoff {
                                continue;
                            }

                            turtler.draw_pixel(avg_color, x, y);
                        }
                    }
                }
            }
        }
    }

    let line_h = 2;
    let line_w = 2;
    for x in 0..width {
        if x % line_w == 0 {
            for y in 0..height {
                if y % line_h == 0 {
                    let mut count = 0;
                    let mut avg_r = 0;
                    let mut avg_g = 0;
                    let mut avg_b = 0;

                    // Calculate avg

                    let x_range = x..x + line_w;
                    let y_range = y..y + line_h;

                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            let x = x % turtler.width;
                            let y = y % turtler.height;

                            let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                            avg_r += r as u32;
                            avg_g += g as u32;
                            avg_b += b as u32;
                            count += 1;
                        }
                    }

                    // Get average
                    let avg_color = color(
                        (avg_r / count) as u8,
                        (avg_g / count) as u8,
                        (avg_b / count) as u8,
                    );

                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            if x % line_w < x_cutoff || y % line_h < y_cutoff {
                                continue;
                            }

                            turtler.draw_pixel(avg_color, x, y);
                        }
                    }
                }
            }
        }
        // Do some really wack stuff now
        for x in 0..width {
            for y in 0..height {
                let [r, g, b, a] = turtler.output_buffer.get_pixel(x, y).0;
                let step_conversion = steps * (u8::MAX / steps.max(1));
                let converted_r = u8::MAX - b;
                let converted_g = u8::MAX - r;
                let converted_b = u8::MAX - b;

                if r > 0 {
                    turtler.draw_pixel(color(converted_r, converted_g, converted_b), x, y);
                }
            }
        }

        // Do some other stuff
        for x in 0..width {
            if x % 2 != 0 {
                for y in 0..height {
                    if y % 2 != 0 {
                        let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                        let converted_r = u8::MAX - g;
                        let converted_g = u8::MAX - b;
                        let converted_b = u8::MAX - r;

                        if r > 0 {
                            turtler.draw_pixel(color(converted_r, converted_g, converted_b), x, y);
                        }
                    }
                }
            } else {
                for y in 0..height {
                    if y % 3 == 0 && y % 2 != 0 {
                        let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                        let converted_r = u8::MAX - b;
                        let converted_g = u8::MAX - r;
                        let converted_b = u8::MAX - g;
                        turtler.draw_pixel(color(converted_r, converted_g, converted_b), x, y);
                    }
                }
            }

            for y in 0..height {
                let [r, g, b, a] = turtler.output_buffer.get_pixel(x, y).0;

                if r == 0 && (g == 0 || b == 0) {
                    turtler.draw_pixel(color(255 / 3, 255 / 3, 255), x, y);
                }
            }
        }
    }

    // Normalish picture
    for x in 0..width {
        if x % 2 == 0 {
            continue;
        }
        for y in 0..height {
            if y % 2 == 0 {
                continue;
            }
            let (r, g, b, a) = {
                if x % 3 == 0 || y % 3 == 0 {
                    let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                    (r, g, b, a)
                } else {
                    let [r0, g0, b0, a0] = loaded_img.get_pixel(x, y).0;
                    let [r1, g1, b1, a1] = turtler.output_buffer.get_pixel(x, y).0;

                    let r = (((r0 as u32) * 5 + r1 as u32) / 5) as u8;
                    let g = (((g0 as u32) * 5 + g1 as u32) / 5) as u8;
                    let b = (((b0 as u32) * 5 + b1 as u32) / 5) as u8;
                    (r, g, b, a0)
                }
            };

            turtler.draw_pixel(color(r, g, b), x, y);
        }
    }

    for x in 0..width {
        for y in 0..height {
            let (r, g, b, a) = {
                if x % 2 == 0 && y % 2 == 0 {
                    let [r0, g0, b0, a0] = turtler.output_buffer.get_pixel(x, y).0;
                    let [r1, g1, b1, a1] = loaded_img.get_pixel(x, y).0;

                    let r = (((r0 as u32) * 5 + r1 as u32) / 5) as u8;
                    let g = (((g0 as u32) * 5 + g1 as u32) / 5) as u8;
                    let b = (((b0 as u32) * 5 + b1 as u32) / 5) as u8;
                    (r, g, b, a0)
                } else {
                    let [r0, g0, b0, a0] = loaded_img.get_pixel(x, y).0;
                    let [r1, g1, b1, a1] = turtler.output_buffer.get_pixel(x, y).0;

                    let r = (((r0 as u32) * 5 + r1 as u32) / 5) as u8;
                    let g = (((g0 as u32) * 5 + g1 as u32) / 5) as u8;
                    let b = (((b0 as u32) * 5 + b1 as u32) / 5) as u8;
                    (r, g, b, a0)
                }
            };

            turtler.draw_pixel(color(r, g, b), x, y);
        }
    }

    for x in 0..width {
        for y in 0..height {
            let (r, g, b, a) = {
                {
                    let [r0, g0, b0, a0] = loaded_img.get_pixel(x, y).0;
                    let [r1, g1, b1, a1] = turtler.output_buffer.get_pixel(x, y).0;

                    let r = (((r0 as u32) * 5 + r1 as u32) / 5) as u8;
                    let g = (((g0 as u32) * 5 + g1 as u32) / 5) as u8;
                    let b = (((b0 as u32) * 5 + b1 as u32) / 5) as u8;
                    (r, g, b, a0)
                }
            };

            turtler.draw_pixel(color(r, g, b), x + 5, y + 5);
            turtler.draw_pixel(color(r, g, b), x - 5, y - 5);
        }
    }

    // Normalish picture
    for x in 0..width {
        if x % line_w == 0 {
            for y in 0..height {
                if y % line_h == 0 {
                    let mut count = 0;
                    let mut avg_r = 0;
                    let mut avg_g = 0;
                    let mut avg_b = 0;

                    // Calculate avg

                    let x_range = x..x + line_w;
                    let y_range = y..y + line_h;

                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            let x = x % turtler.width;
                            let y = y % turtler.height;

                            let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                            avg_r += r as u32;
                            avg_g += g as u32;
                            avg_b += b as u32;
                            count += 1;
                        }
                    }

                    // Get average
                    let avg_color = color(
                        (avg_r / count) as u8,
                        (avg_g / count) as u8,
                        (avg_b / count) as u8,
                    );

                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            if x % line_w < x_cutoff || y % line_h < y_cutoff {
                                continue;
                            }

                            turtler.draw_pixel(avg_color, x, y);
                        }
                    }
                }
            }
        }
    }

    // Layer 1
    for x in 0..width {
        if x % line_w == 0 {
            for y in 0..height {
                if y % line_h == 0 {
                    let mut count = 0;
                    let mut avg_r = 0;
                    let mut avg_g = 0;
                    let mut avg_b = 0;

                    // Calculate avg

                    let x_range = x..x + line_w;
                    let y_range = y..y + line_h;

                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            let x = x % turtler.width;
                            let y = y % turtler.height;

                            let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                            avg_r += r as u32;
                            avg_g += g as u32;
                            avg_b += b as u32;
                            count += 1;
                        }
                    }

                    // Get average
                    let avg_color = color(
                        (avg_r / count) as u8,
                        (avg_g / count) as u8,
                        (avg_b / count) as u8,
                    );

                    for x in x_range.clone() {
                        for y in y_range.clone() {
                            if x % line_w < x_cutoff || y % line_h < y_cutoff {
                                continue;
                            }

                            turtler.draw_pixel(avg_color, x, y);
                        }
                    }
                }
            }
        }
        // Do some really wack stuff now
        for x in 0..width {
            for y in 0..height {
                let [r, g, b, a] = turtler.output_buffer.get_pixel(x, y).0;
                let step_conversion = steps * (u8::MAX / steps.max(1));
                let converted_r = u8::MAX - b;
                let converted_g = u8::MAX - r;
                let converted_b = u8::MAX - b;

                if r > 0 {
                    turtler.draw_pixel(color(converted_r, converted_g, converted_b), x, y);
                }
            }
        }

        // Do some other stuff
        for x in 0..width {
            if x % 2 != 0 {
                for y in 0..height {
                    if y % 2 != 0 {
                        let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                        let converted_r = u8::MAX - b;
                        let converted_g = u8::MAX - r;
                        let converted_b = u8::MAX - b;

                        if r > 0 {
                            turtler.draw_pixel(color(converted_r, converted_g, converted_b), x, y);
                        }
                    }
                }
            } else {
                for y in 0..height {
                    if y % 3 == 0 && y % 2 != 0 {
                        let [r, g, b, a] = loaded_img.get_pixel(x, y).0;
                        let converted_r = u8::MAX - b;
                        let converted_g = u8::MAX - r;
                        let converted_b = u8::MAX - b;
                        turtler.draw_pixel(color(converted_r, converted_g, converted_b), x, y);
                    }
                }
            }

            for y in 0..height {
                let [r, g, b, a] = turtler.output_buffer.get_pixel(x, y).0;

                if r == 0 && (g == 0 || b == 0) {
                    turtler.draw_pixel(color(255 / 3, 255 / 3, 255), x, y);
                }
            }
        }

        for y in 0..height {
            let [r0, g0, b0, a0] = loaded_img.get_pixel(x, y).0;
            let [r1, g1, b1, a1] = turtler.output_buffer.get_pixel(x, y).0;

            let r = ((r0 as u32 + r1 as u32) / 2) as u8;
            let g = ((g0 as u32 + g1 as u32) / 2) as u8;
            let b = ((b0 as u32 + b1 as u32) / 2) as u8;
            turtler.draw_pixel(color(r, g, b), x, y);
        }

        for y in 0..height {
            let [r0, g0, b0, a0] = loaded_img.get_pixel(x, y).0;
            let [r1, g1, b1, a1] = turtler.output_buffer.get_pixel(x, y).0;

            let r = (((r0 as u32) * 42 + r1 as u32) / 45) as u8;
            let g = (((g0 as u32) * 15 + g1 as u32) / 35) as u8;
            let b = (((b0 as u32) * 25 + b1 as u32) / 25) as u8;
            turtler.draw_pixel(color(r, g, b), x, y);
        }
    }

    for x in 0..width {
        for y in 0..height {
            let [r, g, b, a] = turtler.output_buffer.get_pixel(x, y).0;
            turtler.draw_pixel(color(u8::MAX - r, g, b), x, y);
        }
    }

    for x in 0..width {
        for y in 0..height {
            let [r, g, b, a] = turtler.output_buffer.get_pixel(x, y).0;
            turtler.draw_pixel(color(r, u8::MAX - g, b), x, y);
        }
    }

    for x in 0..width {
        for y in 0..height {
            let [r, g, b, a] = turtler.output_buffer.get_pixel(x, y).0;

            let cola = color(r, g, b);
            let colb = color(u8::MAX - r, u8::MAX - g, b);

            let [r0, g0, b0, a0] = cola.0;
            let [r1, g1, b1, a1] = colb.0;

            let r = (((r0 as u32) * 42 + r1 as u32) / 45) as u8;
            let g = (((g0 as u32) * 15 + g1 as u32) / 35) as u8;
            let b = (((b0 as u32) * 25 + b1 as u32) / 25) as u8;
            turtler.draw_pixel(color(r, g, b), x, y);
        }
    }

    for x in 0..width {
        if x % 4 == 0 {
            for y in 0..height {
                if y % 4 == 0 {
                    let [r, g, b, a] = loaded_img.get_pixel(x, y).0;

                    let cola = color(r, g, b);
                    let colb = color(u8::MAX - r, u8::MAX - g, b);

                    let [r0, g0, b0, a0] = cola.0;
                    let [r1, g1, b1, a1] = colb.0;

                    let r = (((r0 as u32) * 42 + r1 as u32) / 45) as u8;
                    let g = (((g0 as u32) * 15 + g1 as u32) / 35) as u8;
                    let b = (((b0 as u32) * 25 + b1 as u32) / 25) as u8;
                    turtler.draw_pixel(color(r, g, b), x, y);
                }
            }
        }
    }

    // Shade it
    for x in 0..width {
        for y in 0..height {
            if x % 4 != 0 && y % 4 != 0 {
                continue;
            }
            let [r, g, b, a] = loaded_img.get_pixel(x, y).0;

            turtler.draw_pixel(color(r, g, b), x, y);
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

    pub fn draw_pixel(&mut self, color: Color, x: u32, y: u32) {
        let x = x % self.width;
        let y = y % self.height;

        *self.output_buffer.get_pixel_mut(x, y) = color;
    }

    pub fn draw_line(&mut self, color: Color, x0: u32, y0: u32, slope: u32, len: u32) {
        let mut x = x0;
        let mut y = y0;
        for l in 0..len {
            for _ in 0..slope {
                self.draw_pixel(color, x, y);

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
