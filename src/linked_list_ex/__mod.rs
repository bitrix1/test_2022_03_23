#![allow(unused)]
mod weak;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
#[derive(Debug)]
struct MyLinkedList {
    head: &'static mut Weak<MyLinkedList>,
    tail: &'static mut Weak<MyLinkedList>,
}

#[cfg(test)]
mod tests {
    use futures::lock::MutexGuard;

    use super::*;
    #[test]
    fn test1() {
        let strong = Rc::new("hello".to_owned());
        let weak = Rc::downgrade(&strong);
        // let weak = Weak::new();
        // weak.upgrade();
        // weak.into_raw();
        let mut l_l = vec![];

        //Element 1
        let mut tmp = MyLinkedList {
            head: &mut Weak::new(),
            tail: &mut Weak::new(),
        };
        let mut tmp = Rc::new(tmp);
        l_l.push(tmp);

        //Element 2
        let mut tmp = MyLinkedList {
            head: &mut Weak::new(),
            tail: &mut Weak::new(),
        };
        let mut tmp = Rc::new(tmp);
        l_l.push(tmp);

        ///
        {
            ///Tail
            // let mut tmp1 = Rc::clone(l_l[0]);
            let mut tmp1 = Rc::downgrade(&l_l[0]).upgrade();
            let mut tmp2 = Rc::downgrade(&l_l[1]).upgrade();
            let tmp1 = match tmp1 {
                Some(x) => x,
                _ => panic!("Error parse MyLinkedList."),
            };
            let tmp2 = match tmp2 {
                Some(x) => x,
                _ => panic!("Error parse MyLinkedList."),
            };
            let asdasd = Rc::downgrade(&tmp2);
            dbg!(tmp1.tail = &mut Rc::downgrade(&tmp2));
            // let tmp2 = match tmp2 {
            //     Some(x) => x,
            // };
            // let mut tmp2 = Rc::downgrade(&l_l[1]);
            // let tmp3 = tmp1.into_raw();
            // dbg!(unsafe { Weak::from_raw(tmp3).upgrade().unwrap().head });
            // (*tmp1.into_raw()).tail = Some(None);
            // Some(&mut tmp2.into_raw());
            // tmp1.into_raw().tail = Some(&mut tmp2.into_raw());
            // let mut un_tmp1 = match &mut *tmp1 {
            //     x => x,
            //     _ => panic!("Error parse MyLinkedList."),
            // };
            // let mut tmp2 = l_l[1].lock().unwrap();
            // let mut ref_n2 = match &mut *tmp2 {
            //     x => x,
            //     _ => panic!("Error parse MyLinkedList."),
            // };
            // un_tmp1.tail = Some(&mut *ref_n2);
        }
        // {
        //Head
        // let mut tmp1 = l_l[0].lock().unwrap();
        // let mut un_tmp1 = match &mut *tmp1 {
        //     x => x,
        //     _ => panic!("Error parse MyLinkedList."),
        // };
        // let mut tmp2 = l_l[1].lock().unwrap();
        // let un_tmp2 = match &mut *tmp2 {
        //     x => x,
        //     _ => panic!("Error parse MyLinkedList."),
        // };
        // un_tmp2.head = Some(&mut *un_tmp1);
        // }
    }

    #[test]
    fn test4() {}

    #[test]
    fn test2() {
        fn foo(b: &mut u64) {
            let x = &mut *b;
        }
        let mut x = 42;
        foo(&mut x);
    }
    #[test]
    fn test3() {
        struct MyLinkedList<'a> {
            head: Option<&'a mut MyLinkedList<'a>>,
            tail: Option<&'a mut MyLinkedList<'a>>,
        }
    }
}
