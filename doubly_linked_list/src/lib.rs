/***
 * Doubly linked list implementation
 ***/
#![forbid(unsafe_code)]
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, Default)]
struct Node<T: std::fmt::Debug + Default + Clone + PartialEq> {
    key: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}
//New up a new node with a value
impl<T: std::fmt::Debug + Default + Clone + PartialEq> Node<T> {
    pub fn new(key: T) -> Self {
        Self {
            key,
            next: None,
            prev: None,
        }
    }
}
//We implement `From` trait to wrap  up a node in RefCell and Rc
impl<T: std::fmt::Debug + Default + Clone + PartialEq> From<Node<T>>
    for Option<Rc<RefCell<Node<T>>>>
{
    fn from(node: Node<T>) -> Self {
        Some(Rc::new(RefCell::new(node)))
    }
}

#[derive(Debug)]
pub struct List<T: std::fmt::Debug + Default + Clone + PartialEq> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T: std::fmt::Debug + Default + Clone + PartialEq> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::fmt::Debug + Default + Clone + PartialEq> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    //Push to the front of the list
    pub fn push_front(&mut self, key: T) {
        let node = Node::new(key).into();
        match self.head {
            None => {
                self.head = node;
                self.tail = self.head.as_ref().map(Rc::clone);
            }
            Some(ref mut head) => {
                head.borrow_mut().prev = node.as_ref().map(|node| Rc::downgrade(&Rc::clone(node)));
                self.head = node.map(|node| {
                    node.borrow_mut().next = Some(Rc::clone(head));
                    node
                });
            }
        }
        self.size += 1;
    }
    //Push to the back of the list
    pub fn push_back(&mut self, key: T) {
        let mut node = Node::new(key).into();
        match self.tail {
            None => {
                self.head = node;
                self.tail = self.head.as_ref().map(Rc::clone);
            }
            Some(ref mut tail) => {
                self.tail = node.take().map(|node| {
                    node.borrow_mut().prev = Some(Rc::downgrade(&Rc::clone(tail)));
                    tail.borrow_mut().next = Some(Rc::clone(&node));
                    node
                });
            }
        }
        self.size += 1;
    }

    //Pop out from the front of the list
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(ref mut head) => {
                self.head = head.borrow_mut().next.take().map(|next| {
                    next.borrow_mut().prev.take();
                    next
                });
                if self.head.is_none() {
                    self.tail.take();
                }
                self.size -= 1;
                //Use of default
                Some(head.take().key)
            }
        }
    }

    //Pop out from the back of the list
    pub fn pop_back(&mut self) -> Option<T> {
        match self.tail.take() {
            None => None,
            Some(ref mut tail) => {
                self.tail = tail.borrow().prev.as_ref().and_then(|prev| {
                    prev.upgrade().map(|prev| {
                        prev.borrow_mut().next = None;
                        prev
                    })
                });
                if self.tail.is_none() {
                    self.head.take();
                }
                self.size -= 1;
                //Use of default
                Some(tail.take().key)
            }
        }
    }

    //Is the passed in node reference the first in the list?
    fn is_first(&self, node: &Rc<RefCell<Node<T>>>) -> bool {
        match self.head {
            None => false,
            Some(ref head) => Rc::ptr_eq(head, node),
        }
    }

    //Is the passed in node reference the last in the list?
    fn is_last(&self, node: &Rc<RefCell<Node<T>>>) -> bool {
        match self.tail {
            None => false,
            Some(ref tail) => Rc::ptr_eq(tail, node),
        }
    }

    //Delete a node that has previous and next
    fn delete_inner(&mut self, target: &Rc<RefCell<Node<T>>>) -> Option<T> {
        let prev = target.borrow_mut().prev.take();
        let next = target.borrow_mut().next.take();
        if let Some(ref next) = next {
            next.borrow_mut().prev = prev.as_ref().cloned();
        }
        if let Some(ref prev) = prev {
            if let Some(prev) = prev.upgrade() {
                prev.borrow_mut().next = next.as_ref().map(Rc::clone);
            }
        }
        self.size -= 1;
        Some(target.take().key)
    }

    //Delete a key from the list. We try to find the by using iterator `find` method.
    //If found - we check if it is the first or last key in the list. If the found node
    //happens to be first or last - we call pop_front or pop_back as required.
    //If the key is in between head and and tail - the deletion is handled accordingly.
    pub fn delete(&mut self, key: &T) -> Option<T> {
        match self.node_iter().find(|node| node.borrow().key == *key) {
            None => None,
            Some(ref target) => match (self.is_first(target), self.is_last(target)) {
                (true, true) | (true, false) => self.pop_front(),
                (false, true) => self.pop_back(),
                (_, _) => self.delete_inner(target),
            },
        }
    }
    //Does a given key exist in the list
    pub fn exists(&self, key: &T) -> bool {
        self.node_iter()
            .find(|node| node.borrow().key == *key)
            .is_some()
    }

    pub fn insert_after(&mut self, key: T, after: &T) -> bool {
        match self.node_iter().find(|node| node.borrow().key == *after) {
            None => false,
            Some(ref mut after) => {
                let mut after_next = after.borrow().next.as_ref().map(Rc::clone);
                let mut new_node = Node::new(key);
                new_node.next = after_next.as_ref().map(Rc::clone);
                new_node.prev = Some(Rc::downgrade(&Rc::clone(after)));
                let new_node = Rc::new(RefCell::new(new_node));
                if let Some(ref mut after_next) = after_next {
                    after_next.borrow_mut().prev = Some(Rc::downgrade(&Rc::clone(&new_node)));
                }
                after.borrow_mut().next = Some(new_node);

                self.size += 1;
                true
            }
        }
    }

    fn move_to_front(&mut self, node: Option<&Rc<RefCell<Node<T>>>>) {
        let key = match node {
            None => return,
            Some(node) => node.borrow().key.clone(),
        };
        let deleted = self.delete(&key);
        if let Some(deleted) = deleted {
            self.push_front(deleted);
        }
    }

    //Returns a forward iterator that is used internally.
    fn node_iter(&self) -> NodeIterator<T> {
        NodeIterator {
            head: self.head.as_ref().map(Rc::clone),
            tail: self.tail.as_ref().map(Rc::clone),
            ptr_crossed: false,
        }
    }
    //Returns an iterator for public consumption. We are breaking rust convention here. Instead of
    //returning Option<&T>, we return Option<T> when we call `next` on this iterator.
    //We are cloning T.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            //We are calling above helper function here
            nodes: self.node_iter(),
        }
    }

    pub fn into_iter(&mut self) -> IntoIterator<'_, T> {
        //Taking a mutable reference to the list
        IntoIterator { list: self }
    }
}

