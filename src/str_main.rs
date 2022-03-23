#![allow(unused)]

use std::fmt::Debug;
use std::ops::Add;

fn main() {
    println!("Hello, world!");

    let a = "a";
    let b = string_a(a);
    let bb = A { a: "123" };
    let p = Point { x: 1, y: 2 };

    println!("{:?}", bb + A { a: "123" });

    // let mut num = 5;
    // let n1 = &num as *const i32;
    // let n2 = &mut num as *mut i32;
    // println!("n1={:p}", n1);
    // println!("n2={:p}", n2);
}
fn string_a(a: &str) -> &str {
    a
}
struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct A {
    a: &'static str,
}
struct B {
    a: String,
}
// trait Add{
//     fn a(&self, add:&str) -> &str {"sdfsdfsdf"}
// }

impl Add for A {
    type Output = &'static str;
    fn add(self, _: Self) -> Self::Output {
        "fn add()"
    }
    // fn add(self, _: Self) -> &'static str  { "todo!()" }
    // type Output = Vec<u8>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn add() {
        assert_eq!(A { a: "123" } + A { a: "123" }, "fn add()");
    }
}
