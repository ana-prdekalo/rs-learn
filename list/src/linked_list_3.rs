use std::rc::Rc;

pub struct LinkedList<T> {
    head: Option<Rc<Node<T>>>,
}

pub struct Node<T> {
    data: T,
    next: Option<Rc<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList { head: None }
    }
}

impl<T> LinkedList<T> {
    pub fn new(head: Option<Rc<Node<T>>>) -> Self {
        LinkedList { head }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // list1 = A -> B -> C -> D
    // list2 = tail(list1) = B -> C -> D
    // list3 = push(list2, X) = X -> B -> C -> D

    // list1 -> A ---+
    //               |
    //               v
    // list2 ------> B -> C -> D
    //               ^
    //               |
    // list3 -> X ---+

    pub fn tail(&self) -> LinkedList<T> {
        if self.is_empty() {
            return LinkedList::default();
        }

        LinkedList::new(self.head.as_ref().and_then(|n| n.next.clone()))
    }

    pub fn push_front(&self, data: T) -> LinkedList<T> {
        if self.is_empty() {
            return LinkedList::new(Some(Rc::new(Node::new(data))));
        }

        let mut node = Node::new(data);
        node.next = self.head.clone();

        LinkedList::new(Some(Rc::new(node)))
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.data)
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: self.head.as_deref(),
        }
    }
}

pub struct LinkedListIterator<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|n| {
            self.current = n.next.as_deref();
            &n.data
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_front() {
        let list = LinkedList::default();
        let list = list.push_front(1);
        let list = list.push_front(2);
        let list = list.push_front(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn tail() {
        let list = LinkedList::default();
        let list = list.push_front(1);
        let list = list.push_front(2);
        let list = list.push_front(3);
        let list = list.tail();
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn basics() {
        let list = LinkedList::default();
        assert_eq!(list.peek(), None);

        let list = list.push_front(1).push_front(2).push_front(3);
        assert_eq!(list.peek(), Some(&3));

        let list = list.tail();
        assert_eq!(list.peek(), Some(&2));

        let list = list.tail();
        assert_eq!(list.peek(), Some(&1));

        let list = list.tail();
        assert_eq!(list.peek(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.peek(), None);
    }
}
