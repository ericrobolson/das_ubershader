mod data;
mod op;

pub use data::*;
pub use op::*;

use crate::Texture;
use game_utils::collections::Stack;
use image::GenericImageView;

/// Various errors that may occur.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// An invalid type was provided.
    /// TODO: somehow link to required inputs off of Ops.
    InvalidType { got: Data },
    /// The stack was empty and a value was attempted to be popped off.
    /// TODO: somehow link to required inputs off of Ops.
    StackUnderflow,
    /// There was an unhandled token.
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
    /// TODO: test
    pub fn execute(&mut self, op: Op) -> Result<(), Error> {
        match op {
            Op::Data(data) => {
                self.push(data)?;
                Ok(())
            }
            Op::FragPos => {
                self.push(Data::U32(self.x))?;
                self.push(Data::U32(self.y))?;
                Ok(())
            }
            Op::TexturePixel => {
                // TODO: wire up required types in the event of an error.
                let texture_id = self.pop_u32()?;
                let y = self.pop_u32()?;
                let x = self.pop_u32()?;

                let color = self.get_color(texture_id, x, y);
                self.push(Data::Color(color))?;
                Ok(())
            }
        }
    }

    /// TODO: test
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

    /// TODO: tests
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

    /// Creates a new pixel machine.
    pub fn new(x: u32, y: u32, textures: Vec<Texture>) -> Self {
        let stack = Stack::new();
        Self {
            stack,
            textures,
            x,
            y,
        }
    }

    /// Attempts to parse the given token.
    pub fn parse(&self, token: &str) -> Result<Op, Error> {
        match token {
            "fragPos" => Ok(Op::FragPos),
            "texturePixel" => Ok(Op::TexturePixel),
            _ => {
                if let Ok(u) = token.parse::<u32>() {
                    Ok(Op::Data(Data::U32(u)))
                } else if let Ok(b) = token.parse::<bool>() {
                    Ok(Op::Data(Data::Bool(b)))
                } else {
                    Err(Error::UnhandledToken {
                        got: token.to_string(),
                    })
                }
            }
        }
    }

    /// Pops a value off the stack.
    fn pop(&mut self) -> Result<Data, Error> {
        // TODO: wire up required types in the event of an error.

        match self.stack.pop() {
            Some(data) => Ok(data),
            None => Err(Error::StackUnderflow),
        }
    }

    /// Pops a bool off the stack.
    fn pop_bool(&mut self) -> Result<bool, Error> {
        // TODO: wire up required types in the event of an error.

        match self.pop()? {
            Data::Bool(value) => Ok(value),
            data => Err(Error::InvalidType { got: data }),
        }
    }

    /// Pops a color off the stack.
    fn pop_color(&mut self) -> Result<Color, Error> {
        // TODO: wire up required types in the event of an error.

        match self.pop()? {
            Data::Color(value) => Ok(value),
            data => Err(Error::InvalidType { got: data }),
        }
    }

    /// Pops a u32 off the stack
    fn pop_u32(&mut self) -> Result<u32, Error> {
        // TODO: wire up required types in the event of an error.

        match self.pop()? {
            Data::U32(u) => Ok(u),
            data => Err(Error::InvalidType { got: data }),
        }
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, data: Data) -> Result<(), Error> {
        self.stack.push(data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const W: u32 = 640;
    const H: u32 = 480;

    fn machine() -> PixelMachine {
        PixelMachine::new(
            640,
            480,
            vec![std::sync::Arc::new(image::DynamicImage::new_rgba8(W, H))],
        )
    }

    mod parse {
        use super::*;

        #[test]
        fn bool_true() {
            let token = "true";
            assert_eq!(Ok(Op::Data(Data::Bool(true))), machine().parse(token));
        }

        #[test]
        fn bool_false() {
            let token = "false";
            assert_eq!(Ok(Op::Data(Data::Bool(false))), machine().parse(token));
        }

        #[test]
        fn frag_pos() {
            let token = "fragPos";
            assert_eq!(Ok(Op::FragPos), machine().parse(token));
        }

        #[test]
        fn invalid_token() {
            let token = "garbageDay!!!";
            assert_eq!(
                Err(Error::UnhandledToken {
                    got: "garbageDay!!!".into()
                }),
                machine().parse(token)
            );
        }

        #[test]
        fn texture_pixel() {
            let token = "texturePixel";
            assert_eq!(Ok(Op::TexturePixel), machine().parse(token));
        }

        #[test]
        fn u32_valid() {
            let token = "0123";
            assert_eq!(Ok(Op::Data(Data::U32(123))), machine().parse(token));
        }

        #[test]
        fn u32_invalid() {
            let token = "-0123";
            assert_eq!(
                Err(Error::UnhandledToken {
                    got: "-0123".into()
                }),
                machine().parse(token)
            );
        }
    }

    mod pop {
        use super::*;

        // Pop
        #[test]
        fn pop() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();
            assert_eq!(Ok(Data::Bool(true)), m.pop());
        }

        #[test]
        fn pop_returns_underflow() {
            let mut m = machine();
            assert_eq!(Err(Error::StackUnderflow), m.pop());
        }

        // Bool
        #[test]
        fn pop_bool() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();

            assert_eq!(Ok(true), m.pop_bool());
        }

        #[test]
        fn pop_bool_underflow() {
            let mut m = machine();
            assert_eq!(Err(Error::StackUnderflow), m.pop_bool());
        }

        #[test]
        fn pop_bool_wrong_type() {
            let mut m = machine();
            m.push(Data::Color((0, 0, 0).into())).unwrap();

            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::Color((0, 0, 0).into())
                }),
                m.pop_bool()
            );
        }

        // Color
        #[test]
        fn pop_color() {
            let mut m = machine();
            m.push(Data::Color((0, 0, 0).into())).unwrap();
            assert_eq!(Ok((0, 0, 0).into()), m.pop_color());
        }

        #[test]
        fn pop_color_underflow() {
            let mut m = machine();
            assert_eq!(Err(Error::StackUnderflow), m.pop_color());
        }

        #[test]
        fn pop_color_wrong_type() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();

            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::Bool(true)
                }),
                m.pop_color()
            );
        }

        // U32
        #[test]
        fn pop_u32() {
            let mut m = machine();
            m.push(Data::U32(333)).unwrap();
            assert_eq!(Ok(333), m.pop_u32());
        }

        #[test]
        fn pop_u32_underflow() {
            let mut m = machine();
            assert_eq!(Err(Error::StackUnderflow), m.pop_u32());
        }

        #[test]
        fn pop_u32_wrong_type() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();

            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::Bool(true)
                }),
                m.pop_u32()
            );
        }
    }

    mod push {
        use super::*;

        #[test]
        fn push() {
            let mut m = machine();
            let d = Data::Bool(true);
            assert_eq!(Ok(()), m.push(d));
            assert_eq!(Ok(true), m.pop_bool());
        }
    }
}
