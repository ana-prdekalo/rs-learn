use std::{
    fmt::{self, Display, Formatter},
    mem,
};

#[derive(PartialEq, Eq, Default)]
pub enum LinkedList {
    #[default]
    Empty,
    Elem(usize, Box<LinkedList>),
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList::Empty
    }

    pub fn val(&self) -> Option<usize> {
        match self {
            LinkedList::Empty => None,
            LinkedList::Elem(val, _) => Some(*val),
        }
    }

    pub fn len(&self) -> usize {
        let mut head = self;
        let mut count = 0;
        while let LinkedList::Elem(_, next) = head {
            head = next;
            count += 1;
        }

        count
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, LinkedList::Empty)
    }

    pub fn push_front(&mut self, item: usize) {
        let mut old_head = LinkedList::new();
        std::mem::swap(&mut old_head, self);
        *self = LinkedList::Elem(item, Box::new(old_head));
    }

    pub fn push_back(&mut self, item: usize) {
        let tail = LinkedList::Elem(item, Box::new(LinkedList::Empty));
        if self.is_empty() {
            *self = tail;
            return;
        }

        let mut head = self;
        while let LinkedList::Elem(_, next) = head {
            if **next == LinkedList::Empty {
                *next = Box::new(tail);
                return;
            }
            head = next;
        }
    }

    pub fn pop_back(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        // Special case: only one element in the list
        if let LinkedList::Elem(_, next) = self {
            if **next == LinkedList::Empty {
                let mut tail = LinkedList::Empty;
                mem::swap(self, &mut tail);
                return tail.val();
            }
        }

        let mut head = self;
        while let LinkedList::Elem(_, next) = head {
            if let LinkedList::Elem(_, next_next) = &**next {
                if **next_next == LinkedList::Empty {
                    let mut tail = LinkedList::Empty;
                    mem::swap(&mut tail, next);
                    return tail.val();
                }
            }
            head = next;
        }

        None
    }

    pub fn pop_front(&mut self) -> Option<usize> {
        match std::mem::replace(self, LinkedList::Empty) {
            LinkedList::Empty => None,
            LinkedList::Elem(val, next) => {
                *self = *next;
                Some(val)
            }
        }
    }

    pub fn find(&self, item: usize) -> Option<usize> {
        let mut idx = 0;
        let mut head = self;

        while let LinkedList::Elem(v, next) = head {
            if *v == item {
                return Some(idx);
            }
            idx += 1;
            head = next;
        }

        None
    }

    pub fn iter(&self) -> LinkedListIter {
        LinkedListIter { head: self }
    }
}

impl fmt::Debug for LinkedList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in self.iter() {
            writeln!(f, "{:?} ", item)?;
        }

        Ok(())
    }
}

impl Display for LinkedList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for item in self.iter() {
            writeln!(f, "{} ", item)?;
        }

        Ok(())
    }
}

pub struct LinkedListIter<'a> {
    head: &'a LinkedList,
}

impl<'a> Iterator for LinkedListIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            LinkedList::Empty => None,
            LinkedList::Elem(val, next) => {
                self.head = next;
                Some(*val)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::linked_list_1::LinkedList;

    #[test]
    fn push_back_and_front() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);

        list.push_back(1);
        list.push_back(2);

        assert_eq!(
            list,
            LinkedList::Elem(
                1,
                Box::new(LinkedList::Elem(2, Box::new(LinkedList::Empty)))
            )
        );

        for i in 1..3 {
            list.push_front(i);
        }

        assert_eq!(list.len(), 4);
    }

    #[test]
    fn find() {
        let mut list = LinkedList::new();
        assert_eq!(list.find(4), None);

        list.push_back(1);
        assert_eq!(list.find(1), Some(0));

        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.find(3), Some(2));
        assert_eq!(list.find(1), Some(0));
        assert_eq!(list.find(4), None);
    }

    #[test]
    fn pop_back() {
        let mut list = LinkedList::new();
        list.push_back(1);

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list, LinkedList::Empty);

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.len(), 2);

        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.len(), 1);

        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.len(), 0);

        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_front() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(), 2);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.len(), 0);

        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_front_and_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);

        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(), 3);

        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.len(), 2);

        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);

        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.len(), 0);

        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }
}
