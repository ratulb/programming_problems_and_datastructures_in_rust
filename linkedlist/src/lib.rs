use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;
#[derive(Debug, Default)]
pub struct LinkedList<T: std::fmt::Debug + Default + Clone + Ord> {
    head: Link<T>,
    len: usize,
}

#[derive(Debug, Default, PartialEq)]
struct Node<T: std::fmt::Debug + Default + Clone + Ord> {
    value: T,
    next: Link<T>,
}

impl<T: std::fmt::Debug + Default + Clone + Ord> Node<T> {
    fn new(value: T) -> Node<T> {
        Node { value, next: None }
    }

    fn wrapped(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(value)))
    }

    fn wrappred_with_link(value: T, link: &Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let with_next_link = Node {
            value,
            next: Some(Rc::clone(link)),
        };
        Rc::new(RefCell::new(with_next_link))
    }

    fn replace(&mut self, value: T) -> T {
        std::mem::replace(&mut self.value, value)
    }

    fn swap(this: &mut Self, that: &mut Self) {
        std::mem::swap(&mut this.value, &mut that.value);
    }

    fn swap_with_next(mut prev: Option<Rc<RefCell<Self>>>) {
        if let Some(ref mut prev_inner) = prev {
            let mut prev_borrowed = prev_inner.borrow_mut();
            if let Some(ref mut next) = prev_borrowed.next.as_ref().cloned() {
                Self::swap(&mut prev_borrowed, &mut next.borrow_mut());
            }
        }
    }

    fn ordered(node: Option<Rc<RefCell<Self>>>) -> bool {
        node.and_then(|node| {
            node.borrow()
                .next
                .as_ref()
                .map(|next| next.borrow().value > node.borrow().value)
        })
        //.unwrap_or(true)
        .is_some_and(|ordered| ordered == true)
    }
}

impl<T: std::fmt::Debug + Default + Clone + Ord> LinkedList<T> {
    pub fn new(value: T) -> LinkedList<T> {
        let link = Some(Node::wrapped(value));
        LinkedList { head: link, len: 1 }
    }

    //A no head list
    pub fn empty() -> Self {
        LinkedList { head: None, len: 0 }
    }

    //Push to the front of the list - making current head the next link if it exists
    //Or else set the head if it the list is empty
    pub fn push_front(&mut self, value: T) {
        match self.head.take() {
            Some(head_link) => {
                self.head = Some(Node::wrappred_with_link(value, &head_link));
            }
            None => {
                self.head = Some(Node::wrapped(value));
            }
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(outgoing_head) => {
                self.head = outgoing_head.borrow().next.as_ref().cloned();
                self.len -= 1;
                Some(outgoing_head.take().value)
            }
            None => None,
        }
    }
    //Replace  a list entry based on its key value
    pub fn replace(&mut self, key: &T, new_value: T) -> Option<T> {
        self.link_iterator()
            .find(|link| link.borrow().value == *key)
            .as_mut()
            .map(|link| link.borrow_mut().replace(new_value))
    }

    //Push to the back of the list - o(n) operation
    pub fn push_back(&mut self, value: T) {
        match self.is_empty() {
            true => {
                self.push_front(value);
            }
            false => {
                let mut back = self
                    .link_iterator()
                    .enumerate()
                    .skip_while(|(index, _)| index != &(self.len() - 1));
                if let Some(ref mut back) = back.next() {
                    let new_back = Some(Node::wrapped(value));
                    back.1.borrow_mut().next = new_back.as_ref().cloned();
                    self.len += 1;
                }
            }
        }
    }

    pub fn insert_before(&mut self, index: usize, value: Option<T>) {
        match (index, value) {
            (_, None) => (),
            (idx, _) if idx >= self.len() => (),
            (idx, Some(val)) if idx == 0 => self.push_front(val),
            (idx, Some(val)) => {
                let prev = self
                    .link_iterator()
                    .enumerate()
                    .skip_while(|(idex, _)| idex != &(idx - 1))
                    .take(1)
                    .next()
                    .map(|(_, link)| link);
                let next = self
                    .link_iterator()
                    .enumerate()
                    .skip_while(|(idex, _)| idex != &idx)
                    .take(1)
                    .next()
                    .map(|(_, link)| link);
                let node = Node::wrapped(val);
                if let Some(previous) = prev {
                    previous.borrow_mut().next = Some(Rc::clone(&node));
                }
                node.borrow_mut().next = next.as_ref().cloned();
                self.len += 1;
            }
        }
    }

