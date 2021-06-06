use super::{Data, Type};

/// Various operations that may be performed by the VM.
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    /// Push some form of data onto the stack
    /// OP: `[] -> [A]
    Data(Data),
    /// Duplicates the top element of the stack
    /// OP: `dup = [A] -> [A A]`
    Dup,
    /// Puts the given fragment position onto the stack
    /// OP: `fragPos = [] -> [x:u32 y:u32]
    FragPos,

    /// Rotates the top two elements of the stack
    /// OP: `rot = [A B] -> [B A]`
    Rot,
    /// Loads a pixel from the given texture.
    /// OP: `texturePixel = [x:u32 y:u32 textureIdx:u32] -> [color]
    TexturePixel,
}

impl Op {
    /// Returns the required inputs for the given op.
    pub fn required_inputs(&self) -> &[Type] {
        match self {
            Op::Data(_) => &[],
            Op::Dup => &[Type::Any],
            Op::FragPos => &[],
            Op::Rot => &[Type::Any, Type::Any],
            Op::TexturePixel => &[Type::U32, Type::U32, Type::U32],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod required_inputs {
        use super::*;

        #[test]
        fn data() {
            let op = Op::Data(Data::U32(3));
            let expected: &[Type] = &[];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn dup() {
            let op = Op::Dup;
            let expected: &[Type] = &[Type::Any];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn frag_pos() {
            let op = Op::FragPos;
            let expected: &[Type] = &[];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn rot() {
            let op = Op::Rot;
            let expected: &[Type] = &[Type::Any, Type::Any];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn texture_pixel() {
            let op = Op::TexturePixel;
            let expected: &[Type] = &[Type::U32, Type::U32, Type::U32];
            assert_eq!(expected, op.required_inputs());
        }
    }
}