pub struct Iter<T: std::fmt::Debug + Default + Clone + PartialEq> {
    nodes: NodeIterator<T>,
}

//Itearor that returns Option<T>
//Values are cloned
//Underlying list remain intact
impl<T: std::fmt::Debug + Default + Clone + PartialEq> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes
            .next()
            .as_ref()
            .map(|next| next.borrow().key.clone())
    }
}
//Iterate back
//Calling next_back should not returned elements seen by calling next and vice versa
impl<T: std::fmt::Debug + Default + Clone + PartialEq> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.nodes
            .next_back()
            .as_ref()
            .map(|prev| prev.borrow().key.clone())
    }
}

pub struct IntoIterator<'a, T: std::fmt::Debug + Default + Clone + PartialEq> {
    list: &'a mut List<T>,
}

//Iterator that consumes the list elements from the front
impl<'a, T: std::fmt::Debug + Default + Clone + PartialEq> Iterator for IntoIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<'a, T: std::fmt::Debug + Default + Clone + PartialEq> DoubleEndedIterator
    for IntoIterator<'a, T>
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

//This struct holds head and tail of the list
//ptr_crossed flag indicates whether front and back iterators crossed each other
struct NodeIterator<T: std::fmt::Debug + Default + Clone + PartialEq> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    ptr_crossed: bool,
}

impl<T: std::fmt::Debug + Default + Clone + PartialEq> Iterator for NodeIterator<T> {
    type Item = Rc<RefCell<Node<T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            Some(_) => {
                match self
                    .head
                    .as_ref()
                    .map(|next| (Rc::clone(next), next.borrow().next.as_ref().map(Rc::clone)))
                {
                    None => None,
                    Some(this_and_next) => match self.ptr_crossed {
                        true => None,
                        false => {
                            let this = this_and_next.0;
                            let next = this_and_next.1;
                            self.ptr_crossed = self
                                .tail
                                .as_ref()
                                .map(|tail| Rc::ptr_eq(&this, tail))
                                .unwrap_or(false);
                            self.head = next;
                            Some(this)
                        }
                    },
                }
            }
            None => None,
        }
    }
}

impl<T: std::fmt::Debug + Default + Clone + PartialEq> DoubleEndedIterator for NodeIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.tail {
            Some(_) => {
                match self.tail.as_ref().map(|tail| {
                    (
                        Rc::clone(tail),
                        tail.borrow()
                            .prev
                            .as_ref()
                            .cloned()
                            .and_then(|prev| prev.upgrade()),
                    )
                }) {
                    None => None,
                    Some(this_and_prev) => match self.ptr_crossed {
                        true => None,
                        false => {
                            let this = this_and_prev.0;
                            let prev = this_and_prev.1;
                            self.ptr_crossed = self
                                .head
                                .as_ref()
                                .map(|head| Rc::ptr_eq(&this, head))
                                .unwrap_or(false);
                            self.tail = prev;
                            Some(this)
                        }
                    },
                }
            }
            None => None,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_push_and_pop_front() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
    #[test]
    fn test_push_and_pop_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_into_iter() {
        let mut list = List::new();
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
    fn test_into_iter_both_ends() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn test_delete() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.delete(&100);
        list.delete(&1);
        list.delete(&3);
        list.delete(&2);
    }

    #[test]
    fn test_delete_inner() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.delete(&2), Some(2));
        println!("The list: {:?}", list);
        assert_eq!(list.delete(&1), Some(1));
        println!("The list: {:?}", list);
        assert_eq!(list.delete(&3), Some(3));
        println!("The list: {:?}", list);
    }

    #[test]
    fn test_double_ended_iterator() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }
    #[test]
    fn test_exists() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert!(list.exists(&1));
        assert!(list.exists(&2));
        assert!(list.exists(&3));
        assert!(!list.exists(&4));
    }
    #[test]
    fn test_insert_after() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(3);
        list.insert_after(2, &1);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_move_to_front() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        let mut iter = list.node_iter();
        let third = iter.next_back();
        println!("The third = {:?}", third);
        list.move_to_front(third.as_ref());
        //list.move_to_front(&3);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_size() {
        let mut list = List::new();
        assert_eq!(list.size(), 0);
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.size(), 3);
        let _ = list.pop_front();
        assert_eq!(list.size(), 2);
    }
}