    pub fn delete(&mut self, index: usize) -> Option<T> {
        match index {
            idx if idx >= self.len() => None,
            idx if idx == 0 => self.pop_front(),
            idx if idx == self.len() - 1 => self.pop_back(),
            _ => {
                let prev = self
                    .link_iterator()
                    .enumerate()
                    .skip_while(|(idx, _)| idx != &(index - 1))
                    .take(1)
                    .next()
                    .map(|(_, link)| link);

                let elem = self
                    .link_iterator()
                    .enumerate()
                    .skip_while(|(idx, _)| idx != &index)
                    .take(1)
                    .next()
                    .map(|(_, link)| link);

                let next = self
                    .link_iterator()
                    .enumerate()
                    .skip_while(|(idx, _)| idx != &(index + 1))
                    .take(1)
                    .next()
                    .map(|(_, link)| link);

                if let Some(previous) = prev {
                    previous.borrow_mut().next = next.as_ref().cloned();
                }
                self.len -= 1;
                elem.map(|link| link.take().value)
            }
        }
    }

    //Pop back from the end of the list - o(n) operation
    pub fn pop_back(&mut self) -> Option<T> {
        match self.len() {
            0 => None,
            1 => self.pop_front(),
            _ => {
                let mut back_predecessor = self
                    .link_iterator()
                    .enumerate()
                    .skip_while(|(index, _)| index != &(self.len() - 2));
                let last = back_predecessor.next().as_mut().and_then(|predecessor| {
                    predecessor
                        .1
                        .borrow_mut()
                        .next
                        .take()
                        .map(|node| node.take().value)
                });
                self.len -= 1;
                last
            }
        }
    }
    //Reverse the list
    pub fn reverse(&mut self) {
        if self.len < 2 {
            return;
        }
        let mut previous = None;
        let mut current = self.head.take();
        while let Some(ref mut curr_inner) = current {
            let next = curr_inner.borrow_mut().next.take();
            curr_inner.borrow_mut().next = previous;
            previous = current;
            current = next;
        }
        self.head = previous;
    }

    pub fn insert_sorted(&mut self, value: T, ascending: bool) {
        if self.is_empty() {
            self.push_front(value);
            return;
        }
        let mut prev = None;
        let insert_at = self
            .link_iterator()
            .map(|link| {
                if prev.is_none() {
                    prev = Some(Rc::clone(&link));
                    if ascending {
                        (None, link.borrow().value >= value, Rc::clone(&link))
                    } else {
                        (None, link.borrow().value <= value, Rc::clone(&link))
                    }
                } else {
                    let curr_prev = prev.as_ref().cloned();
                    prev = Some(Rc::clone(&link));
                    if ascending {
                        (curr_prev, link.borrow().value >= value, Rc::clone(&link))
                    } else {
                        (curr_prev, link.borrow().value <= value, Rc::clone(&link))
                    }
                }
            })
            .find(|(_, greater, _)| greater == &true)
            .map(|(prev, _, next)| (prev, next));
        match insert_at {
            None => self.push_back(value),
            Some((None, _)) => self.push_front(value),
            Some((mut prev, ref next)) => {
                let new_entry = Some(Node::wrappred_with_link(value, next));
                if let Some(ref mut prev) = prev {
                    prev.borrow_mut().next = new_entry.as_ref().cloned();
                    self.len += 1;
                }
            }
        }
    }

