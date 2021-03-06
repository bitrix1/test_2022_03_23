#![allow(unused)]

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    mem,
    rc::{Rc, Weak},
};
#[derive(Debug)]
pub struct SnakePart {
    // id: &str,
    name: String,
    next: RefCell<Weak<SnakePart>>,
    prev: RefCell<Option<Rc<SnakePart>>>,
}

impl SnakePart {
    fn push_next(&mut self, tail: Rc<SnakePart>) {
        // let old_v = mem::replace(
        //     self,
        //     SnakePart {
        //         name: "move".to_string(),
        //         next: RefCell::new(Weak::new()),
        //         prev: RefCell::new(Option::None),
        //     },
        // );

        // let mut rcc = &mut RefCell::new(Option::None);
        // let head = SnakePart {
        //     name: "move".to_string(),
        //     next: RefCell::new(Weak::new()),
        //     prev: mem::replace(self.prev.borrow_mut(), rcc),
        // };

        // let new_self = &mut *self;
        // let old_v = Rc::new(old_v);
        // *tail.prev.borrow_mut() = Some(Rc::new(*self));
        // mem::replace(self, old_v);
        *self.next.borrow_mut() = Rc::downgrade(&tail);
    }
    fn move_next(&self) -> Rc<SnakePart> {
        self.next.borrow().upgrade().unwrap()
        // let head = self.move_next();
        // dbg!(&head);
        // head
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test2() {
        let tail = Rc::new(SnakePart {
            name: "1".to_string(),
            next: RefCell::new(Weak::new()),
            prev: RefCell::new(Option::None),
        });

        let head = Rc::new(SnakePart {
            name: "0".to_string(),
            next: RefCell::new(Rc::downgrade(&tail)),
            prev: RefCell::new(None),
        });
        head.move_next();
        head.move_next();
    }
    #[test]
    fn test1() {
        let tail = Rc::new(SnakePart {
            name: "1".to_string(),
            next: RefCell::new(Weak::new()),
            prev: RefCell::new(Option::None),
        });

        let head = Rc::new(SnakePart {
            name: "0".to_string(),
            next: RefCell::new(Weak::new()),
            prev: RefCell::new(Option::None),
        });
        *head.next.borrow_mut() = Rc::downgrade(&tail);
        //???????????????????? ???????????? ???? ??????????, ?? ???? ???????????????????? ??????????????.
        let head1 = head.move_next();
        *head1.prev.borrow_mut() = Some(head);
        // dbg!(&head1);
        //End

        let tail = Rc::new(SnakePart {
            name: "2".to_string(),
            next: RefCell::new(Weak::new()),
            prev: RefCell::new(Option::None),
        });

        *head1.next.borrow_mut() = Rc::downgrade(&tail);
        let head = head1.move_next();
        *head.prev.borrow_mut() = Some(head1);
        // dbg!(&head);
        // dbg!(&head1.move_next());
        //End

        let tail = Rc::new(SnakePart {
            name: "3".to_string(),
            next: RefCell::new(Weak::new()),
            prev: RefCell::new(Option::None),
        });

        *head.next.borrow_mut() = Rc::downgrade(&tail);
        let head1 = head.move_next();
        *head1.prev.borrow_mut() = Some(head);
        //End
        // {
        //     let tail = Rc::new(SnakePart {
        //         name: "3".to_string(),
        //         next: RefCell::new(Weak::new()),
        //         prev: RefCell::new(Option::None),
        //     });

        //     // *(&head.next.borrow_mut()) = Rc::downgrade(&tail);
        //     // let head1 = head.move_next();
        //     *head1.prev.borrow_mut() = &&Rc::downgrade(head);
        //     dbg!(&head1);
        // }

        // let tail = Rc::new(SnakePart {
        //     name: "2".to_string(),
        //     next: RefCell::new(Weak::new()),
        //     prev: RefCell::new(Option::None),
        // });

        // *head.next.borrow_mut() = Rc::downgrade(&tail);

        //invalid left-hand side of assignment
        // head.next.borrow_mut() = tail;
        // dbg!(&head);
    }
}
