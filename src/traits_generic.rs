#![allow(unused)]
mod derive;

use std::fmt::Debug;
fn main() {
    // struct Pair<T> {
    //     x: T,
    //     y: T,
    // }
    // impl<T> Pair<T> {
    //     fn new(x: T, y: T) -> Self {
    //         Self { x, y }
    //     }
    // }

    #[derive(Debug)]
    struct A<'a> {
        a: &'a str,
    };

    trait TA<'a> {
        fn se() -> Self;
        // fn se() -> A<'a>;
        // fn new() -> Option<&'a Self>;
        fn test1(&self) -> &'a str;
    };
    impl<'a> A<'a> {
        fn new(aa: &'a str) -> Self {
            Self { a: aa }
        }
        // fn new(aa: &'a str) -> A<'a> {
        //     A { a: aa }
        // }
    }
    impl<'a> TA<'a> for A<'a> {
        fn test1(&self) -> &'a str {
            "self.a"
        }

        fn se() -> Self {
            Self { a: "123" }
        }
        // fn se() -> A<'a> {
        //     self.a;
        // }
        // fn new() -> A {
        //     A { a: "123" }
        // }
    };
    let a = A::new("1234");
    // let a = A { a: "123" };
    a.test1();
    println!("{:?}", a);
}

// trait Says {
//     fn say(&self);
// }

// struct Base {}

// impl Says for Base {
//     fn say(&self) {
//         println!("Я - base");
//     }
// }

// struct Derived {
//     base: Base,
// }

// impl Derived {
//     fn new() -> Derived {
//         Derived { base: Base {} }
//     }
// }

// impl Says for Derived {
//     fn say(&self) {
//         print!("Я - derived и хочу переиспользовать код из base. ");
//         self.base.say();
//         println!("А теперь я хочу продолжить выводить всё в ту же строку. Упс. ");
//     }
// }

// fn main() {
//     let d = Derived::new();
//     d.say();
// }
