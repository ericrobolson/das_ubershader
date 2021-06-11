use super::{Data, Type};

/// Various operations that may be performed by the VM.
#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    /// Adds two values on the stack.
    /// OP: `+ = [a:Number b:Number] -> [Number]`
    Add,
    /// Takes two bools and executes an `&&`.
    /// OP: `&& = [a:bool b:bool] -> [bool]`
    And,
    /// Push some form of data onto the stack
    /// OP: `[] -> [A]
    Data(Data),
    /// Returns the height and width of the image.
    /// OP: `dim = [] -> [w:u32 h:u32]`
    Dimensions,
    /// Divides a number by a second number.
    /// OP: `/ = [divisor:Number n:Number] -> [Number]`
    Divide,
    /// Signals a bool for an if statement
    /// OP: `do = [] -> []`
    Do,
    /// Drops an item off the stack.
    /// OP: `drop = [A] -> []`
    Drop,
    /// Duplicates the top element of the stack.
    /// OP: `dup = [A] -> [A A]`
    Dup,
    /// A end label for a conditional.
    /// OP: `end = [] -> []`
    End,
    /// Checks whether two things are equal.
    /// OP: `== = [A A] -> [bool]`
    Equal,
    /// Puts the given fragment position onto the stack.
    /// OP: `fragPos = [] -> [x:u32 y:u32]
    FragPos,
    /// Checks whether a is greater than b.
    /// OP: `> = [a:Number b:Number] -> [bool]`
    GreaterThan,
    /// Checks whether a is greater than or equal to b.
    /// OP: `>= = [a:Number b:Number] -> [bool]`
    GreaterThanEqual,
    /// If the top of the stack is true, execute the proceeding block.
    /// Otherwise skip to the `end` op.
    /// OP: `if = [cond:bool] -> []`
    If,
    /// Checks whether a is less than b.
    /// OP: `< = [a:Number b:Number] -> [bool]`
    LessThan,
    /// Checks whether a is less than or equal to b.
    /// OP: `=< = [a:Number b:Number] -> [bool]`
    LessThanEqual,
    /// Converts 4 u8's to a color.
    /// OP: `makeColor = [r:u8 g:u8 b:u8 a:u8] -> [color:Color]`
    MakeColor,
    /// Performs a modulo on two numbers.
    /// OP: `% = [n:Number modulus:Number] -> [Number]`
    Modulo,
    /// Multiplies two numbers.
    /// OP: `* = [multiplier:Number n:Number] -> [Number]`
    Multiply,
    /// Rotates the top two elements of the stack.
    /// OP: `rot = [A B] -> [B A]`
    Rot,
    /// Rotates the top element and the Nth elements of the stack.
    /// OP: `rotN = [A .. B N] -> [B .. A]`
    RotN,
    /// Splits a color into each individual part.
    /// OP: `splitColor = [c:color] -> [r:u8 g:u8 b:u8 a:u8]
    SplitColor,
    /// Subtracts the top two elements of the stack.
    /// OP: `- = [subtractor:Number n:Number] -> [Number]`
    Subtract,
    /// Loads a pixel from the given texture.
    /// OP: `texturePixel = [x:u32 y:u32 textureIdx:u32] -> [color]
    TexturePixel,
}

impl Op {
    /// Returns the required inputs for the given op.
    pub fn required_inputs(&self) -> &[Type] {
        match self {
            Op::Add => todo!(),
            Op::And => &[Type::Bool, Type::Bool],
            Op::Data(_) => &[],
            Op::Dimensions => todo!(),
            Op::Divide => todo!(),
            Op::Do => todo!(),
            Op::Drop => &[Type::Any],
            Op::Dup => &[Type::Any],
            Op::End => todo!(),
            Op::Equal => &[Type::Any, Type::Any],
            Op::FragPos => &[],
            Op::GreaterThan => &[Type::U32, Type::U32],
            Op::GreaterThanEqual => &[Type::U32, Type::U32],
            Op::If => &[Type::Bool],
            Op::LessThan => &[Type::U32, Type::U32],
            Op::LessThanEqual => &[Type::U32, Type::U32],
            Op::MakeColor => &[Type::U8, Type::U8, Type::U8, Type::U8],
            Op::Modulo => todo!(),
            Op::Multiply => todo!(),
            Op::Rot => &[Type::Any, Type::Any],
            Op::RotN => &[Type::Any, Type::Any, Type::U32],
            Op::SplitColor => &[Type::Color],
            Op::Subtract => todo!(),
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
        fn and() {
            let op = Op::And;
            let expected: &[Type] = &[Type::Bool, Type::Bool];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn data() {
            let op = Op::Data(Data::U32(3));
            let expected: &[Type] = &[];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn drop() {
            let op = Op::Drop;
            let expected: &[Type] = &[Type::Any];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn dup() {
            let op = Op::Dup;
            let expected: &[Type] = &[Type::Any];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn eq() {
            let op = Op::Equal;
            let expected: &[Type] = &[Type::Any, Type::Any];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn frag_pos() {
            let op = Op::FragPos;
            let expected: &[Type] = &[];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn greater_than() {
            let op = Op::GreaterThan;
            let expected: &[Type] = &[Type::U32, Type::U32];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn greater_than_equal() {
            let op = Op::GreaterThanEqual;
            let expected: &[Type] = &[Type::U32, Type::U32];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn if_() {
            let op = Op::If;
            let expected: &[Type] = &[Type::Bool];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn less_than() {
            let op = Op::LessThan;
            let expected: &[Type] = &[Type::U32, Type::U32];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn less_than_equal() {
            let op = Op::LessThan;
            let expected: &[Type] = &[Type::U32, Type::U32];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn make_color() {
            let op = Op::MakeColor;
            let expected: &[Type] = &[Type::U8, Type::U8, Type::U8, Type::U8];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn rot() {
            let op = Op::Rot;
            let expected: &[Type] = &[Type::Any, Type::Any];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn rot_n() {
            let op = Op::RotN;
            let expected: &[Type] = &[Type::Any, Type::Any, Type::U32];
            assert_eq!(expected, op.required_inputs());
        }

        #[test]
        fn split_color() {
            let op = Op::SplitColor;
            let expected: &[Type] = &[Type::Color];
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
