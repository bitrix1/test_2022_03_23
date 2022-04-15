#![allow(unused)]

#[cfg(test)]
mod tests {

    use std::borrow::Borrow;

    use super::*;
    #[test]
    fn fn1() {
        //https://users.rust-lang.org/t/borrowed-value-does-not-live-long-enough/7225
        //https://users.rust-lang.org/t/error-e0716-temporary-value-dropped-while-borrowed/63428
        let mut d: &mut Option<String> = &mut None;
        //department.as_ref() not work!
        //1
        d.get_or_insert_with(|| String::from("234"));
        //department = Some("1".to_string());
        //department.as_ref().unwrap() = "".to_string();

        //2
        *d = Some("".to_string());
        //3
        let s = &mut Some(String::from(""));
        //3
        d = s;
        //error! consider using a `let` binding to create a longer lived value
        // d = &mut Some(String::from(""));
        d = &mut None;
        let s1: &mut Option<String> = &mut None;
        d = s1;
        //4
        //let r = *department.as_ref().unwrap() == "".to_string();
        dbg!(&d);
    }
    #[test]
    fn fn2() {
        let mut a: &mut String = &mut "None".to_string();
        *a = "".to_string();
        a = &mut "".to_string();
    }
    #[test]
    fn fn3() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let p: Option<Rc<RefCell<_>>>;
        // let p: Option<Rc<String>>;
        // p = Some(Rc::new("".to_string()));
        // let p: Option<i32> = Some(1);
        p = Some(Rc::new(RefCell::new(1)));
        // let pp = Borrow::borrow(&p.unwrap());
        // assert_eq!(p, p);
    }
    #[test]
    fn fn4() {
        fn check<T: Borrow<str>>(s: T) {
            assert_eq!("Hello", s.borrow());
        }
        let s = "Hello".to_string();
        check(s);
        let s = "Hello";
        check(s);

        fn check1(s: String) {
            // assert_eq!("Hello", s);
        }
        let s1 = "Hello".to_string();
        check1(s1);
        let s1 = "Hello";
        // check1(s1);
    }
    #[test]
    fn fn5() {
        let s = vec![1, 2, 3];
        //1) &s:0x7ffcf83939a0 x:0x7ffcf83939a0
        is_hello(&s);
        let x = <Vec<i32> as AsRef<Vec<i32>>>::as_ref(&s);
        //2) &s:0x7ffcf8393988 x:0x7ffcf8393988
        println!("2) &s:{:p} x:{:p}", &s, x);
        //3) 0x3def6ff3b0 0x3def6ff3b0 0x3def6ff3b0
        println!("3) {:p} {:p} {:p}", &s, *&&s, **&&&s);
        fn is_hello<T: AsRef<Vec<i32>>>(s: T) {
            let x = s.as_ref();
            // let s1 = *s;
            println!("1) &s:{:p} x:{:p}", &s, x);
            // s
        }

        fn is_hello1(s: &String) {
            let x = &s;
            println!("1) &s:{:p} x:{:p}", *&s, *x);
            // s
        }
        // is_hello(x);
        // assert_eq!(&vec![], s.as_ref());
        // fn is_hello<T: AsRef<str>>(s: T) {
        //     assert_eq!("hello", s.as_ref());
        //  }
        // impl<T: AsRef<i32>> std::vec::Vec<T> {
        //     pub fn as_ref(&self) -> &[i32] {
        //         &self.0
        //     }
        // }
        // impl Vec<i32> {
        //     fn as_ref(&self) -> &Self {
        //         self
        //     }
        // }
        //`<Vec<T, A> as AsRef<Vec<T, A>>>::as_ref(x)`, `<Vec<T, A> as AsRef<[T]>>::as_ref(x)`
        //
        // let xx = x as AsRef<Vec<_,_>> >;
        // println!("x:{:p} {:p}", &x, x);
    }
    #[test]
    fn fn6() {
        let y = {
            let x = 42;
            //&x //`x` does not live long enough borrowed value does not live long enough
        };
    }
}
