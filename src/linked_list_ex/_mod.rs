#![allow(unused)]

use std::sync::{Arc, Mutex};
struct MyLinkedList<'a> {
    head: Option<&'a mut MyLinkedList<'a>>,
    tail: Option<&'a mut MyLinkedList<'a>>,
}

#[cfg(test)]
mod tests {
    use futures::lock::MutexGuard;

    use super::*;
    #[test]
    fn test1() {
        let mut l_l = vec![];

        //Element 1
        let mut tmp = MyLinkedList {
            head: None,
            tail: None,
        };
        let mut tmp: Arc<Mutex<MyLinkedList>> = Arc::new(Mutex::new(tmp));
        l_l.push(tmp);

        //Element 2
        let mut tmp = MyLinkedList {
            head: None,
            tail: None,
        };
        let mut tmp: Arc<Mutex<MyLinkedList>> = Arc::new(Mutex::new(tmp));
        l_l.push(tmp);

        ///
        {
            ///Tail
            let mut tmp1 = l_l[0].lock().unwrap();
            let mut un_tmp1 = match &mut *tmp1 {
                x => x,
                _ => panic!("Error parse MyLinkedList."),
            };
            let mut tmp2 = l_l[1].lock().unwrap();
            let mut ref_n2 = match &mut *tmp2 {
                x => x,
                _ => panic!("Error parse MyLinkedList."),
            };
            // un_tmp1.tail = Some(&mut *ref_n2);
        }
        {
            ///Head
            let mut tmp1 = l_l[0].lock().unwrap();
            let mut un_tmp1 = match &mut *tmp1 {
                x => x,
                _ => panic!("Error parse MyLinkedList."),
            };
            let mut tmp2 = l_l[1].lock().unwrap();
            let un_tmp2 = match &mut *tmp2 {
                x => x,
                _ => panic!("Error parse MyLinkedList."),
            };
            // un_tmp2.head = Some(&mut *un_tmp1);
        }
        let mutex = Mutex::new(Some(1i32));
        let mut guard = mutex.lock().unwrap();
        let mutex1 = Mutex::new(Some(MyLinkedList {
            head: None,
            tail: None,
        }));
        let mut guard1 = mutex1.lock().unwrap();

        // Directly change inner value
        *guard = Some(2);
        // *guard1 = Some(2);

        let mut ttt = match &mut *guard1 {
            Some(x) => Some(x),
            None => None,
        };
    }

    #[test]
    fn test4() {
        let mut tmp = MyLinkedList {
            head: None,
            tail: None,
        };
        let mut l_l: Vec<MyLinkedList> = vec![];
        l_l.push(tmp);
        // let l_l[0]
        // let tmp = &mut l_l[0];

        // l_l.push(MyLinkedList {
        //     head: Some(&mut l_l[0]),
        //     tail: None,
        // })
    }

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
