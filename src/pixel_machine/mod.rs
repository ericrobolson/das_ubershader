mod data;
mod op;

use std::u8;

pub use data::*;
pub use op::*;

use crate::Texture;
use game_utils::collections::Stack;
use image::GenericImageView;

/// Various errors that may occur.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// A number was attempted to be divided by zero.
    DivideByZero,
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
    width: u32,
    height: u32,
    x: u32,
    y: u32,
}
impl PixelMachine {
    /// TODO: test
    pub fn execute(&mut self, op: Op) -> Result<(), Error> {
        match op {
            Op::Add => {
                let a = self.pop_u32()?;
                let b = self.pop_u32()?;
                self.push(Data::U32(a.wrapping_add(b)))?;
                Ok(())
            }
            Op::Data(data) => {
                self.push(data)?;
                Ok(())
            }
            Op::Dimensions => {
                self.push(Data::U32(self.width))?;
                self.push(Data::U32(self.height))?;

                Ok(())
            }
            Op::Divide => {
                let n = self.pop_u32()?;
                let divisor = self.pop_u32()?;

                if divisor == 0 {
                    return Err(Error::DivideByZero);
                }

                let divided = n / divisor;

                self.push(Data::U32(divided))?;
                Ok(())
            }
            Op::Drop => {
                self.pop()?;
                Ok(())
            }
            Op::Dup => {
                let data = self.pop()?;
                self.push(data.clone())?;
                self.push(data)?;
                Ok(())
            }
            Op::Equal => {
                let a = self.pop()?;
                let b = self.pop()?;

                if a == b {
                    self.push(Data::Bool(true))?;
                } else {
                    // If there are two numbers, convert to u32 and try out those
                    self.push(a)?;
                    self.push(b)?;

                    let a = self.pop_u32();
                    let b = self.pop_u32();
                    if let Ok(a) = a {
                        if let Ok(b) = b {
                            if a == b {
                                self.push(Data::Bool(true))?;
                                return Ok(());
                            }
                        }
                    }

                    self.push(Data::Bool(false))?;
                }
                Ok(())
            }
            Op::FragPos => {
                self.push(Data::U32(self.x))?;
                self.push(Data::U32(self.y))?;
                Ok(())
            }
            Op::GreaterThan => {
                let a = self.pop_u32()?;
                let b = self.pop_u32()?;
                self.push(Data::Bool(b > a))?;
                Ok(())
            }
            Op::GreaterThanEqual => {
                let a = self.pop_u32()?;
                let b = self.pop_u32()?;
                self.push(Data::Bool(b >= a))?;
                Ok(())
            }
            Op::LessThan => {
                let a = self.pop_u32()?;
                let b = self.pop_u32()?;
                self.push(Data::Bool(b < a))?;
                Ok(())
            }
            Op::LessThanEqual => {
                let a = self.pop_u32()?;
                let b = self.pop_u32()?;
                self.push(Data::Bool(b <= a))?;
                Ok(())
            }
            Op::Modulo => {
                let n = self.pop_u32()?;
                let modulus = self.pop_u32()?;

                if modulus == 0 {
                    return Err(Error::DivideByZero);
                }

                let modded = n % modulus;

                self.push(Data::U32(modded))?;
                Ok(())
            }
            Op::MakeColor => {
                let a = self.pop_u8()?;
                let b = self.pop_u8()?;
                let g = self.pop_u8()?;
                let r = self.pop_u8()?;

                let color: Color = (r, g, b, a).into();
                self.push(Data::Color(color))?;
                Ok(())
            }
            Op::Multiply => {
                let multiplier = self.pop_u32()?;
                let n = self.pop_u32()?;

                self.push(Data::U32(n.wrapping_mul(multiplier)))?;
                Ok(())
            }
            Op::Rot => {
                let a = self.pop()?;
                let b = self.pop()?;
                self.push(a)?;
                self.push(b)?;
                Ok(())
            }
            Op::RotN => {
                let n = self.pop_u32()?;

                if n == 0 {
                    return Ok(());
                }

                let mut working_stack = Stack::new();

                let a = self.pop()?;

                for _ in 0..n - 1 {
                    working_stack.push(self.pop()?);
                }

                let b = self.pop()?;

                self.push(a)?;
                while let Some(value) = working_stack.pop() {
                    self.push(value)?;
                }

                self.push(b)?;
                Ok(())
            }
            Op::SplitColor => {
                let color = self.pop_color()?;
                self.push(Data::U8(color.r))?;
                self.push(Data::U8(color.g))?;
                self.push(Data::U8(color.b))?;
                self.push(Data::U8(color.a))?;

                Ok(())
            }
            Op::Subtract => {
                let n = self.pop_u32()?;
                let subtractor = self.pop_u32()?;
                self.push(Data::U32(n.wrapping_sub(subtractor)))?;
                Ok(())
            }
            // TODO: test
            Op::TexturePixel => {
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

        let x = x % self.width;
        let y = y % self.height;

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
    pub fn new(x: u32, y: u32, width: u32, height: u32, textures: Vec<Texture>) -> Self {
        let stack = Stack::new();
        Self {
            stack,
            textures,
            width,
            height,
            x,
            y,
        }
    }

    /// Attempts to parse the given token.
    pub fn parse(&self, token: &str) -> Result<Op, Error> {
        match token {
            "+" => Ok(Op::Add),
            "/" => Ok(Op::Divide),
            "%" => Ok(Op::Modulo),
            "*" => Ok(Op::Multiply),
            "-" => Ok(Op::Subtract),
            "==" => Ok(Op::Equal),
            ">" => Ok(Op::GreaterThan),
            ">=" => Ok(Op::GreaterThanEqual),
            "<" => Ok(Op::LessThan),
            "<=" => Ok(Op::LessThanEqual),
            "dim" => Ok(Op::Dimensions),
            "drop" => Ok(Op::Drop),
            "dup" => Ok(Op::Dup),
            "fragPos" => Ok(Op::FragPos),
            "makeColor" => Ok(Op::MakeColor),
            "rot" => Ok(Op::Rot),
            "rotN" => Ok(Op::RotN),
            "splitColor" => Ok(Op::SplitColor),
            "texturePixel" => Ok(Op::TexturePixel),
            _ => {
                if let Ok(u) = token.parse::<u8>() {
                    Ok(Op::Data(Data::U8(u)))
                } else if let Ok(u) = token.parse::<u32>() {
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

    /// Pops a string off the stack.
    fn pop_string(&mut self) -> Result<String, Error> {
        // TODO: wire up required types in the event of an error.

        match self.pop()? {
            Data::String(value) => Ok(value),
            data => Err(Error::InvalidType { got: data }),
        }
    }

    /// Pops a u32 off the stack
    fn pop_u32(&mut self) -> Result<u32, Error> {
        // TODO: wire up required types in the event of an error.

        match self.pop()? {
            Data::U8(u) => Ok(u as u32),
            Data::U32(u) => Ok(u),
            data => Err(Error::InvalidType { got: data }),
        }
    }

    /// Pops a u8 off the stack
    fn pop_u8(&mut self) -> Result<u8, Error> {
        // TODO: wire up required types in the event of an error.

        match self.pop()? {
            Data::U32(u) => {
                let u = {
                    if u <= u8::MAX as u32 {
                        u as u8
                    } else {
                        (u % (u8::MAX as u32)) as u8
                    }
                };

                Ok(u)
            }
            Data::U8(u) => Ok(u),
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
            320,
            240,
            W,
            H,
            vec![std::sync::Arc::new(image::DynamicImage::new_rgba8(W, H))],
        )
    }

    mod execute {
        use super::*;

        #[test]
        fn add() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::U32(2)).unwrap();
            assert_eq!(Ok(()), m.execute(Op::Add));

            assert_eq!(Ok(4), m.pop_u32());
        }

        #[test]
        fn add_wraps() {
            let mut m = machine();
            m.push(Data::U32(u32::MAX)).unwrap();
            m.push(Data::U32(2)).unwrap();
            assert_eq!(Ok(()), m.execute(Op::Add));

            assert_eq!(Ok(u32::MAX.wrapping_add(2)), m.pop_u32());
        }

        #[test]
        fn add_not_numbers() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::Bool(true)).unwrap();
            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::Bool(true)
                }),
                m.execute(Op::Add)
            );

            let mut m = machine();
            m.push(Data::String("garbage".into())).unwrap();
            m.push(Data::U32(2)).unwrap();
            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::String("garbage".into())
                }),
                m.execute(Op::Add)
            );
        }

        #[test]
        fn data() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();
            m.push(Data::U32(1)).unwrap();
            m.push(Data::String("testy".into())).unwrap();

            assert_eq!("testy".to_string(), m.pop_string().unwrap());
            assert_eq!(1, m.pop_u32().unwrap());
            assert_eq!(true, m.pop_bool().unwrap());
        }

        #[test]
        fn dimensions() {
            let mut m = machine();
            assert_eq!(Ok(()), m.execute(Op::Dimensions));

            assert_eq!(Ok(m.height), m.pop_u32());
            assert_eq!(Ok(m.width), m.pop_u32());
        }

        #[test]
        fn divide() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::U32(8)).unwrap();
            assert_eq!(Ok(()), m.execute(Op::Divide));

            assert_eq!(Ok(4), m.pop_u32());
        }

        #[test]
        fn divide_by_zero() {
            let mut m = machine();
            m.push(Data::U32(0)).unwrap();
            m.push(Data::U32(8)).unwrap();
            assert_eq!(Err(Error::DivideByZero), m.execute(Op::Divide));
        }

        #[test]
        fn divide_not_numbers() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::Bool(true)).unwrap();
            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::Bool(true)
                }),
                m.execute(Op::Divide)
            );

            let mut m = machine();
            m.push(Data::String("garbage".into())).unwrap();
            m.push(Data::U32(2)).unwrap();
            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::String("garbage".into())
                }),
                m.execute(Op::Divide)
            );
        }

        #[test]
        fn drop_no_stack_returns_err() {
            assert_eq!(Err(Error::StackUnderflow), machine().execute(Op::Drop));
        }

        #[test]
        fn drop() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();
            assert_eq!(Ok(()), m.execute(Op::Drop));
            assert_eq!(None, m.stack.peek());
        }

        #[test]
        fn dup_no_stack_returns_err() {
            assert_eq!(Err(Error::StackUnderflow), machine().execute(Op::Dup));
        }

        #[test]
        fn dup_duplicates_top() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();
            m.execute(Op::Dup).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());
            assert_eq!(true, m.pop_bool().unwrap());
        }

        #[test]
        fn eq_true() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();
            m.push(Data::Bool(true)).unwrap();
            m.execute(Op::Equal).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U8(18)).unwrap();
            m.push(Data::U32(18)).unwrap();
            m.execute(Op::Equal).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::String("wut".into())).unwrap();
            m.push(Data::String("wut".into())).unwrap();
            m.execute(Op::Equal).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());
        }

        #[test]
        fn eq_false() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();
            m.push(Data::Bool(false)).unwrap();
            m.execute(Op::Equal).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());

            m.push(Data::U8(12)).unwrap();
            m.push(Data::U32(18)).unwrap();
            m.execute(Op::Equal).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());

            m.push(Data::String("wut".into())).unwrap();
            m.push(Data::String("w2ut".into())).unwrap();
            m.execute(Op::Equal).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());
        }

        #[test]
        fn frag_pos() {
            let mut m = machine();
            m.execute(Op::FragPos).unwrap();
            let expected_x = m.x;
            let expected_y = m.y;
            assert_eq!(expected_y, m.pop_u32().unwrap());
            assert_eq!(expected_x, m.pop_u32().unwrap());
        }

        #[test]
        fn greater_than_true() {
            let mut m = machine();
            m.push(Data::U8(3)).unwrap();
            m.push(Data::U32(2)).unwrap();
            m.execute(Op::GreaterThan).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U32(18)).unwrap();
            m.push(Data::U32(1)).unwrap();
            m.execute(Op::GreaterThan).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U8(5)).unwrap();
            m.push(Data::U8(2)).unwrap();
            m.execute(Op::GreaterThan).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());
        }

        #[test]
        fn greater_than_false() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::U8(3)).unwrap();
            m.execute(Op::GreaterThan).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());

            m.push(Data::U32(1)).unwrap();
            m.push(Data::U32(18)).unwrap();
            m.execute(Op::GreaterThan).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());

            m.push(Data::U8(2)).unwrap();
            m.push(Data::U8(5)).unwrap();
            m.execute(Op::GreaterThan).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());
        }

        #[test]
        fn greater_than_equal_true() {
            let mut m = machine();
            m.push(Data::U8(3)).unwrap();
            m.push(Data::U32(2)).unwrap();
            m.execute(Op::GreaterThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U32(18)).unwrap();
            m.push(Data::U32(1)).unwrap();
            m.execute(Op::GreaterThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U8(5)).unwrap();
            m.push(Data::U8(2)).unwrap();
            m.execute(Op::GreaterThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U8(3)).unwrap();
            m.push(Data::U32(3)).unwrap();
            m.execute(Op::GreaterThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U32(18)).unwrap();
            m.push(Data::U32(18)).unwrap();
            m.execute(Op::GreaterThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U8(5)).unwrap();
            m.push(Data::U8(5)).unwrap();
            m.execute(Op::GreaterThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());
        }

        #[test]
        fn greater_than_equal_false() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::U8(3)).unwrap();
            m.execute(Op::GreaterThanEqual).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());

            m.push(Data::U32(1)).unwrap();
            m.push(Data::U32(18)).unwrap();
            m.execute(Op::GreaterThanEqual).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());

            m.push(Data::U8(2)).unwrap();
            m.push(Data::U8(5)).unwrap();
            m.execute(Op::GreaterThanEqual).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());
        }

        #[test]
        fn less_than_true() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::U8(3)).unwrap();
            m.execute(Op::LessThan).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U32(1)).unwrap();
            m.push(Data::U32(18)).unwrap();
            m.execute(Op::LessThan).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U8(2)).unwrap();
            m.push(Data::U8(5)).unwrap();
            m.execute(Op::LessThan).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());
        }

        #[test]
        fn less_than_false() {
            let mut m = machine();
            m.push(Data::U8(3)).unwrap();
            m.push(Data::U32(2)).unwrap();
            m.execute(Op::LessThan).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());

            m.push(Data::U32(18)).unwrap();
            m.push(Data::U32(1)).unwrap();
            m.execute(Op::LessThan).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());

            m.push(Data::U8(5)).unwrap();
            m.push(Data::U8(2)).unwrap();
            m.execute(Op::LessThan).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());
        }

        #[test]
        fn less_than_equal_true() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::U8(3)).unwrap();
            m.execute(Op::LessThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U32(1)).unwrap();
            m.push(Data::U32(18)).unwrap();
            m.execute(Op::LessThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U8(2)).unwrap();
            m.push(Data::U8(5)).unwrap();
            m.execute(Op::LessThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U8(3)).unwrap();
            m.push(Data::U32(3)).unwrap();
            m.execute(Op::LessThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U32(18)).unwrap();
            m.push(Data::U32(18)).unwrap();
            m.execute(Op::LessThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());

            m.push(Data::U8(5)).unwrap();
            m.push(Data::U8(5)).unwrap();
            m.execute(Op::LessThanEqual).unwrap();

            assert_eq!(true, m.pop_bool().unwrap());
        }

        #[test]
        fn less_than_equal_false() {
            let mut m = machine();
            m.push(Data::U8(3)).unwrap();
            m.push(Data::U32(2)).unwrap();
            m.execute(Op::LessThanEqual).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());

            m.push(Data::U32(18)).unwrap();
            m.push(Data::U32(1)).unwrap();
            m.execute(Op::LessThanEqual).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());

            m.push(Data::U8(5)).unwrap();
            m.push(Data::U8(2)).unwrap();
            m.execute(Op::LessThanEqual).unwrap();

            assert_eq!(false, m.pop_bool().unwrap());
        }

        #[test]
        fn make_color() {
            let mut m = machine();
            m.push(Data::U8(1)).unwrap();
            m.push(Data::U8(2)).unwrap();
            m.push(Data::U8(3)).unwrap();
            m.push(Data::U8(4)).unwrap();

            m.execute(Op::MakeColor).unwrap();
            let expected: Color = (1, 2, 3, 4).into();
            assert_eq!(expected, m.pop_color().unwrap());
        }

        #[test]
        fn modulo() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::U32(7)).unwrap();
            assert_eq!(Ok(()), m.execute(Op::Modulo));

            assert_eq!(Ok(1), m.pop_u32());
        }

        #[test]
        fn modulo_by_zero() {
            let mut m = machine();
            m.push(Data::U32(0)).unwrap();
            m.push(Data::U32(8)).unwrap();
            assert_eq!(Err(Error::DivideByZero), m.execute(Op::Modulo));
        }

        #[test]
        fn modulo_not_numbers() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::Bool(true)).unwrap();
            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::Bool(true)
                }),
                m.execute(Op::Divide)
            );

            let mut m = machine();
            m.push(Data::String("garbage".into())).unwrap();
            m.push(Data::U32(2)).unwrap();
            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::String("garbage".into())
                }),
                m.execute(Op::Modulo)
            );
        }

        #[test]
        fn multiply() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::U32(4)).unwrap();
            assert_eq!(Ok(()), m.execute(Op::Multiply));

            assert_eq!(Ok(8), m.pop_u32());
        }

        #[test]
        fn multiply_wraps() {
            let mut m = machine();
            m.push(Data::U32(u32::MAX)).unwrap();
            m.push(Data::U32(3)).unwrap();
            assert_eq!(Ok(()), m.execute(Op::Multiply));

            assert_eq!(Ok(u32::MAX.wrapping_mul(3)), m.pop_u32());
        }

        #[test]
        fn rot_0_element_underflows() {
            let mut m = machine();

            assert_eq!(Err(Error::StackUnderflow), m.execute(Op::Rot));
        }

        #[test]
        fn rot_1_element_underflows() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();

            assert_eq!(Err(Error::StackUnderflow), m.execute(Op::Rot));
        }

        #[test]
        fn rot() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();
            m.push(Data::Bool(false)).unwrap();
            m.execute(Op::Rot).unwrap();
            assert_eq!(true, m.pop_bool().unwrap());
            assert_eq!(false, m.pop_bool().unwrap());
        }

        #[test]
        fn rot_n() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();
            m.push(Data::U32(123)).unwrap();
            m.push(Data::Bool(false)).unwrap();
            m.push(Data::U32(2)).unwrap();
            m.execute(Op::RotN).unwrap();
            assert_eq!(true, m.pop_bool().unwrap());
            assert_eq!(123, m.pop_u32().unwrap());
            assert_eq!(false, m.pop_bool().unwrap());
        }

        #[test]
        fn rot_n_when_n_0() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();
            m.push(Data::U32(123)).unwrap();
            m.push(Data::Bool(false)).unwrap();
            m.push(Data::U32(0)).unwrap();
            m.execute(Op::RotN).unwrap();
            assert_eq!(false, m.pop_bool().unwrap());
            assert_eq!(123, m.pop_u32().unwrap());
            assert_eq!(true, m.pop_bool().unwrap());
        }

        #[test]
        fn split_color_underflow() {
            let mut m = machine();

            assert_eq!(Err(Error::StackUnderflow), m.execute(Op::SplitColor));
        }

        #[test]
        fn split_color() {
            let mut m = machine();
            m.push(Data::Color((0, 1, 2, 3).into())).unwrap();
            assert_eq!(Ok(()), m.execute(Op::SplitColor));
            assert_eq!(3, m.pop_u8().unwrap());
            assert_eq!(2, m.pop_u8().unwrap());
            assert_eq!(1, m.pop_u8().unwrap());
            assert_eq!(0, m.pop_u8().unwrap());
        }

        #[test]
        fn subtract() {
            let mut m = machine();
            m.push(Data::U32(7)).unwrap();
            m.push(Data::U32(9)).unwrap();
            assert_eq!(Ok(()), m.execute(Op::Subtract));

            assert_eq!(Ok(2), m.pop_u32());
        }

        #[test]
        fn sub_wraps() {
            let mut m = machine();
            m.push(Data::U32(9)).unwrap();
            m.push(Data::U32(2)).unwrap();
            assert_eq!(Ok(()), m.execute(Op::Subtract));
            assert_eq!(Ok((2 as u32).wrapping_sub(9)), m.pop_u32());
        }

        #[test]
        fn subtract_not_numbers() {
            let mut m = machine();
            m.push(Data::U32(2)).unwrap();
            m.push(Data::Bool(true)).unwrap();
            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::Bool(true)
                }),
                m.execute(Op::Divide)
            );

            let mut m = machine();
            m.push(Data::String("garbage".into())).unwrap();
            m.push(Data::U32(2)).unwrap();
            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::String("garbage".into())
                }),
                m.execute(Op::Subtract)
            );
        }
    }

    mod parse {
        use super::*;

        #[test]
        fn add() {
            let token = "+";
            assert_eq!(Ok(Op::Add), machine().parse(token));
        }

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
        fn dimensions() {
            let token = "dim";
            assert_eq!(Ok(Op::Dimensions), machine().parse(token));
        }

        #[test]
        fn divide() {
            let token = "/";
            assert_eq!(Ok(Op::Divide), machine().parse(token));
        }

        #[test]
        fn drop() {
            let token = "drop";
            assert_eq!(Ok(Op::Drop), machine().parse(token));
        }

        #[test]
        fn dup() {
            let token = "dup";
            assert_eq!(Ok(Op::Dup), machine().parse(token));
        }

        #[test]
        fn eq() {
            let token = "==";
            assert_eq!(Ok(Op::Equal), machine().parse(token));
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
        fn greater_than() {
            let token = ">";
            assert_eq!(Ok(Op::GreaterThan), machine().parse(token));
        }

        #[test]
        fn greater_than_equal() {
            let token = ">=";
            assert_eq!(Ok(Op::GreaterThanEqual), machine().parse(token));
        }

        #[test]
        fn less_than() {
            let token = "<";
            assert_eq!(Ok(Op::LessThan), machine().parse(token));
        }

        #[test]
        fn less_than_equal() {
            let token = "<=";
            assert_eq!(Ok(Op::LessThanEqual), machine().parse(token));
        }

        #[test]
        fn make_color() {
            let token = "makeColor";
            assert_eq!(Ok(Op::MakeColor), machine().parse(token));
        }

        #[test]
        fn modulo() {
            let token = "%";
            assert_eq!(Ok(Op::Modulo), machine().parse(token));
        }

        #[test]
        fn multiply() {
            let token = "*";
            assert_eq!(Ok(Op::Multiply), machine().parse(token));
        }

        #[test]
        fn rot() {
            let token = "rot";
            assert_eq!(Ok(Op::Rot), machine().parse(token));
        }

        #[test]
        fn rot_n() {
            let token = "rotN";
            assert_eq!(Ok(Op::RotN), machine().parse(token));
        }

        #[test]
        fn subtract() {
            let token = "-";
            assert_eq!(Ok(Op::Subtract), machine().parse(token));
        }

        #[test]
        fn texture_pixel() {
            let token = "texturePixel";
            assert_eq!(Ok(Op::TexturePixel), machine().parse(token));
        }

        #[test]
        fn split_color() {
            let token = "splitColor";
            assert_eq!(Ok(Op::SplitColor), machine().parse(token));
        }

        #[test]
        fn u8_valid() {
            let token = "0123";
            assert_eq!(Ok(Op::Data(Data::U8(123))), machine().parse(token));
        }

        #[test]
        fn u32_valid() {
            let token = "266";
            assert_eq!(Ok(Op::Data(Data::U32(266))), machine().parse(token));
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

        // String
        #[test]
        fn pop_string() {
            let mut m = machine();
            m.push(Data::String("abc123".to_string())).unwrap();
            assert_eq!(Ok("abc123".to_string()), m.pop_string());
        }

        #[test]
        fn pop_string_underflow() {
            let mut m = machine();
            assert_eq!(Err(Error::StackUnderflow), m.pop_string());
        }

        #[test]
        fn pop_string_wrong_type() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();

            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::Bool(true)
                }),
                m.pop_string()
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
        fn pop_u32_converts_u8() {
            let mut m = machine();
            m.push(Data::U8(3)).unwrap();
            assert_eq!(Ok(3), m.pop_u32());
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

        // U8
        #[test]
        fn pop_u8() {
            let mut m = machine();
            m.push(Data::U8(2)).unwrap();
            assert_eq!(Ok(2), m.pop_u8());
        }

        #[test]
        fn pop_u8_u32_fits() {
            let mut m = machine();
            m.push(Data::U32(255)).unwrap();
            assert_eq!(Ok(255), m.pop_u8());
        }

        #[test]
        fn pop_u8_u32_doesnt_fit_modulos() {
            let mut m = machine();
            m.push(Data::U32(258)).unwrap();
            assert_eq!(Ok(3), m.pop_u8());
        }

        #[test]
        fn pop_u8_underflow() {
            let mut m = machine();
            assert_eq!(Err(Error::StackUnderflow), m.pop_u8());
        }

        #[test]
        fn pop_u8_wrong_type() {
            let mut m = machine();
            m.push(Data::Bool(true)).unwrap();

            assert_eq!(
                Err(Error::InvalidType {
                    got: Data::Bool(true)
                }),
                m.pop_u8()
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
