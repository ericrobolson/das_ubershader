use crate::{Data, Type};

/// The list of available operations that may occur
/// `[] -> []` represents something that is performed on a stack and returns a stack.
/// `%{} -> %{}` represents the dictionary and something that is done there.
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    /// Pushes some data onto the stack.
    /// OP `[] -> [A]`
    Data(Data),
    /// Gets the color at the given pixel
    /// OP `color_at = [x:u32 y:u32] -> [Color]`
    ColorAt,
    /// Sets a given pixel
    /// OP `[x:u32, y:u32 color:Color] -> []`
    SetPixel,
    /// Loads an image.
    /// TODO: op
    LoadImage,
}

impl Op {
    /// Returns the list of required inputs on the stack for the operation.
    pub fn required_inputs(&self) -> Vec<Type> {
        match self {
            Op::ColorAt => vec![Type::U32, Type::U32],
            Op::Data(_) => vec![],
            Op::LoadImage => vec![Type::String],
            Op::SetPixel => vec![Type::U32, Type::U32, Type::Color],
        }
    }

    /// Returns the type the operation returns
    pub fn return_type(&self) -> Type {
        match self {
            Op::ColorAt => Type::Color,
            Op::Data(d) => d.get_type(),
            Op::LoadImage => todo!(),
            Op::SetPixel => Type::Empty,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod required_inputs {
        use super::*;

        #[test]
        fn color_at() {
            assert_eq!(vec![Type::U32, Type::U32], Op::ColorAt.required_inputs());
        }

        #[test]
        fn data() {
            assert_eq!(Vec::<Type>::new(), Op::Data(Data::U32(1)).required_inputs());
        }

        #[test]
        fn load_image() {
            assert_eq!(vec![Type::String], Op::LoadImage.required_inputs());
        }

        #[test]
        fn set_pixel() {
            assert_eq!(
                vec![Type::U32, Type::U32, Type::Color],
                Op::SetPixel.required_inputs()
            );
        }
    }

    mod return_type {
        use super::*;

        #[test]
        fn color_at() {
            assert_eq!(Type::Color, Op::ColorAt.return_type());
        }

        #[test]
        fn data() {
            todo!();
        }

        #[test]
        fn load_image() {
            todo!()
        }

        #[test]
        fn set_pixel() {
            assert_eq!(Type::Empty, Op::SetPixel.return_type());
        }
    }
}
