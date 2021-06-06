use super::{Data, Type};

pub enum Op {
    /// Push some form of data onto the stack
    /// OP: `[] -> [A]
    Data(Data),
    /// Loads a pixel from the given texture.
    /// OP: `[x:u32 y:u32 texture:u32] -> [color]
    GetPixel,
    /// Puts the given fragment position onto the stack
    /// OP: `fragPos = [] -> [x:u32 y:u32]
    FragPos,
}

impl Op {
    /// Returns the required inputs for the given op.
    pub fn required_inputs(&self) -> &[Type] {
        match self {
            Op::Data(_) => todo!(),
            Op::GetPixel => &[Type::U32, Type::U32, Type::String],
            Op::FragPos => &[],
        }
    }
}
