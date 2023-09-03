#[cfg(test)]
use crate::linked_list_1::LinkedList;

#[test]
fn push_back_and_front() {
    let mut list = LinkedList::new();
    assert_eq!(list.len(), 0);

    list.push_back(1);
    list.push_back(2);

    // assert_eq!(
    //     list,
    //     LinkedList::Elem(
    //         1,
    //         Box::new(LinkedList::Elem(2, Box::new(LinkedList::Empty)))
    //     )
    // );

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
