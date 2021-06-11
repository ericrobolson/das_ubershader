# das_ubershader
* Create and execute image processing operations.
* Stack based interpreter.

# Usage:
* Create a new `cfg.json` which will specify the output, the dimensions, the inputs and the program to execute
* Add in your images/program
* Execute with `cargo run example/cfg.json`

# Coding standards:
* Alphabetize all things
* * At top level, it's `mod` -> `pub use` -> `use` then rest of code
* * After that, it's `pub enum` -> `enum` -> `pub fn` -> `fn` -> `pub struct` -> `struct` -> `tests`
* * All `fn` params should be alphabetized
* * All `struct` props should be alphabetized, except in the case of convention like colors where `r: u8, g: u8, b: u8, a: u8` is acceptable.
* * If `fn` prop count >= 5, use a struct
* Comments
* * Should end in a `.`
* * Applies to `enum`, `struct`, `fn`, and `struct` properties
* Tests should be modularized by `fn` name, then by inputs
