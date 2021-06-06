use game_utils::collections::Stack;

mod data;
mod op;
pub use data::*;
use image::GenericImageView;
pub use op::*;

use crate::Texture;

#[derive(Debug)]
pub enum Error {
    InvalidType { got: Data },
    StackUnderflow,
    UnhandledToken { got: String },
}

/// A virtual machine that operates on a pixel.
pub struct PixelMachine {
    stack: Stack<Data>,
    textures: Vec<Texture>,
    x: u32,
    y: u32,
}
impl PixelMachine {
    pub fn new(x: u32, y: u32, textures: Vec<Texture>) -> Self {
        let stack = Stack::new();
        Self {
            stack,
            textures,
            x,
            y,
        }
    }

    ///
    pub fn interpret(&mut self, program: &str) -> Result<Color, Error> {
        // Replace all comments
        let program = {
            const COMMENT_END: &'static str = "\n";
            const COMMENT_START: &'static str = "#";

            let mut s = program.to_string().replace("\r\n", "\n");

            let mut i = 0;
            let mut removing_comment = false;
            while i < s.len() + 1 {
                if s.get(i..i + 1).unwrap_or_default() == COMMENT_START {
                    removing_comment = true;
                } else if s.get(i..i + 1).unwrap_or_default() == COMMENT_END {
                    removing_comment = false;
                }

                if removing_comment {
                    s.remove(i);
                } else {
                    i += 1;
                }
            }
            s
        };

        // Interpret the program
        for token in program.split_whitespace() {
            let op = self.parse(token)?;
            self.execute(op)?;
        }

        // Return the final color
        self.pop_color()
    }

    pub fn parse(&self, token: &str) -> Result<Op, Error> {
        match token {
            "fragPos" => Ok(Op::FragPos),
            "texturePixel" => Ok(Op::GetPixel),
            _ => {
                if let Ok(u) = token.parse::<u32>() {
                    Ok(Op::Data(Data::U32(u)))
                } else {
                    Err(Error::UnhandledToken {
                        got: token.to_string(),
                    })
                }
            }
        }
    }

    pub fn push(&mut self, data: Data) -> Result<(), Error> {
        self.stack.push(data);
        Ok(())
    }

    fn get_color(&self, texture_id: u32, x: u32, y: u32) -> Color {
        if self.textures.is_empty() {
            return Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            };
        }

        let texture_id = texture_id as usize;
        let texture = texture_id % self.textures.len();

        let [r, g, b, a] = self.textures[texture].get_pixel(x, y).0;

        Color { r, g, b, a }
    }

    pub fn execute(&mut self, op: Op) -> Result<(), Error> {
        match op {
            Op::Data(data) => {
                self.push(data)?;
                Ok(())
            }
            Op::GetPixel => {
                let texture_id = self.pop_u32()?;
                let y = self.pop_u32()?;
                let x = self.pop_u32()?;

                let color = self.get_color(texture_id, x, y);
                self.push(Data::Color(color))?;
                Ok(())
            }
            Op::FragPos => {
                self.push(Data::U32(self.x))?;
                self.push(Data::U32(self.y))?;
                Ok(())
            }
        }
    }

    fn pop(&mut self) -> Result<Data, Error> {
        match self.stack.pop() {
            Some(data) => Ok(data),
            None => Err(Error::StackUnderflow),
        }
    }

    fn pop_u32(&mut self) -> Result<u32, Error> {
        match self.pop()? {
            Data::U32(u) => Ok(u),
            data => Err(Error::InvalidType { got: data }),
        }
    }

    fn pop_color(&mut self) -> Result<Color, Error> {
        match self.pop()? {
            Data::Color(value) => Ok(value),
            data => Err(Error::InvalidType { got: data }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