    //Sort the list with bubble sort
    pub fn bubble_sort(&mut self) {
        if self.len() < 2 {
            return;
        }
        let len = self.len() - 1;
        for i in 0..len {
            let mut current = self.head.as_ref().cloned();
            for _ in 0..(len  - i) {
                let ordered = Node::ordered(current.as_ref().cloned());
                if !ordered {
                    Node::swap_with_next(current.as_ref().cloned());
                }
                current = current.and_then(|current| current.borrow().next.as_ref().cloned());
            }
        }
    }
    //Sort the list using selection sort (slow)
    pub fn selection_sort(&mut self) {
        if self.len() < 2 {
            return;
        }
        self.link_iterator()
            .enumerate()
            .for_each(|(index, current)| {
                self.link_iterator()
                    .enumerate()
                    .skip_while(|(index_inner, _)| index_inner <= &index)
                    .map(|(_, node)| node)
                    .for_each(|node| {
                        let unordered = current.borrow().value >= node.borrow().value;
                        if unordered {
                            Node::swap(&mut current.borrow_mut(), &mut node.borrow_mut());
                        }
                    });
            });
    }
    //Sort the linked list via insertion sort
    pub fn insertion_sort(&mut self, ascending: bool) {
        if self.len() < 2 {
            return;
        }
        let mut current = self.head.take();
        self.len = 0;
        while let Some(curr) = current {
            let mut node = curr.take();
            self.insert_sorted(node.value, ascending);
            current = node.next.take();
        }
    }
    pub fn quicksort(&mut self, _ascending: bool) {
        let len = self.len() - 1;
        Self::quick_sort(self, _ascending, 0, len);
    }

    fn quick_sort(&mut self, _ascending: bool, left: usize, right: usize) {
        if left >= right {
            return;
        }
        let pivot = Self::partition(self, _ascending, left, right);
        if pivot > 0 {
            Self::quick_sort(self, _ascending, left, pivot - 1);
        }
        Self::quick_sort(self, _ascending, pivot + 1, right);
    }

    fn partition_elements(
        &mut self,
        left: usize,
        right: usize,
        pivot: Link<T>,
    ) -> Option<impl Iterator<Item = (usize, Rc<RefCell<Node<T>>>)>> {
        println!("The PIVOT is = {:?}, left = {}, right = {}", pivot, left, right);
        let elements = self
            .link_iterator()
            .enumerate()
            .skip_while(move |(index, _)| index == &(left))
            .take_while(move |(index, _)| index != &(right+1));
        Some(
            elements
                .filter(move |(_, elem)| {
                    pivot
                        .as_ref()
                        .map(|pivot| elem.borrow().value <= pivot.borrow().value)
                        .unwrap_or(false)
                })
                .map(|(index, link)| (index, link)),
        )
    }

    fn partition(&mut self, _ascending: bool, left: usize, right: usize) -> usize {
        let pivot = self
            .link_iterator()
            .enumerate()
            .skip_while(|(index, _)| index != &left)
            .take(1)
            .next()
            .map(|(_, link)| {
                //link.borrow_mut().next = None;
                link
            });

        println!("Pivot is = {:?}", pivot);
        let elements = self.partition_elements(left, right, pivot);
        let mut deleted = vec![];
        if let Some(elems) = elements {
            for (index, _link) in elems {
                println!("Index = {:?}", index);
                if let Some(value) = self.delete(index) {
                    deleted.push(Some(value));
                }
            }
        }
        println!("Deleted = {:?}", deleted);
        //let pivot = deleted.len() + left;
        let mut count = left;
        for dele in deleted {
            self.insert_before(count, dele);
            count += 1;
        }
        count
    }

    pub fn mergesort(&mut self) {
        if self.len() < 2 {
            return;
        }
        let mut splits = Self::split_at_mid(std::mem::take(self));
        if let Some((ref mut left, ref mut right)) = splits {
            Self::mergesort(left);
            Self::mergesort(right);
            *self = Self::merge(std::mem::take(left), std::mem::take(right), true);
        }
    }
    //Is the list sorted in ascending or descending order?
    pub fn is_sorted(&self, ascending: bool) -> bool {
        if self.len() < 2 {
            return true;
        }
        let mut first = None;
        for t in self.iter() {
            match first {
                None => first = Some(t),
                Some(prev) => match ascending {
                    true if prev > t => return false,
                    false if prev < t => return false,
                    _ => first = Some(t),
                },
            }
        }
        true
    }

