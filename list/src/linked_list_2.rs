use anyhow::{bail, Result};

#[derive(Default)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn has_exactly_one_element(&self) -> bool {
        self.head.as_ref().is_some_and(|n| n.next.is_none())
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        for _ in self.iter() {
            count += 1;
        }
        count
    }

    pub fn push_back(&mut self, data: T) {
        let mut current = &mut self.head;
        while let Some(node) = current {
            current = &mut node.next;
        }
        *current = Node::new_boxed(data, None)
    }

    pub fn push_front(&mut self, data: T) {
        if self.is_empty() {
            self.head = Node::new_boxed(data, None);
            return;
        }

        self.head = Node::new_boxed(data, self.head.take());
    }

    pub fn pop_back(&mut self) -> Option<T> {
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

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn first_index_of(&self, data: T) -> Option<usize>
    where
        T: PartialEq,
    {
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
    pub fn insert_at(&mut self, index: usize, data: T) {
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

        let current = current.as_mut().expect("None case already handled");
        let new_node = Node::new_boxed(data, current.next.take());
        current.next = new_node;
    }

    pub fn split_at(mut self, index: usize) -> Result<(LinkedList<T>, LinkedList<T>)> {
        if index >= self.len() {
            bail!("Index out of bounds");
        }

        // edge cases
        if self.is_empty() {
            return Ok((Self::new(), Self::new()));
        }

        if self.has_exactly_one_element() {
            return Ok((self, Self::new()));
        }

        let split_at_last_element = index == self.len() - 1; // because we have 0 based index
        if split_at_last_element {
            return Ok((self, Self::new()));
        }

        let mut current = &mut self.head;
        for _ in 0..index {
            if let Some(node) = current {
                current = &mut node.next;
            }
        }

        let split_node = current.as_mut().expect(
            "Current here must exist, as case of splitting at last element was already handled",
        );
        let second_list = Self {
            head: split_node.next.take(),
        };

        Ok((self, second_list))
    }

    pub fn merge(&mut self, other: Self) {
        match (&self.head, &other.head) {
            (None, None) => return,
            (Some(_), None) => return,
            (None, Some(_)) => {
                self.head = other.head;
                return;
            }
            (_, _) => {}
        }

        let mut current = self
            .head
            .as_mut()
            .expect("None case already handled, this cannot be None");

        while let Some(ref mut node_next) = current.next {
            current = node_next;
        }

        current.next = other.head;
    }

    pub fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            current: &self.head,
        }
    }

    pub fn iter_mut(&mut self) -> LinkedListIterMut<T> {
        LinkedListIterMut {
            current: self.head.as_mut(),
        }
    }
}
impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        LinkedListIntoIter(self)
    }
}

pub struct LinkedListIntoIter<T>(LinkedList<T>);

impl<T> Iterator for LinkedListIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub struct LinkedListIter<'a, T> {
    current: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            None => None,
            Some(node) => {
                self.current = &node.next;
                Some(&node.data)
            }
        }
    }
}

pub struct LinkedListIterMut<'a, T> {
    current: Option<&'a mut Box<Node<T>>>,
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|n| {
            self.current = n.next.as_mut();
            &mut n.data
        })
    }
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node { data, next: None }
    }

    fn new_boxed(data: T, next: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
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

    #[test]
    fn push_str() {
        let mut list = LinkedList::new();
        list.push_back("a");
        list.push_back("b");
        list.push_back("c");
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some("c"));
        assert_eq!(list.pop_back(), Some("b"));
        assert_eq!(list.pop_back(), Some("a"));
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn push_point() {
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        let mut list = LinkedList::new();
        list.push_back(Point { x: 1, y: 2 });
        list.push_back(Point { x: 2, y: 3 });
        list.push_back(Point { x: 3, y: 4 });
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(Point { x: 3, y: 4 }));
        assert_eq!(list.len(), 2);

        let index_of = list.first_index_of(Point { x: 2, y: 3 });
        assert_eq!(index_of, Some(1));

        let index_of = list.first_index_of(Point { x: 3, y: 4 });
        assert_eq!(index_of, None);
    }

    #[test]
    fn split_at() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        let (mut first, mut second) = list.split_at(0).unwrap();
        assert_eq!(first.len(), 1);
        assert_eq!(second.len(), 3);
        assert_eq!(first.pop_back(), Some(1));
        assert_eq!(second.pop_front(), Some(2));
        assert_eq!(second.pop_back(), Some(4));

        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        let (mut first, mut second) = list.split_at(1).unwrap();
        assert_eq!(first.len(), 2);
        assert_eq!(second.len(), 2);
        assert_eq!(first.pop_back(), Some(2));
        assert_eq!(second.pop_front(), Some(3));

        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        let (mut first, mut second) = list.split_at(2).unwrap();
        assert_eq!(first.len(), 3);
        assert_eq!(second.len(), 1);
        assert_eq!(first.pop_back(), Some(3));
        assert_eq!(second.pop_front(), Some(4));

        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        let (mut first, mut second) = list.split_at(3).unwrap();
        assert_eq!(first.len(), 4);
        assert_eq!(second.len(), 0);
        assert_eq!(first.pop_back(), Some(4));
        assert_eq!(second.pop_back(), None);

        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        let r = list.split_at(4);
        assert!(r.is_err())
    }

    #[test]
    fn merge() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut list2 = LinkedList::new();
        list2.push_back(4);
        list2.push_back(5);
        list2.push_back(6);
        list.merge(list2);
        assert_eq!(list.len(), 6);
        assert_eq!(list.pop_back(), Some(6));
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));

        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let list2 = LinkedList::new();
        list.merge(list2);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(1));

        let mut list = LinkedList::new();
        let mut list2 = LinkedList::new();
        list2.push_back(4);
        list2.push_back(5);
        list2.push_back(6);
        list.merge(list2);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(6));

        let mut list: LinkedList<()> = LinkedList::new();
        let list2 = LinkedList::new();
        list.merge(list2);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        *iter.next().unwrap() = 4;

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&4));
    }
}
