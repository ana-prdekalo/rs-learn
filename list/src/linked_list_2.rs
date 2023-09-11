#[derive(Default)]
pub struct LinkedList {
    head: Option<Box<Node>>,
}

struct Node {
    data: usize,
    next: Option<Box<Node>>,
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn has_exactly_one_element(&self) -> bool {
        self.head.as_ref().is_some_and(|n| n.next.is_none())
    }

    //TODO: improve after implementing iterator
    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next;
        }

        count
    }

    pub fn push_back(&mut self, data: usize) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Node::new_boxed(data, None)
    }

    pub fn push_front(&mut self, data: usize) {
        if self.is_empty() {
            self.head = Node::new_boxed(data, None);
            return;
        }

        self.head = Node::new_boxed(data, self.head.take());
    }

    pub fn pop_back(&mut self) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        if self.has_exactly_one_element() {
            return self.head.take().map(|node| node.data);
        }

        let mut current = &mut self.head;
        while let Some(ref mut node) = *current {
            //Note: in next.as_ref()?, '?' is OK, because we've handle special case of 1 element
            //above.In this loop we look forward 2 elements. In case next_next is empty (None)
            //we are done. This is why  we know next will always exist
            if node.next.as_ref()?.next.is_none() {
                return node.next.take().map(|node| node.data);
            }
            current = &mut node.next;
        }

        //unreachable because we've handle special case of 0 elements before loop
        unreachable!()
    }

    pub fn pop_front(&mut self) -> Option<usize> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn first_index_of(&self, data: usize) -> Option<usize> {
        let mut current = &self.head;
        let mut index = 0;

        while let Some(node) = current {
            if node.data == data {
                return Some(index);
            }

            index += 1;
            current = &node.next;
        }

        None
    }

    /// Insert data at index, which is 0 based, meaning first element has index 0
    /// If index >= list.len(), inserts at the end of the list
    /// NOTE: this could've been implemented so that it returns result ic case index >= list.len()
    /// But I didn't want to complicate API
    pub fn insert_at(&mut self, index: usize, data: usize) {
        if index == 0 {
            self.push_front(data);
            return;
        }

        let mut current = &mut self.head;
        for _ in 0..index - 1 {
            match current {
                Some(node) => current = &mut node.next,
                // as soon as we go out of bounds, we push_back and we're done
                None => {
                    self.push_back(data);
                    return;
                }
            }
        }

        //unwrap is ok, because, above in case None we have early return
        let current = current.as_mut().unwrap();
        let new_node = Node::new_boxed(data, current.next.take());
        current.next = new_node;
    }
}

impl Node {
    fn new(data: usize) -> Self {
        Node { data, next: None }
    }

    fn new_boxed(data: usize, next: Option<Box<Node>>) -> Option<Box<Node>> {
        match &next {
            Some(_) => Some(Box::new(Node { data, next })),
            None => Some(Box::new(Node::new(data))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_back() {
        let mut list = LinkedList::new();
        assert_eq!(list.len(), 0);
        list.push_back(1);
        assert_eq!(list.len(), 1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn push_front() {
        let mut list = LinkedList::new();

        assert_eq!(list.len(), 0);
        list.push_front(1);
        assert_eq!(list.len(), 1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pop_back() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_back(), None);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn pop_front() {
        let mut list = LinkedList::new();
        assert_eq!(list.pop_front(), None);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn has_one_element() {
        let mut list = LinkedList::new();
        assert!(!list.has_exactly_one_element());
        list.push_back(1);
        assert!(list.has_exactly_one_element());
        list.push_back(2);
        assert!(!list.has_exactly_one_element());
        list.pop_back();
        assert!(list.has_exactly_one_element());
    }

    #[test]
    fn index_of() {
        let mut list = LinkedList::new();
        assert_eq!(list.first_index_of(1), None);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.first_index_of(1), Some(0));
        assert_eq!(list.first_index_of(2), Some(1));
        assert_eq!(list.first_index_of(3), Some(2));
        assert_eq!(list.first_index_of(4), None);

        list.pop_back();
        assert_eq!(list.first_index_of(3), None);
        list.pop_front();
        assert_eq!(list.first_index_of(1), None);
    }

    #[test]
    fn insert_at() {
        let mut list = LinkedList::new();
        list.insert_at(0, 1);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.len(), 0);
        list.insert_at(0, 1);
        list.insert_at(1, 2);
        list.insert_at(2, 3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.len(), 0);
        list.insert_at(0, 1);
        list.insert_at(0, 2);
        list.insert_at(0, 3);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.len(), 0);
        list.insert_at(0, 1);
        list.insert_at(1, 2);
        list.insert_at(2, 3);
        list.insert_at(1, 4);
        assert_eq!(list.len(), 4);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.len(), 0);

        list.insert_at(5, 1);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back(), Some(1));
    }
}
