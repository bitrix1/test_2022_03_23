#![allow(unused)]

use std::{
    cell::RefCell,
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
    fn move_next(&self) -> Rc<SnakePart> {
        self.move_next()
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
        //Перемещаем курсор на хвост, и он становится головой.
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
        dbg!(&head1);
        //End

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