    //Split the list at the given index `at` - the list would be consumed!
    pub fn split_at(mut self, at: usize) -> Option<(Self, Self)> {
        match at {
            pos if pos >= self.len() => None,
            pos if pos == 0 => Some((Self::empty(), self)),
            _ => {
                let mut index = 0;
                let current = &mut self.head.take();
                self.len = 0;
                while let Some(curr) = current {
                    if index == at {
                        break;
                    }
                    let mut node = curr.take();
                    self.push_back(node.value);
                    *current = node.next.take();
                    index += 1;
                }
                let mut right = Self::empty();
                while let Some(curr) = current {
                    let mut node = curr.take();
                    right.push_back(node.value);
                    *current = node.next.take();
                }
                Some((self, right))
            }
        }
    }
    //Merge two sorted list
    pub fn merge(this: Self, that: Self, ascending: bool) -> Self {
        match (this.is_empty(), that.is_empty()) {
            (true, true) => return Self::empty(),
            (false, true) => return this,
            (true, false) => return that,
            (_, _) => {}
        }
        assert!(this.is_sorted(ascending));
        assert!(that.is_sorted(ascending));
        let mut this = this.head;
        let mut that = that.head;
        let mut merged = Self::empty();
        while let (Some(this_inner), Some(that_inner)) = (&mut this, &mut that) {
            let this_smaller = this_inner.borrow().value < that_inner.borrow().value;
            if this_smaller {
                let mut node = this_inner.take();
                merged.push_back(node.value);
                this = node.next.take();
                that = Some(Rc::clone(that_inner));
            } else {
                let mut node = that_inner.take();
                merged.push_back(node.value);
                that = node.next.take();
                this = Some(Rc::clone(this_inner));
            }
        }
        while let Some(inner) = this {
            let mut node = inner.take();
            merged.push_back(node.value);
            this = node.next.take();
        }
        while let Some(inner) = that {
            let mut node = inner.take();
            merged.push_back(node.value);
            that = node.next.take();
        }

        merged
    }

