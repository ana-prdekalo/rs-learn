use std::cell::RefCell;

#[derive(Default)]
pub struct LinkedListCell<T> {
    head: Option<RefCell<Box<Node<T>>>>,
}

#[derive(Default)]
struct Node<T> {
    data: T,
    next: Option<RefCell<Box<Node<T>>>>,
}

impl<T> LinkedListCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            head: Node::new_boxed(value),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn push_back(&mut self, value: T) {
        match self.head {
            None => self.head = Node::new_boxed(value),
            Some(ref mut n) => n.borrow_mut().push_back(value),
        }
    }
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node { data, next: None }
    }

    pub fn new_boxed(value: T) -> Option<RefCell<Box<Node<T>>>> {
        Some(RefCell::new(Box::new(Node::new(value))))
    }

    pub fn push_back(&mut self, value: T) {
        match &mut self.next {
            None => self.next = Node::new_boxed(value),
            Some(n) => {
                n.borrow_mut().push_back(value);
            }
        }
    }

    pub fn pop_back(&mut self) -> T {
        match &mut self.next {
            None => self.next = Node::new_boxed(value),
            Some(n) => {
                n.borrow_mut().pop_back(value);
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_linked_list_cell() {
        use super::LinkedListCell;

        let mut list = LinkedListCell::new(1);
        assert_eq!(list.is_empty(), false);

        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.is_empty(), false);
    }

    #[test]
    fn test_linked_list_cell_empty() {
        use super::LinkedListCell;

        let list: LinkedListCell<i32> = LinkedListCell::default();
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn test_linked_list_cell_push_back() {
        use super::LinkedListCell;

        let mut list = LinkedListCell::new(1);
        assert_eq!(list.is_empty(), false);

        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);

        assert_eq!(list.is_empty(), false);
    }
}
