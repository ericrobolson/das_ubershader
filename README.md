# das_ubershader
* Create and execute image processing operations.
* Stack based interpreter.

## What does this solve?
* TODO: Fill in before shipping

## Why use it?
* TODO: Fill in before shipping

# Usage:
* TODO: Fill in before shipping

# Tech choices:
* TODO: Fill in before shipping

# Architecture:
* TODO: Fill in before shipping

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
