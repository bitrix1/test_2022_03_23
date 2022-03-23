//! # Example
//! pub mod macros_ex;
//! use crate::macros_ex::hello::fn1_arg_default;
//! fn1_arg_default();
#![allow(unused)]
// This is a simple macro named `say_hello`.
pub fn fn1() {}
#[macro_export]
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello default!")
    };
    ($expression:expr) => {
        // `stringify!` will convert the expression *as it is* into a string.
        // pub fn () {
        // }
        println!("Hello {:?} = {:?}", stringify!($expression), $expression);
    };
}

pub fn fn1_arg_default() {
    // assert_eq!(2 + 2, 4);
    say_hello!();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fn1_arg_default() {
        // assert_eq!(2 + 2, 4);
        say_hello!();
    }
    #[test]
    fn fn1_arg_1() {
        // assert_eq!(2 + 2, 4);
        say_hello!("asdasd");
    }
}
