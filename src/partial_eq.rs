#![allow(unused)]

use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug, PartialEq, PartialOrd)]
struct S<'a> {
    name2: &'a str,
    name1: &'a str,
    a_val: i32,
}
#[allow(dead_code)]
pub fn test12312313() {}

fn main() {
    println!("Hello, world!");
    let s1 = S {
        name2: "name1", // 3) name2 < or > name2
        name1: "name1", // 2) name1 < or > name1
        a_val: 2,       // 1) a_val < or > a_val
    };
    let s2 = S {
        a_val: 1,
        name2: "name1",
        name1: "name1",
    };
    // Сравнение структур в алфавитном порядке.
    println!("{:?}", s1 > s2)
}

#[cfg(test)]
mod tests {
    use super::*;
}