    //Split the list into two halves - first containing 0..len/2 and
    //second len/2..len - right index not included
    pub fn split_at_mid(self) -> Option<(Self, Self)> {
        let mid = self.len() / 2;
        self.split_at(mid)
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn first(&self) -> Option<T> {
        self.head.as_ref().map(|node| node.borrow().value.clone())
    }
    //o(n) operation
    pub fn last(&self) -> Option<T> {
        self.iter().last()
    }

    //An iterator that consumes that list
    pub fn iter_into(self) -> IteratorInto<T> {
        IteratorInto(self)
    }
    //An iterator used internally
    fn link_iterator(&self) -> LinkIterator<T> {
        LinkIterator {
            link: self.head.as_ref().map(Rc::clone),
        }
    }
    //Returns an that does not mutate the underlying list
    //Note: Values returned are Option<T> !NOT! Option<&T>
    pub fn iter(&self) -> Iter<T> {
        Iter {
            links: self.link_iterator(),
        }
    }
}

impl<T: std::fmt::Debug + Default + Clone + Ord> PartialEq for LinkedList<T> {
    fn eq(&self, that: &LinkedList<T>) -> bool {
        let this = self.iter();
        let mut that = that.iter();
        for ref this_next in this {
            match that.next() {
                None => return false,
                Some(ref that_next) if this_next == that_next => continue,
                Some(_) => return false,
            }
        }
        true
    }
}

//Consumes the list
pub struct IteratorInto<T: std::fmt::Debug + Default + Clone + Ord>(LinkedList<T>);

impl<T: std::fmt::Debug + Default + Clone + Ord> Iterator for IteratorInto<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub struct Iter<T: std::fmt::Debug + Default + Clone + Ord> {
    links: LinkIterator<T>,
}
//Iterator that does not consume the list
impl<T: std::fmt::Debug + Default + Clone + Ord> Iterator for Iter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.links
            .next()
            .as_ref()
            .map(|link| link.borrow().value.clone())
    }
}
//An iterator that is used interally
#[derive(Debug)]
struct LinkIterator<T: std::fmt::Debug + Default + Clone + Ord> {
    link: Link<T>,
}
impl<T: std::fmt::Debug + Default + Clone + Ord> Iterator for LinkIterator<T> {
    type Item = Rc<RefCell<Node<T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.link.take() {
            None => None,
            Some(link) => {
                self.link = link.borrow_mut().next.as_ref().map(Rc::clone);
                Some(link)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    fn test_push_and_pop_front() {
        let mut ll = LinkedList::empty();
        assert_eq!(ll.pop_front(), None);
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);
        assert_eq!(ll.pop_front(), Some(3));
        assert_eq!(ll.pop_front(), Some(2));
        assert_eq!(ll.pop_front(), Some(1));
        assert_eq!(ll.pop_front(), None);
    }
    #[test]
    fn test_iter() {
        let mut ll = LinkedList::new(1);
        ll.push_front(2);
        ll.push_front(3);
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        //iter does not consume the list
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_reverse() {
        let mut ll = LinkedList::empty();
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        ll.reverse();
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

        let mut iter = ll.iter_into();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_into() {
        let mut ll = LinkedList::new(1);
        ll.push_front(2);
        ll.push_front(3);
        //Consumes the list
        let mut iter = ll.iter_into();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn replace_test() {
        let mut ll = LinkedList::new(1);
        ll.push_front(2);
        ll.push_front(3);
        //Consumes the list
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        ll.replace(&1, 100);
        ll.replace(&2, 200);
        ll.replace(&3, 300);

        let mut iter = ll.iter();

        assert_eq!(iter.next(), Some(300));
        assert_eq!(iter.next(), Some(200));
        assert_eq!(iter.next(), Some(100));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_push_back() {
        let mut ll = LinkedList::empty();
        ll.push_back(1);
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(1));
        ll.push_back(2);
        ll.push_back(3);

        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }
    #[test]
    fn test_pop_back() {
        let mut ll = LinkedList::empty();
        assert_eq!(ll.pop_back(), None);
        ll.push_back(1);
        assert_eq!(ll.pop_back(), Some(1));
        assert_eq!(ll.pop_back(), None);
        ll.push_back(1);
        ll.push_back(2);

        assert_eq!(ll.pop_back(), Some(2));
        assert_eq!(ll.pop_back(), Some(1));
        assert_eq!(ll.pop_back(), None);

        let mut iter = ll.iter();
        assert_eq!(iter.next(), None);

        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);

        assert_eq!(ll.pop_back(), Some(1));
        assert_eq!(ll.pop_back(), Some(2));
        assert_eq!(ll.pop_back(), Some(3));
    }

    #[test]
    fn test_equality() {
        let ll = LinkedList::new(100);
        assert_eq!(LinkedList::new(100), ll);
        assert_ne!(LinkedList::new(101), ll);
    }
    #[test]
    fn test_bubble_sort() {
        let mut ll = LinkedList::new(-10);
        ll.push_front(42);
        ll.push_front(21);
        ll.push_front(21);
        ll.push_front(90);
        ll.push_front(200);

        let mut iter = ll.iter();

        assert_eq!(iter.next(), Some(200));
        assert_eq!(iter.next(), Some(90));
        assert_eq!(iter.next(), Some(21));
        assert_eq!(iter.next(), Some(21));
        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), Some(-10));
        assert_eq!(iter.next(), None);

        ll.bubble_sort();
        let mut iter = ll.iter();

        assert_eq!(iter.next(), Some(-10));
        assert_eq!(iter.next(), Some(21));
        assert_eq!(iter.next(), Some(21));
        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), Some(90));
        assert_eq!(iter.next(), Some(200));
        assert_eq!(iter.next(), None);

