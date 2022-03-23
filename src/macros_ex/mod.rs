//! https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
//! https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#:~:text=Rust%20has%20excellent%20support%20for,are%20expanded%20during%20compile%20time.
#![allow(unused)]
pub mod hello;

pub use hello::fn1;
// pub use hello::say_hello;
// #[macro_use]
pub use crate::say_hello; // Auto set "say_hello" in root module.
pub use hello::fn1_arg_default;
// pub mod hello {
//     pub fn say_hello() {
//         println!("hello");
//     }
// }
