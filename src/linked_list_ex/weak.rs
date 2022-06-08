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
    fn push_next(&mut self, mut tail: SnakePart) {
        // *tail.prev.borrow_mut() = Some(Rc::new(*self));
        *self.next.borrow_mut() = Rc::downgrade(&Rc::new(tail));
    }
    fn move_next(&self) -> Rc<SnakePart> {
        self.next.borrow().upgrade().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
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
        // let tmp = Rc::downgrade(&head.move_next());
        let mut tmp = SnakePart {
            name: "2".to_string(),
            next: RefCell::new(Weak::new()),
            prev: RefCell::new(None),
        };
        // let tmp = &tmp;
        // head.push_next(tmp);
        // head.move_next();
        // head.move_next();
    }
}