        let mut ll = LinkedList::new(-10);
        ll.bubble_sort();
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(-10));
        assert_eq!(iter.next(), None);

        let mut ll = LinkedList::new(-10);
        ll.push_front(200);
        ll.bubble_sort();
        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(-10));
        assert_eq!(iter.next(), Some(200));
        assert_eq!(iter.next(), None);

        let mut ll = LinkedList::<i32>::empty();
        ll.bubble_sort();
        let mut iter = ll.iter();
        assert_eq!(iter.next(), None);

    }
    #[test]
    fn test_selction_sort() {
        let mut ll = LinkedList::new(-10);
        ll.push_front(42);
        ll.push_front(21);
        ll.push_front(200);
        ll.push_front(21);
        ll.push_front(90);
        ll.push_front(200);

        let mut iter = ll.iter();

        assert_eq!(iter.next(), Some(200));
        assert_eq!(iter.next(), Some(90));
        assert_eq!(iter.next(), Some(21));
        assert_eq!(iter.next(), Some(200));
        assert_eq!(iter.next(), Some(21));
        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), Some(-10));
        assert_eq!(iter.next(), None);

        ll.selection_sort();
        let mut iter = ll.iter();

        assert_eq!(iter.next(), Some(-10));
        assert_eq!(iter.next(), Some(21));
        assert_eq!(iter.next(), Some(21));
        assert_eq!(iter.next(), Some(42));
        assert_eq!(iter.next(), Some(90));
        assert_eq!(iter.next(), Some(200));
        assert_eq!(iter.next(), Some(200));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn selection_sort_test2() {
        let mut runs = 5;
        loop {
            let mut items: [u16; 128] = [0; 128];
            rand::thread_rng().fill(&mut items);
            let mut ll = LinkedList::empty();
            for elem in items {
                ll.push_front(elem);
            }
            ll.selection_sort();
            if !ll.is_sorted(true) {
                panic!("List is not sorted...");
            }
            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }

    #[test]
    fn test_is_sorted() {
        let mut ll = LinkedList::empty();
        assert!(ll.is_sorted(true));
        assert!(ll.is_sorted(false));
        ll.push_front(1);
        ll.push_back(2);
        assert!(ll.is_sorted(true));
        assert!(!ll.is_sorted(false));

        let mut ll = LinkedList::empty();

        ll.push_front(1);
        ll.push_front(-1);
        ll.push_front(0);
        assert!(!ll.is_sorted(true));
        assert!(!ll.is_sorted(false));
    }
    #[test]
    fn test_first_last() {
        let mut ll = LinkedList::empty();
        assert!(ll.first().is_none());
        assert!(ll.last().is_none());
        ll.push_front(1);
        assert!(ll.first().is_some());
        assert!(ll.last().is_some());
        assert_eq!(ll.first(), Some(1));
        assert_eq!(ll.last(), Some(1));

        ll.push_back(2);
        ll.push_back(3);
        assert_eq!(ll.first(), Some(1));
        assert_eq!(ll.last(), Some(3));
        ll.pop_front();
        assert_eq!(ll.first(), Some(2));
        ll.pop_back();
        assert_eq!(ll.last(), Some(2));
        ll.pop_front();
        assert!(ll.first().is_none());
        assert!(ll.last().is_none());
    }
    #[test]
    fn test_insertion_sort() {
        let mut runs = 5;
        loop {
            let mut items: [u16; 10] = [0; 10];
            rand::thread_rng().fill(&mut items);
            let mut ll = LinkedList::empty();
            for elem in items {
                ll.push_front(elem);
            }
            let len = ll.iter().count();
            ll.insertion_sort(false);
            assert_eq!(len, ll.len());
            if !ll.is_sorted(false) {
                panic!("List is not sorted...");
            }
            runs -= 1;
            if runs == 0 {
                break;
            }
        }
        let mut runs = 5;
        loop {
            let mut items: [u16; 128] = [0; 128];
            rand::thread_rng().fill(&mut items);
            let mut ll = LinkedList::empty();
            for elem in items {
                ll.push_back(elem);
            }
            ll.insertion_sort(true);
            if !ll.is_sorted(true) {
                panic!("List is not sorted...");
            }
            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }

    #[test]
    fn test_insert_sorted() {
        let mut ll = LinkedList::empty();
        ll.push_back(4);
        ll.push_back(5);
        ll.insert_sorted(3, true);
        assert!(ll.is_sorted(true));

        let mut ll = LinkedList::empty();
        ll.insert_sorted(4, true);
        ll.insert_sorted(5, true);
        ll.insert_sorted(3, true);
        assert!(ll.is_sorted(true));

        let mut runs = 5;
        loop {
            let mut items: [u8; 10] = [0; 10];
            rand::thread_rng().fill(&mut items);
            let mut ll = LinkedList::empty();
            for elem in items {
                ll.insert_sorted(elem, false);
            }
            assert_eq!(items.len(), ll.len());
            if !ll.is_sorted(false) {
                panic!("List is not sorted...");
            }
            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }
    #[test]
    fn test_split_at() {
        let mut ll = LinkedList::empty();
        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(3);
        let mut left = LinkedList::empty();
        left.push_back(1);
        let mut right = LinkedList::empty();
        right.push_back(2);
        right.push_back(3);
        let rs = ll.split_at(1);
        assert_eq!(rs, Some((left, right)));
    }

    #[test]
    fn test_split_at_mid() {
        let mut ll = LinkedList::empty();
        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(3);
        let mut left = LinkedList::empty();
        left.push_back(1);
        let mut right = LinkedList::empty();
        right.push_back(2);
        right.push_back(3);
        let rs = ll.split_at(1);
        assert_eq!(rs, Some((left, right)));

        let mut ll = LinkedList::empty();
        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(3);
        ll.push_back(4);
        let mut left = LinkedList::empty();
        left.push_back(1);
        left.push_back(2);
        let mut right = LinkedList::empty();
        right.push_back(3);
        right.push_back(4);
        let rs = ll.split_at_mid();
        assert_eq!(rs, Some((left, right)));

        let mut ll = LinkedList::empty();
        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(3);
        ll.push_back(4);
        ll.push_back(5);
        let mut left = LinkedList::empty();
        left.push_back(1);
        left.push_back(2);
        let mut right = LinkedList::empty();
        right.push_back(3);
        right.push_back(4);
        right.push_back(5);
        let rs = ll.split_at_mid();
        assert_eq!(rs, Some((left, right)));
    }

    #[test]
    fn test_merge() {
        let mut l1 = LinkedList::empty();
        let mut l2 = LinkedList::empty();
        l1.push_back(1);
        l2.push_back(2);
        l1.push_back(3);
        l2.push_back(4);
        l1.push_back(5);
        l2.push_back(6);
        l2.push_back(6);
        l2.push_back(7);
        let mut expected = LinkedList::empty();
        expected.push_back(1);
        expected.push_back(2);
        expected.push_back(3);
        expected.push_back(4);
        expected.push_back(5);
        expected.push_back(6);
        expected.push_back(6);
        expected.push_back(7);
        let result = LinkedList::merge(l1, l2, true);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_mergesort() {
        let mut ll = LinkedList::empty();
        ll.push_back(1);
        ll.push_back(-9);
        ll.push_back(0);
        ll.push_back(1);
        ll.push_back(-7);
        ll.mergesort();
        let mut expected = LinkedList::empty();
        expected.push_back(-9);
        expected.push_back(-7);
        expected.push_back(0);
        expected.push_back(1);
        expected.push_back(1);
        assert_eq!(ll, expected);
    }
    #[test]
    fn test_quicksort() {
        let mut ll = LinkedList::empty();
        ll.push_back(10);
        ll.push_back(9);
        ll.push_back(8);
        /***ll.push_back(7);
        ll.push_back(6);***/
        ll.quicksort(true);
        println!(" At the end ll = {:?}", ll);
    }
    #[test]
    fn test_delete() {
        let mut ll = LinkedList::empty();
        ll.push_back(10);
        ll.push_back(9);
        ll.push_back(8);
        ll.push_back(7);
        ll.push_back(6);
        let index = 0;
        let result = ll.delete(index);
        assert_eq!(result, Some(10));
        assert_eq!(ll.len(), 4);
        println!("Result = {:?}", result);
        println!("After delete index {},  = {:?}", index, ll);

        let index = 2;
        let result = ll.delete(index);
        println!("Result = {:?}", result);
        assert_eq!(result, Some(7));
        assert_eq!(ll.len(), 3);
        println!("After delete index {},  = {:?}", index, ll);

        let index = ll.len() - 1;
        let result = ll.delete(index);
        println!("Result = {:?}", result);
        assert_eq!(result, Some(6));
        assert_eq!(ll.len(), 2);
        println!("After delete index {},  = {:?}", index, ll);
    }
    #[test]
    fn test_insert_before() {
        let mut ll = LinkedList::empty();
        ll.push_back(9);
        ll.push_back(8);
        ll.push_back(7);
        ll.push_back(6);
        let index = 0;
        ll.insert_before(index, Some(10));
        assert_eq!(ll.len(), 5);
        println!("After insert  = {:?}", ll);

        let mut ll = LinkedList::empty();
        ll.push_back(9);
        ll.push_back(8);
        ll.push_back(6);
        let index = 2;
        ll.insert_before(index, Some(7));
        assert_eq!(ll.len(), 4);
        println!("After insert  = {:?}", ll);
    }
}
