pub mod codewars_ex;
pub mod macros_ex;
pub mod module_ex;
pub mod option_unwrap_ex;
pub mod raw_ptr_ex;
pub mod thread_ex;
pub mod tokio_ex;
pub use crate::module_ex::say_a;

mod partial_eq;
pub use crate::partial_eq::test12312313;

pub fn hello_a() {
    println!("Hello, world!");
    // crate::macros_ex::hello::();
    // crate::macros_ex::hello::fn1_arg_default();
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn fn1_arg_default1() {
        // assert_eq!(2 + 2, 4);
        // use hello;
        // fn1_arg_default();
        // hello::fn1_arg_default(); //fn1_arg_default();
        // use crate::module_ex::
        use crate::macros_ex::hello::fn1_arg_default;
        fn1_arg_default();
    }
    // #[test]
    // fn fn1_arg_1() {
    //     // assert_eq!(2 + 2, 4);
    //     say_hello!("asdasd");
    // }
}
