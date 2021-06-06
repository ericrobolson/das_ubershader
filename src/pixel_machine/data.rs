/// A value that may live on the stack.
#[derive(Debug, Clone, PartialEq)]
pub enum Data {
    Bool(bool),
    Color(Color),
    String(String),
    U32(u32),
}

impl Data {
    /// Returns the type for the data
    pub fn get_type(&self) -> Type {
        match self {
            Data::Bool(_) => Type::Bool,
            Data::Color(_) => Type::Color,
            Data::U32(_) => Type::U32,
            Data::String(_) => Type::String,
        }
    }
}

/// The various types used in the PixelMachine.
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Any,
    Bool,
    Color,
    String,
    U32,
}

/// A color.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Into<Color> for (u8, u8, u8) {
    fn into(self) -> Color {
        let (r, g, b) = self;

        Color { r, g, b, a: 255 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod data {
        use super::*;

        mod get_type {
            use super::*;

            #[test]
            fn get_type_bool() {
                let d = Data::Bool(true);
                assert_eq!(Type::Bool, d.get_type());
            }

            #[test]
            fn get_type_color() {
                let d = Data::Color((0, 0, 0).into());
                assert_eq!(Type::Color, d.get_type());
            }

            #[test]
            fn get_type_string() {
                let d = Data::String("garbage".into());
                assert_eq!(Type::String, d.get_type());
            }

            #[test]
            fn get_type_u32() {
                let d = Data::U32(22);
                assert_eq!(Type::U32, d.get_type());
            }
        }
    }
}
