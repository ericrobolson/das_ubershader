use crate::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// The maximum stack size has been exceeded.
    StackOverflow,
    /// An item was popped off the stack, but nothing existed.
    StackUnderflow,
    /// There was a type mismatch.
    TypeMismatch { expected: Type, got: Type },

    /// A token was passed that is not defined.
    UndefinedToken { got: String },
}
