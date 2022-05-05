// extern crate test_2022_03_23;

// pub mod partial_eq;

mod foo;
mod tokio_ex;

#[tokio::main]
async fn main() {
    // test_2022_03_23::foo;
    // foo::public_function();
    // foo::zoo::public_function();
    // test_2022_03_23::hello_a(); //from lib.rs
    // test_2022_03_23::say_a(); //from a.rs>lib.rs
    tokio_ex::future::main().await;
}
// mod my;
// fn function() {
//     println!("called `function()`");
// }

// fn main() {
//     my::function();

//     function();

//     my::indirect_access();

//     // my::nested::function();
// }
