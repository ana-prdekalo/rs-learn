use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

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
        if self.is_empty() {
            self.head = Node::new_boxed(value);
            return;
        }

        let mut current = self.head.as_ref().unwrap();
        loop {
            if current.borrow().next.is_none() {
                current.borrow_mut().next = Node::new_boxed(value);
                break;
            }

            //current = current.borrow().next;
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
}
