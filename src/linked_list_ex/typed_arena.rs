use std::cell::Cell;
use typed_arena::Arena;

#[derive(Debug)]
struct CycleParticipant<'a> {
    name: String,
    prev: Cell<Option<&'a CycleParticipant<'a>>>,
    next: Cell<Option<&'a CycleParticipant<'a>>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let arena = Arena::new();

        let a = arena.alloc(CycleParticipant {
            name: "a".to_string(),
            prev: Cell::new(None),
            next: Cell::new(None),
        });
        let b = arena.alloc(CycleParticipant {
            name: "b".to_string(),
            prev: Cell::new(None),
            next: Cell::new(None),
        });
        let c = arena.alloc(CycleParticipant {
            name: "c".to_string(),
            prev: Cell::new(None),
            next: Cell::new(None),
        });

        a.next.set(Some(b));
        b.prev.set(Some(a));
        b.next.set(Some(c));
        c.prev.set(Some(b));
        c.next.set(Some(a));
        dbg!(&a);
    }
}
