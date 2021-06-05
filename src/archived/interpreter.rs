use std::collections::HashMap;

use game_utils::collections::Stack;

use crate::{Data, Error, Op};

/// The interpreter used for performing ops
pub struct Interpreter {
    /// Data store that can persist values.
    dictionary: HashMap<String, Data>,
    /// The stack that contains working values.
    stack: Stack<Data>,
}

impl Interpreter {
    /// Creates a new interpreter
    pub fn new() -> Self {
        Self {
            dictionary: HashMap::new(),
            stack: Stack::new(),
        }
    }

    /// Parses the given token
    pub fn parse(&self, token: &str) -> Result<Op, Error> {
        match token {
            "load_img" => Ok(Op::LoadImage),
            _ => Err(Error::UndefinedToken {
                got: token.to_string(),
            }),
        }
    }

    /// Executes the given token.
    pub fn execute(&mut self, op: Op) -> Result<(), Error> {
        // Validate the type
        {
            let mut working_stack = Stack::new();
            let mut error = None;

            for required_type in op.required_inputs() {
                // TODO: check that stack has those types
                match self.stack.pop() {
                    Some(data) => {
                        let data_type = data.get_type();
                        if data_type == required_type {
                            working_stack.push(data);
                        } else {
                            // The type is a mismatch so return an error
                            error = Some(Error::TypeMismatch {
                                expected: required_type,
                                got: data_type,
                            });
                            break;
                        }
                    }
                    None => todo!(),
                }
            }

            // Reset the stack
            while let Some(data) = working_stack.pop() {
                self.stack.push(data);
            }

            // Exit if errored
            if let Some(error) = error {
                return Err(error);
            }
        }

        // Execute the op

        todo!()
    }

    /// Pops an item off the stack
    fn pop(&mut self) -> Result<Data, Error> {
        match self.stack.pop() {
            Some(data) => Ok(data),
            None => Err(Error::StackUnderflow),
        }
    }

    /// Pushes an item onto the stack
    fn push(&mut self, data: Data) -> Result<(), Error> {
        self.stack.push(data);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn interpreter() -> Interpreter {
        Interpreter::new()
    }

    mod execute {
        use super::*;

        #[test]
        fn execute_no_required_types_executes_succesfully() {
            todo!();
        }

        #[test]
        fn execute_first_required_type_fails_returns_err() {
            todo!();
        }

        #[test]
        fn execute_second_required_type_fails_returns_err() {
            todo!()
        }

        #[test]
        fn execute_all_required_types_pass_returns_ok() {
            todo!()
        }
    }

    #[test]
    fn pop_value_returns_ok() {
        let mut i = interpreter();
        i.push(Data::String("wut".into())).unwrap();

        assert_eq!(Ok(Data::String("wut".into())), i.pop());
    }

    #[test]
    fn pop_no_value_returns_err() {
        let mut i = interpreter();
        assert_eq!(Err(Error::StackUnderflow), i.pop());
    }

    #[test]
    fn push() {
        let mut i = interpreter();
        let result = i.push(Data::String("wut".into()));
        let mut expected = Stack::new();
        expected.push(Data::String("wut".into()));

        assert_eq!(expected.peek(), i.stack.peek());
        assert_eq!(Ok(()), result);
    }

    mod parse {
        use super::*;

        #[test]
        fn parse() {
            todo!()
        }

        #[test]
        fn parse_load_img() {
            assert_eq!(Ok(Op::LoadImage), interpreter().parse("load_img"));
        }
    }
}
