#![forbid(unsafe_code)]
use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::fmt::{Debug, Error, Formatter};
use std::ops::{Add, Deref, DerefMut};
use std::rc::Rc;

type Cell<T> = Rc<RefCell<Node<T>>>;
type Link<T> = Option<Cell<T>>;

pub(crate) struct Node<T> {
    elem: T,
    next: Link<T>,
}

///
///The link list structure for arbritary type T. 'T' should have a default value.
///

pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

pub struct NonMutT<'a, T>(Ref<'a, Node<T>>);
pub struct MutT<'a, T>(RefMut<'a, Node<T>>);

impl<'a, T> NonMutT<'a, T> {
    pub fn t(&self) -> &T {
        &self.0.elem
    }

    pub fn next(&self) -> Option<NonMutT<'_, T>> {
        self.0.next.as_ref().map(|next| NonMutT(next.borrow()))
    }
}

impl<'a, T> MutT<'a, T> {
    pub fn t(&mut self) -> &mut T {
        &mut self.0.elem
    }
    pub fn next(&mut self) -> Option<MutT<'_, T>> {
        self.0.next.as_mut().map(|next| MutT(next.borrow_mut()))
    }
}

impl<'a, T> Deref for NonMutT<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0.elem
    }
}

impl<'a, T: PartialOrd> PartialOrd for NonMutT<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.elem.partial_cmp(&other.0.elem)
    }
}

impl<'a, T: PartialEq> PartialEq for NonMutT<'a, T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.elem == other.0.elem
    }
}

impl<'a, T> Deref for MutT<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0.elem
    }
}

impl<T: Default> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.elem
    }
}

impl<'a, T> DerefMut for MutT<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.elem
    }
}

impl<T: Default> Node<T> {
    pub fn new(elem: T) -> Self {
        Self { elem, next: None }
    }

    fn with_link(elem: T, link: Cell<T>) -> Cell<T> {
        Rc::new(RefCell::new(Self {
            elem,
            next: Some(link),
        }))
    }

    #[inline(always)]
    fn rc_cell(elem: T) -> Cell<T> {
        Rc::new(RefCell::new(Self::new(elem)))
    }

    //Is this node in order? i.e. Greater or equal/less or equal it next node's value
    #[inline(always)]
    fn in_order(node: Option<&Cell<T>>, asc: bool) -> bool
    where
        T: PartialOrd,
    {
        node.and_then(|node| {
            node.borrow()
                .next
                .as_ref()
                .map(|next| if asc { next >= node } else { next <= node })
        })
        .unwrap_or(true)
    }
    #[inline(always)]
    fn swap(this: &mut Self, that: &mut Self) {
        std::mem::swap(&mut this.elem, &mut that.elem);
    }

    fn swap_with_next(mut curr_node: Option<&Cell<T>>) {
        if let Some(ref mut node) = curr_node {
            let mut mutable_node = node.borrow_mut();
            if let Some(ref mut next) = mutable_node.next.as_ref().map(Rc::clone) {
                Node::swap(&mut mutable_node, &mut next.borrow_mut());
            }
        }
    }

    //Takes out the value from the node. Replaces it with the default value for type 'T'.
    #[inline(always)]
    fn take(&mut self) -> T {
        std::mem::take(&mut self.elem)
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.next {
            None => write!(f, "{:?}", self.elem),
            Some(ref next) => {
                let _ = write!(f, "{:?} -> ", self.elem);
                next.borrow().fmt(f)
            }
        }
    }
}

impl<T: PartialOrd> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.elem.partial_cmp(&other.elem)
    }
}

impl<T: PartialEq> Eq for Node<T> {}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        match (&self.elem, &self.next, &other.elem, &other.next) {
            (elem, _, other_elem, _) if *elem != *other_elem => false,
            (elem, None, other_elem, None) if *elem == *other_elem => true,
            (elem, None, other_elem, Some(_)) if *elem == *other_elem => false,
            (elem, Some(_), other_elem, None) if *elem == *other_elem => false,
            (elem, Some(ref this), other_elem, Some(ref that)) if *elem == *other_elem => {
                *this.borrow() == *that.borrow()
            }
            (_, _, _, _) => false,
        }
    }
}

impl<T: PartialEq> Eq for LinkedList<T> {}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    #[inline]
    fn eq(&self, other: &LinkedList<T>) -> bool {
        self.head == other.head && self.len == other.len
    }
}

impl<T: Default> LinkedList<T> {
    //New up a list with a single value
    pub fn new(elem: T) -> Self {
        Self {
            head: Some(Node::rc_cell(elem)),
            len: 1,
        }
    }

    //Readily create a list from clonable slice of values. Internally values are never cloned hereafter.
    pub fn from_slice<U: Clone + Default>(elems: &[U]) -> LinkedList<U> {
        let mut list = LinkedList::<U>::default();
        elems.iter().for_each(|elem| list.push_back(elem.clone()));
        list
    }
    //Push value to the front of the list
    pub fn push_front(&mut self, elem: T) {
        match self.head.take() {
            Some(as_link) => self.head = Some(Node::with_link(elem, as_link)),
            None => self.head = Some(Node::rc_cell(elem)),
        }
        self.len += 1;
    }

    //Pop value out from the front of the list - O(1) operation
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.len -= 1;
            self.head = head.borrow_mut().next.take();
            head.borrow_mut().take()
        })
    }
    //Push values to the back of the list. O(n) recursive operation in worst case
    pub fn push_back(&mut self, elem: T) {
        if self.is_empty() {
            self.push_front(elem);
        } else {
            let mut last = self
                .link_iterator()
                .enumerate()
                .skip_while(|(index, _)| index != &(self.len() - 1))
                .map(|(_, cell)| cell)
                .next();

            if let Some(ref mut last) = last {
                last.borrow_mut().next = Some(Node::rc_cell(elem));
                self.len += 1;
            }
        }
    }

    //Pop values from the end of the list - obvious O(n) operation
    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            None
        } else if self.len() == 1 {
            self.len -= 1;
            self.head.take().map(|head| head.borrow_mut().take())
        } else {
            let penultimate = self
                .link_iterator()
                .enumerate()
                .skip_while(|(index, _)| index != &(self.len() - 2))
                .map(|(_, cell)| cell)
                .next();

            penultimate.and_then(|penultimate| {
                penultimate.borrow_mut().next.take().map(|last| {
                    self.len -= 1;
                    last.borrow_mut().take()
                })
            })
        }
    }

    //Count of values in the list
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    //Convert to another list applying a function that takes reference to values
    pub fn translate<U: Default, F: Fn(&T) -> U>(&self, f: F) -> LinkedList<U> {
        let mut result = LinkedList::default();
        let iter = self.link_iterator();
        for t in iter {
            result.push_back(f(&t.borrow().elem));
        }
        result
    }
    //Mutate the list applying a function to the mutable values of the list. The value would be
    //changed to the return value of the function
    pub fn transmute<F: Fn(&mut T) -> T>(&mut self, f: F) {
        let iter = self.link_iterator();
        for cell in iter {
            let t: T = f(&mut cell.borrow_mut().elem);
            cell.borrow_mut().elem = t;
        }
    }
    //Convert to another list by applying a function that consumes the values
    pub fn transform<U: Default, F: Fn(T) -> U>(self, f: F) -> LinkedList<U> {
        let mut result = LinkedList::default();
        for t in self {
            result.push_back(f(t));
        }
        result
    }

    //Find all the indices meeting a criteria
    pub fn indices<F: Fn(&T) -> bool>(&self, f: F) -> LinkedList<usize> {
        match self.head {
            None => LinkedList::<usize>::default(),
            Some(_) => self
                .link_iterator()
                .enumerate()
                .filter(|(_, cell)| f(&cell.borrow().elem))
                .map(|(index, _)| index)
                .collect(),
        }
    }

    //Find the last index of a given value
    pub fn last_index_of(&self, value: &T) -> Option<usize>
    where
        T: PartialOrd,
    {
        match self.head {
            None => None,
            Some(_) => self
                .link_iterator()
                .enumerate()
                .filter(|(_, cell)| cell.borrow().elem == *value)
                .map(|(index, _)| index)
                .last(),
        }
    }

    //Find the first index of a value
    #[inline(always)]
    pub fn index_of(&self, value: &T) -> Option<usize>
    where
        T: PartialOrd,
    {
        match self.head {
            None => None,
            _ => self
                .link_iterator()
                .enumerate()
                .find(|(_, cell)| cell.borrow().elem == *value)
                .map(|(index, _)| index),
        }
    }

    //Delete a node at a given index
    //Retturns the deleted value
    //O(n) operation
    pub fn delete_at_index(&mut self, index: usize) -> Option<T> {
        match index {
            idx if idx >= self.len() => None,
            0 => self.pop_front(),
            idx if idx == self.len() - 1 => self.pop_back(),
            _ => {
                let mut prev = self
                    .link_iterator()
                    .enumerate()
                    .skip_while(|(idx, _)| idx != &(index - 1))
                    .take(1)
                    .next()
                    .map(|(_, link)| link);

                let mut elem = prev.as_mut().and_then(|prev| prev.borrow_mut().next.take());
                let next = elem.as_mut().and_then(|elem| elem.borrow_mut().next.take());

                if let Some(prev) = prev {
                    prev.borrow_mut().next = next;
                }
                self.len -= 1;
                elem.map(|elem| elem.borrow_mut().take())
            }
        }
    }

    //Delete the first occurence of a value
    pub fn delete_last(&mut self, value: &T) -> Option<T>
    where
        T: PartialOrd,
    {
        match self.last_index_of(value) {
            None => None,
            Some(index) => self.delete_at_index(index),
        }
    }

    //Delete the first occurence of a value
    pub fn delete_first(&mut self, value: &T) -> Option<T>
    where
        T: PartialOrd,
    {
        match self.index_of(value) {
            None => None,
            Some(index) => self.delete_at_index(index),
        }
    }

    //Reverse the list
    pub fn reverse(&mut self) {
        if self.len < 2 {
            return;
        }
        let mut previous = None;
        let mut current = self.head.take();
        while let Some(ref mut node) = current {
            let next = node.borrow_mut().next.take();
            node.borrow_mut().next = previous;
            previous = current;
            current = next;
        }
        self.head = previous;
    }

    ///
    ///Append another list to this
    ///
    pub fn append(&mut self, other: Self) {
        self.extend(other);
    }
    /// Splits the list into two at the given index.
    ///
    /// Returns a newlist  containing the elements in the range
    /// `[at, len)`. After the call, the original list will be left containing
    /// the elements `[0, at)`.
    /// If this list is empty or index is more than length of this list,
    /// would return an empty list.
    /// If index is 0 this list would become empty
    ///
    pub fn split_off(&mut self, index: usize) -> Self {
        if self.len == 0 || index >= self.len {
            Self::default()
        } else if index == 0 {
            return std::mem::take(self);
        } else {
            let split = self
                .link_iterator()
                .enumerate()
                .find(|(idx, _)| *idx == index - 1)
                .and_then(|(_, cell)| cell.borrow_mut().next.take());
            let split_len = self.len - index;
            self.len = index;
            LinkedList {
                head: split,
                len: split_len,
            }
        }
    }
    ///
    ///Merge this sorted list with another sorted list in ascending or descending order
    ///
    pub fn merge_with(&mut self, other: Self, ascending: bool)
    where
        T: PartialOrd,
    {
        let len = self.len + other.len;
        let this = self.split_off(0);
        *self = Self::merge(this, other, ascending);
        self.len = len;
    }

    ///
    ///Merge two sorted list to one single list in ascending or discending order
    ///
    pub fn merge(mut list1: Self, mut list2: Self, ascending: bool) -> LinkedList<T>
    where
        T: PartialOrd,
    {
        let mut list: LinkedList<Option<T>> = Default::default();
        while list1.front().is_some() && list2.front().is_some() {
            if ascending {
                if list1.front() > list2.front() {
                    list.push_back(list2.pop_front());
                } else {
                    list.push_back(list1.pop_front());
                }
            } else if list1.front() > list2.front() {
                list.push_back(list1.pop_front());
            } else {
                list.push_back(list2.pop_front());
            }
        }
        while list1.front().is_some() {
            list.push_back(list1.pop_front());
        }
        while list2.front().is_some() {
            list.push_back(list2.pop_front());
        }

        Self::flatten(list)
    }

    fn flatten(mut list: LinkedList<Option<T>>) -> LinkedList<T> {
        let mut result: LinkedList<T> = Default::default();
        for _ in 0..list.len {
            if let Some(Some(t)) = list.pop_front() {
                result.push_back(t);
            }
        }
        result
    }

    pub(crate) fn link_iterator(&self) -> LinkIterator<T> {
        LinkIterator {
            links: self.head.as_ref().map(Rc::clone),
        }
    }

    //Is the list sorted in order - ascending or descending?
    pub fn is_sorted(&self, ascending: bool) -> bool
    where
        T: PartialOrd,
    {
        let mut current: Link<T> = None;
        for cell in self.link_iterator() {
            match current {
                None => current = Some(cell),
                Some(prev) => match ascending {
                    true if prev > cell => return false,
                    false if prev < cell => return false,
                    _ => current = Some(cell),
                },
            }
        }
        true
    }

    //Insert values in ascending or descending order. O(n) worst case operation to find the (prev,
    //next) tuple within which to place the value
    pub fn insert_sorted(&mut self, elem: T, ascending: bool)
    where
        T: PartialOrd,
    {
        if self.is_empty() {
            self.push_front(elem);
            return;
        }
        let mut prev = None;
        let insert_at = self
            .link_iterator()
            .map(|link| {
                if prev.is_none() {
                    //First item of the iterator. prev is None - set this item as prev for later
                    //If the first item satifies our query - its of no use
                    prev = Some(Rc::clone(&link));
                    //First value itself satisfy the condition - find returns this with
                    //No previous i.e. value need to be inserted at the beginning
                    if ascending {
                        (None, link.borrow().elem >= elem, Rc::clone(&link))
                    } else {
                        (None, link.borrow().elem <= elem, Rc::clone(&link))
                    }
                } else {
                    //Condition was not satisfied with first item of the iterator or so far.
                    //prev was set before - hence if current item of iterator satisfy our query
                    //that prev is our required prev and current item is our required next to be
                    //returned by "find" method on the iterator
                    //let curr_prev = prev.as_ref().cloned();
                    let existent = prev.as_ref().map(Rc::clone);
                    prev = Some(Rc::clone(&link));
                    if ascending {
                        (existent, link.borrow().elem >= elem, Rc::clone(&link))
                    } else {
                        (existent, link.borrow().elem <= elem, Rc::clone(&link))
                    }
                }
            })
            .find(|(_, gle, _)| gle == &true) //gle => greater/lesser/equal
            .map(|(prev, _, next)| (prev, next));
        match insert_at {
            None => self.push_back(elem),
            //All items are smaller(bigger) than the value to be inserted if ascending(descending).
            //Hence found none. Hence value goes to the end
            Some((None, _)) => self.push_front(elem), //First item itself was bigger or
            //equal(smaller or equal) if ascending(descending). Hence value goes to the front
            Some((mut prev, next)) => {
                //Found prev and next. Stick in between them
                let entry = Some(Node::with_link(elem, next));
                if let Some(ref mut prev) = prev {
                    prev.borrow_mut().next = entry;
                    self.len += 1;
                }
            }
        }
    }

    //Implementation of various sorting alogrithms
    pub fn bubble_sort(&self, ascending: bool)
    where
        T: PartialOrd,
    {
        if self.len() < 2 {
            return;
        }
        let len = self.len() - 1;
        for i in 0..len {
            let mut curr_node = self.head.as_ref().map(Rc::clone);
            let mut swapped = false;
            for _ in 0..(len - i) {
                let in_order = Node::in_order(curr_node.as_ref(), ascending);
                if !in_order {
                    Node::swap_with_next(curr_node.as_ref());
                    swapped = true;
                }
                curr_node =
                    curr_node.and_then(|curr_node| curr_node.borrow().next.as_ref().map(Rc::clone));
            }
            if !swapped {
                break;
            }
        }
    }
    //Sort the list values via selection sort
    pub fn selection_sort(&self, ascending: bool)
    where
        T: PartialOrd,
    {
        if self.len < 2 {
            return;
        }
        self.link_iterator()
            .enumerate()
            .take(self.len() - 1) //elems at [0..n-1]
            .for_each(|(i, curr_node)| {
                let mut min_or_max = Rc::clone(&curr_node);
                self.link_iterator()
                    .enumerate()
                    .skip_while(|(j, _)| j <= &i)
                    .map(|(_, node)| node)
                    .for_each(|node| {
                        if ascending {
                            if min_or_max > node {
                                min_or_max = Rc::clone(&node);
                            }
                        } else if min_or_max < node {
                            min_or_max = Rc::clone(&node);
                        }
                    });
                if !Rc::ptr_eq(&curr_node, &min_or_max) {
                    Node::swap(&mut curr_node.borrow_mut(), &mut min_or_max.borrow_mut());
                }
            });
    }

    //Sort the values using insertion sort
    pub fn insertion_sort(&mut self, ascending: bool)
    where
        T: PartialOrd,
    {
        if self.len < 2 {
            return;
        }
        let mut current = self.head.take();
        self.len = 0;
        while let Some(cell) = current {
            let mut node = cell.take();
            self.insert_sorted(node.take(), ascending);
            current = node.next.take();
        }
    }
    //Does the list contain the elem?
    pub fn contains(&self, elem: &T) -> bool
    where
        T: PartialEq<T>,
    {
        self.link_iterator().any(|e| &e.borrow().elem == elem)
    }

    pub fn front(&self) -> Option<NonMutT<'_, T>> {
        self.head.as_ref().map(|node| NonMutT(node.borrow()))
    }

    pub fn front_mut(&mut self) -> Option<MutT<'_, T>> {
        self.head.as_mut().map(|node| MutT(node.borrow_mut()))
    }
    ///Quick sort - slow. Usage is advisable when list size is small
    pub fn quicksort(&self, ascending: bool)
    where
        T: PartialOrd,
    {
        self.quicklysort(ascending, 0, self.len - 1);
    }

    fn quicklysort(&self, ascending: bool, start: usize, end: usize)
    where
        T: PartialOrd,
    {
        if start < end {
            let pivot_index = self.partition(ascending, start, end);
            if pivot_index > 0 {
                self.quicklysort(ascending, 0, pivot_index - 1);
            }
            self.quicklysort(ascending, pivot_index + 1, end);
        }
    }

    fn partition(&self, ascending: bool, start: usize, end: usize) -> usize
    where
        T: PartialOrd,
    {
        let pivot = self.link_iterator().nth(start);
        let mut next_pivot_pos = start + 1; //possibly
        for k in start + 1..=end {
            let kth = self.link_iterator().nth(k);
            let lesser_or_greater = if ascending { kth < pivot } else { kth > pivot };
            if lesser_or_greater {
                if next_pivot_pos != k {
                    if let Some(at_next_pivot_pos) = self.link_iterator().nth(next_pivot_pos) {
                        if let Some(at_kth_pos) = kth {
                            Node::swap(
                                &mut at_next_pivot_pos.borrow_mut(),
                                &mut at_kth_pos.borrow_mut(),
                            );
                        }
                    }
                }
                next_pivot_pos += 1;
            }
        }

        next_pivot_pos -= 1;
        if next_pivot_pos != start {
            let at_pivot_next_pos = self.link_iterator().nth(next_pivot_pos);
            if let Some(pivot) = pivot {
                if let Some(at_next_pos) = at_pivot_next_pos {
                    let value_at_next_pos = at_next_pos.borrow_mut().take();
                    let pivot_value =
                        std::mem::replace(&mut pivot.borrow_mut().elem, value_at_next_pos);
                    let _ = std::mem::replace(&mut at_next_pos.borrow_mut().elem, pivot_value);
                }
            }
        }
        next_pivot_pos
    }

    fn split_and_merge_sorted(mut list: Self, ascending: bool) -> Self
    where
        T: PartialOrd,
    {
        if list.len > 1 {
            let mid = list.len / 2;
            let split = list.split_off(mid);
            Self::merge(
                Self::split_and_merge_sorted(list, ascending),
                Self::split_and_merge_sorted(split, ascending),
                ascending,
            )
        } else {
            list
        }
    }
    pub fn mergesort(&mut self, ascending: bool)
    where
        T: PartialOrd,
    {
        *self = Self::split_and_merge_sorted(self.split_off(0), ascending);
    }
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        Self {
            elem: T::default(),
            next: None,
        }
    }
}

//Default linked list contains nothing
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self { head: None, len: 0 }
    }
}
impl<T: Default> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::default();
        for t in iter {
            list.push_back(t);
        }
        list
    }
}

impl<T: Default> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}
impl<T: Default> Extend<T> for LinkedList<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.push_back(elem);
        }
    }
}

//An iterator used internally
pub(crate) struct LinkIterator<T> {
    links: Link<T>,
}

impl<T> Iterator for LinkIterator<T> {
    type Item = Cell<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.links.take().map(|link| {
            self.links = link.borrow().next.as_ref().map(Rc::clone);
            link
        })
    }
}

impl<T: Debug> Debug for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.head {
            None => write!(f, "Empty linked list"),
            Some(ref node) => {
                let _ = write!(f, "{{");
                let _ = node.borrow().fmt(f);
                let _ = write!(f, "}}, size: ");
                write!(f, "{}", self.len)
            }
        }
    }
}
impl<T: Default> Add for LinkedList<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut itself = self;
        let mut other = other;
        itself.len += other.len;
        if let Some(last) = itself.link_iterator().last().as_mut() {
            last.borrow_mut().next = other.head.take();
        }
        itself
    }
}

pub struct IntoIter<T: Default>(LinkedList<T>);

impl<T: Default> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn is_sorted<T: Debug>(mut input: impl Iterator<Item = T>, ascending: bool) -> bool
    where
        T: PartialOrd,
    {
        let mut current: Option<T> = None;
        for t in input.by_ref() {
            match current {
                None => current = Some(t),
                Some(prev) => match ascending {
                    true if prev > t => return false,
                    false if prev < t => return false,
                    _ => current = Some(t),
                },
            }
        }
        true
    }

    #[test]
    fn linkedlist_mergesort_test_1() {
        let mut list = LinkedList::<i32>::from_slice(&[1, 2, 3, 4, 5, 6]);
        list.mergesort(false);
        println!("The quick sorted list: {:?}", list);
        assert_eq!(list, LinkedList::<i32>::from_slice(&[6, 5, 4, 3, 2, 1]));

        let mut runs = 50;
        loop {
            let mut elems: [u16; 64] = [0; 64];
            rand::thread_rng().fill(&mut elems);
            let mut list = LinkedList::<u16>::from_slice(&elems);

            list.mergesort(false);
            assert!(list.is_sorted(false)); //false for descending

            let sorted = is_sorted(list.into_iter(), false);
            assert!(sorted);

            let mut elems: [u8; 128] = [0; 128];
            rand::thread_rng().fill(&mut elems);
            let mut list = LinkedList::<i32>::from_slice(&elems);

            list.mergesort(true);
            assert!(list.is_sorted(true));

            let sorted = is_sorted(list.into_iter(), true);
            assert!(sorted);

            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }

    #[test]
    fn linkedlist_merge_with_test_1() {
        let list = LinkedList::<i32>::default();
        let mut list1 = LinkedList::<i32>::default();
        list1.merge_with(list, true);
        assert_eq!(list1, LinkedList::<i32>::default());

        let list = LinkedList::<i32>::from_slice(&[1, 3, 5, 6]);
        let mut list1 = LinkedList::<i32>::default();
        list1.merge_with(list, true);
        assert_eq!(list1, LinkedList::<i32>::from_slice(&[1, 3, 5, 6]));

        let mut list = LinkedList::<i32>::from_slice(&[1, 3, 5, 6]);
        let list1 = LinkedList::<i32>::default();
        list.merge_with(list1, true);
        assert_eq!(list, LinkedList::<i32>::from_slice(&[1, 3, 5, 6]));

        let mut list = LinkedList::<i32>::from_slice(&[1, 3, 5, 6]);
        let list1 = LinkedList::<i32>::from_slice(&[2, 4, 5, 6]);
        list.merge_with(list1, true);
        assert_eq!(
            list,
            LinkedList::<i32>::from_slice(&[1, 2, 3, 4, 5, 5, 6, 6])
        );
        let mut runs = 50;
        loop {
            let mut elems: [u16; 16] = [0; 16];
            rand::thread_rng().fill(&mut elems);
            let mut list1 = LinkedList::<u16>::from_slice(&elems);

            list1.quicksort(false);
            assert!(list1.is_sorted(false)); //false for descending

            let mut elems: [u16; 16] = [0; 16];
            rand::thread_rng().fill(&mut elems);
            let list2 = LinkedList::<u16>::from_slice(&elems);

            list2.quicksort(false);
            assert!(list2.is_sorted(false));
            list1.merge_with(list2, false);
            assert!(list1.is_sorted(false));

            /////////////////////////
            let mut elems: [u16; 16] = [0; 16];
            rand::thread_rng().fill(&mut elems);
            let list1 = LinkedList::<u16>::from_slice(&elems);

            list1.quicksort(true);
            assert!(list1.is_sorted(true)); //false for descending

            let mut elems: [u16; 16] = [0; 16];
            rand::thread_rng().fill(&mut elems);
            let mut list2 = LinkedList::<u16>::from_slice(&elems);

            list2.quicksort(true);
            assert!(list2.is_sorted(true));
            list2.merge_with(list1, true);
            assert!(list2.is_sorted(true));

            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }

    #[test]
    fn linkedlist_merge_test_1() {
        let list1 = LinkedList::<i32>::default();
        let list2 = LinkedList::<i32>::default();
        let list = LinkedList::merge(list1, list2, true);
        assert_eq!(list, LinkedList::<i32>::default());

        let list1 = LinkedList::<i32>::from_slice(&[1, 3, 5, 6]);
        let list2 = LinkedList::<i32>::from_slice(&[2, 4, 5, 6]);
        let list = LinkedList::merge(list1, list2, true);
        assert_eq!(
            list,
            LinkedList::<i32>::from_slice(&[1, 2, 3, 4, 5, 5, 6, 6])
        );
        let mut runs = 50;
        loop {
            let mut elems: [u16; 16] = [0; 16];
            rand::thread_rng().fill(&mut elems);
            let list1 = LinkedList::<u16>::from_slice(&elems);

            list1.quicksort(false);
            assert!(list1.is_sorted(false)); //false for descending

            let mut elems: [u16; 16] = [0; 16];
            rand::thread_rng().fill(&mut elems);
            let list2 = LinkedList::<u16>::from_slice(&elems);

            list2.quicksort(false);
            assert!(list2.is_sorted(false));
            let list = LinkedList::merge(list1, list2, false);
            assert!(list.is_sorted(false));

            /////////////////////////
            let mut elems: [u16; 16] = [0; 16];
            rand::thread_rng().fill(&mut elems);
            let list1 = LinkedList::<u16>::from_slice(&elems);

            list1.quicksort(true);
            assert!(list1.is_sorted(true)); //false for descending

            let mut elems: [u16; 16] = [0; 16];
            rand::thread_rng().fill(&mut elems);
            let list2 = LinkedList::<u16>::from_slice(&elems);

            list2.quicksort(true);
            assert!(list2.is_sorted(true));
            let list = LinkedList::merge(list1, list2, true);
            assert!(list.is_sorted(true));

            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }

    #[test]
    fn linkedlist_quicksort_test_1() {
        let list = LinkedList::<i32>::from_slice(&[1, 2, 3, 4, 5, 6]);
        list.quicksort(false);
        println!("The quick sorted list: {:?}", list);
        assert_eq!(list, LinkedList::<i32>::from_slice(&[6, 5, 4, 3, 2, 1]));

        let mut runs = 50;
        loop {
            let mut elems: [u16; 16] = [0; 16];
            rand::thread_rng().fill(&mut elems);
            let list = LinkedList::<u16>::from_slice(&elems);

            list.quicksort(false);
            assert!(list.is_sorted(false)); //false for descending

            let sorted = is_sorted(list.into_iter(), false);
            assert!(sorted);

            let mut elems: [u8; 16] = [0; 16];
            rand::thread_rng().fill(&mut elems);
            let list = LinkedList::<i32>::from_slice(&elems);

            list.quicksort(true);
            assert!(list.is_sorted(true));

            let sorted = is_sorted(list.into_iter(), true);
            assert!(sorted);

            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }

    #[test]
    fn linkedlist_add_test_1() {
        let list1 = LinkedList::<i32>::from_slice(&[1, 2, 3]);
        let list2 = LinkedList::<i32>::from_slice(&[4, 5, 6]);
        let mut list = list1 + list2;
        assert_eq!(list, LinkedList::<i32>::from_slice(&[1, 2, 3, 4, 5, 6]));
        assert_eq!(list.len(), 6);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(6));
        assert_eq!(list.len(), 4);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn linkedlist_split_off_test_1() {
        let mut list = LinkedList::<i32>::from_slice(&[1, 2, 3]);
        let mut split = list.split_off(1);
        assert_eq!(split, LinkedList::<i32>::from_slice(&[2, 3]));
        assert_eq!(list, LinkedList::<i32>::from_slice(&[1]));
        assert_eq!(list.len(), 1);
        split.push_back(4);
        assert_eq!(split, LinkedList::<i32>::from_slice(&[2, 3, 4]));
        assert_eq!(split.len(), 3);
    }
    #[test]
    fn linkedlist_append_test_1() {
        let source = LinkedList::<i32>::from_slice(&[1, 2, 3]);
        let mut target = LinkedList::default();
        target.append(source);
        assert_eq!(target, LinkedList::<i32>::from_slice(&[1, 2, 3]));
    }
    #[test]
    fn linkedlist_extend_test_1() {
        let source = vec![1, 2, 3];
        let mut target = LinkedList::default();
        target.extend(source);
        assert_eq!(target, LinkedList::<i32>::from_slice(&[1, 2, 3]));
    }

    #[test]
    fn linkedlist_insertion_sort_test_1() {
        let mut list = LinkedList::<i32>::from_slice(&[30, 10, 5, 20, 15, 45, 35, 25, 50, 40]);
        list.insertion_sort(true); //true for ascending
        assert_eq!(
            list,
            LinkedList::<i32>::from_slice(&[5, 10, 15, 20, 25, 30, 35, 40, 45, 50])
        );
        list.insertion_sort(false);
        let mut expected = LinkedList::<i32>::from_slice(&[5, 10, 15, 20, 25, 30, 35, 40, 45, 50]);
        expected.reverse();
        assert_eq!(list, expected);

        let mut runs = 50;

        loop {
            let mut elems: [u16; 256] = [0; 256];
            rand::thread_rng().fill(&mut elems);
            let mut list = LinkedList::<u16>::from_slice(&elems);

            list.insertion_sort(false);
            assert!(list.is_sorted(false));

            let sorted = is_sorted(list.into_iter(), false);
            assert!(sorted);

            let mut elems: [i32; 256] = [0; 256];
            rand::thread_rng().fill(&mut elems);
            let mut list = LinkedList::<i32>::from_slice(&elems);

            list.insertion_sort(true);
            assert!(list.is_sorted(true));

            println!("{:?}", list);

            let sorted = is_sorted(list.into_iter(), true);
            assert!(sorted);

            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }

    #[test]
    fn linkedlist_selection_sort_test_1() {
        let list = LinkedList::<i32>::from_slice(&[30, 10, 5, 20, 15, 45, 35, 25, 50, 40]);
        list.selection_sort(true); //true for ascending
        assert_eq!(
            list,
            LinkedList::<i32>::from_slice(&[5, 10, 15, 20, 25, 30, 35, 40, 45, 50])
        );
        list.selection_sort(false);
        let mut expected = LinkedList::<i32>::from_slice(&[5, 10, 15, 20, 25, 30, 35, 40, 45, 50]);
        expected.reverse();
        assert_eq!(list, expected);

        let mut runs = 50;

        loop {
            let mut elems: [u16; 128] = [0; 128];
            rand::thread_rng().fill(&mut elems);
            let list = LinkedList::<u16>::from_slice(&elems);

            list.selection_sort(false);
            assert!(list.is_sorted(false));

            let sorted = is_sorted(list.into_iter(), false);
            assert!(sorted);

            let mut elems: [i32; 128] = [0; 128];
            rand::thread_rng().fill(&mut elems);
            let list = LinkedList::<i32>::from_slice(&elems);

            list.selection_sort(true);
            assert!(list.is_sorted(true));

            println!("{:?}", list);

            let sorted = is_sorted(list.into_iter(), true);
            assert!(sorted);

            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }

    #[test]
    fn linkedlist_front_mut_test_1() {
        let mut list = LinkedList::new(30);
        if let Some(mut t) = list.front_mut() {
            *t.t() *= 3;
        }
        assert_eq!(list, LinkedList::<i32>::from_slice(&[90]));

        let mut list = LinkedList::new(30);
        let opt: Option<MutT<'_, i32>> = list.front_mut();
        *opt.unwrap() *= 3;
        assert_eq!(list, LinkedList::<i32>::from_slice(&[90]));
    }

    #[test]
    fn linkedlist_front_test_1() {
        let mut list = LinkedList::<i32>::default();
        list.insert_sorted(30, true);
        let t: Option<NonMutT<'_, i32>> = list.front();
        assert!(t.as_ref().is_some_and(|t| t.t() == &30));

        let mut list = LinkedList::<i32>::default();
        list.push_back(30);
        let front = list.front().unwrap();
        assert_eq!(front.t(), &30);

        let mut list = LinkedList::<i32>::default();
        list.push_back(30);
        let non_mut_t: NonMutT<'_, i32> = list.front().unwrap();
        assert_eq!(*non_mut_t, 30);
        assert_eq!(non_mut_t.deref(), &30);

        let mut list1 = LinkedList::<i32>::default();
        list1.push_back(100);
        let front1 = list1.front();

        let mut list2 = LinkedList::<i32>::default();
        list2.push_back(100);
        assert!(front1 == list2.front());

        list2.push_front(1000);
        let front2 = list2.front();
        assert!(!(front1 == front2));
    }

    #[test]
    fn linkedlist_contains_test_1() {
        let mut list = LinkedList::<i32>::default();
        list.insert_sorted(30, true);
        assert!(list.contains(&30));
        assert!(!list.contains(&40));
        assert_eq!(list.delete_last(&30), Some(30));
        assert_eq!(list.len(), 0);
        assert!(!list.contains(&30));
    }

    #[test]
    fn linkedlist_insert_sorted_test_1() {
        let mut list = LinkedList::<i32>::default();
        list.insert_sorted(10, true);
        list.insert_sorted(30, true);
        list.insert_sorted(45, true);
        list.insert_sorted(5, true);
        list.insert_sorted(50, true);
        list.insert_sorted(20, true);
        list.insert_sorted(25, true);
        list.insert_sorted(40, true);
        list.insert_sorted(15, true);
        list.insert_sorted(35, true);
        println!("Insert sorted list:{:?}", list);

        assert_eq!(
            list,
            LinkedList::<i32>::from_slice(&[5, 10, 15, 20, 25, 30, 35, 40, 45, 50])
        );

        let mut list = LinkedList::<i32>::default();
        list.insert_sorted(10, false);
        list.insert_sorted(30, false);
        list.insert_sorted(45, false);
        list.insert_sorted(5, false);
        list.insert_sorted(50, false);
        list.insert_sorted(20, false);
        list.insert_sorted(25, false);
        list.insert_sorted(40, false);
        list.insert_sorted(15, false);
        list.insert_sorted(35, false);
        println!("Insert sorted list:{:?}", list);

        assert_eq!(
            list,
            LinkedList::<i32>::from_slice(&[50, 45, 40, 35, 30, 25, 20, 15, 10, 5])
        );
    }

    #[test]
    fn linkedlist_last_index_of_test_1() {
        let elems: [i32; 0] = [];
        let list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.last_index_of(&0), None);

        let elems = [500, 400, 300, 200, 100];
        let list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.last_index_of(&500), Some(0));
        assert_eq!(list.last_index_of(&400), Some(1));
        assert_eq!(list.last_index_of(&300), Some(2));
        assert_eq!(list.last_index_of(&200), Some(3));
        assert_eq!(list.last_index_of(&100), Some(4));
        assert_eq!(list.last_index_of(&1000), None);

        let elems = [500, 400, 300, 200, 100, 500, 400, 300, 200, 100];
        let list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.last_index_of(&500), Some(5));
        assert_eq!(list.last_index_of(&400), Some(6));
        assert_eq!(list.last_index_of(&300), Some(7));
        assert_eq!(list.last_index_of(&200), Some(8));
        assert_eq!(list.last_index_of(&100), Some(9));
        assert_eq!(list.last_index_of(&1000), None);
    }

    #[test]
    fn linkedlist_translate_test_1() {
        let elems = [500, 400, 300, 200, 100];
        let list = LinkedList::<i32>::from_slice(&elems);
        let elems = [250, 200, 150, 100, 50];
        let expect = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.translate(|i| *i / 2), expect);
    }

    #[test]
    fn linkedlist_transform_test_1() {
        let elems = [500, 400, 300, 200, 100];
        let list = LinkedList::<i32>::from_slice(&elems);
        let elems = [250, 200, 150, 100, 50];
        let expect = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.translate(|i| i / 2), expect);
    }

    #[test]
    fn linkedlist_transmute_test_1() {
        let elems = [500, 400, 300, 200, 100];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        list.transmute(|i| *i / 2);
        let elems = [250, 200, 150, 100, 50];
        let expect = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list, expect);
    }

    #[test]
    fn linkedlist_iindices_test_1() {
        let elems: [i32; 0] = [];
        let list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.indices(|i| *i == 0), LinkedList::default());

        let elems = [500, 400, 300, 200, 100];
        let list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.indices(|i| *i == 0), LinkedList::default());
        assert_eq!(
            list.indices(|i| *i == 500),
            LinkedList::<usize>::from_slice(&[0])
        );
        assert_eq!(
            list.indices(|i| *i % 100 == 0),
            LinkedList::<usize>::from_slice(&[0, 1, 2, 3, 4])
        );
        assert_eq!(
            list.indices(|i| *i * 2 == 400),
            LinkedList::<usize>::from_slice(&[3])
        );
    }

    #[test]
    fn linkedlist_index_of_test_1() {
        let elems: [i32; 0] = [];
        let list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.index_of(&0), None);

        let elems = [500, 400, 300, 200, 100];
        let list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.index_of(&500), Some(0));
        assert_eq!(list.index_of(&400), Some(1));
        assert_eq!(list.index_of(&300), Some(2));
        assert_eq!(list.index_of(&200), Some(3));
        assert_eq!(list.index_of(&100), Some(4));
        assert_eq!(list.index_of(&1000), None);
    }
    #[test]
    fn linkedlist_delete_last_test_1() {
        let elems: [i32; 0] = [];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_first(&0), None);
        assert_eq!(list.len(), 0);

        let elems = [200];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_last(&200), Some(200));
        assert_eq!(list.len(), 0);

        let elems = [100, 200];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_last(&100), Some(100));
        assert_eq!(list.len(), 1);

        assert_eq!(list.delete_last(&200), Some(200));
        assert_eq!(list.len(), 0);

        let elems = [100, 200];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_last(&200), Some(200));
        assert_eq!(list.len(), 1);

        let elems = [500, 400, 300];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_last(&400), Some(400));
        assert_eq!(list.len(), 2);

        let elems = [500, 400, 300, 200, 100];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_last(&300), Some(300));
        assert_eq!(list.len(), 4);

        let elems = [500, 400, 300, 200, 200, 200, 100];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_last(&600), None);
        assert_eq!(list.last_index_of(&200), Some(5));

        assert_eq!(list.delete_last(&200), Some(200));
        assert_eq!(list.last_index_of(&200), Some(4));

        assert_eq!(list.delete_last(&200), Some(200));
        assert_eq!(list.last_index_of(&200), Some(3));

        assert_eq!(list.delete_last(&200), Some(200));
        assert_eq!(list.last_index_of(&200), None);
    }

    #[test]
    fn linkedlist_delete_first_test_1() {
        let elems: [i32; 0] = [];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_first(&0), None);
        assert_eq!(list.len(), 0);

        let elems = [200];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_first(&200), Some(200));
        assert_eq!(list.len(), 0);

        let elems = [100, 200];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_first(&100), Some(100));
        assert_eq!(list.len(), 1);

        assert_eq!(list.delete_first(&200), Some(200));
        assert_eq!(list.len(), 0);

        let elems = [100, 200];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_first(&200), Some(200));
        assert_eq!(list.len(), 1);

        let elems = [500, 400, 300];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_first(&400), Some(400));
        assert_eq!(list.len(), 2);

        let elems = [500, 400, 300, 200, 100];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_first(&300), Some(300));
        assert_eq!(list.len(), 4);

        let elems = [500, 400, 300, 200, 100];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_first(&600), None);
        assert_eq!(list.len(), 5);

        assert_eq!(list.delete_first(&200), Some(200));
        assert_eq!(list.delete_first(&200), None);
        assert_eq!(list.len(), 4);
        assert_eq!(list.delete_first(&500), Some(500));
        assert_eq!(list.delete_first(&300), Some(300));
        assert_eq!(list.len(), 2);
        assert_eq!(list.delete_first(&400), Some(400));
        assert_eq!(list.delete_first(&100), Some(100));
        assert_eq!(list.delete_first(&100), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn linkedlist_delete_at_index_test_1() {
        let elems: [i32; 0] = [];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_at_index(0), None);
        assert_eq!(list.len(), 0);

        let elems = [200];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_at_index(0), Some(200));
        assert_eq!(list.len(), 0);

        let elems = [100, 200];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_at_index(0), Some(100));
        assert_eq!(list.len(), 1);

        assert_eq!(list.delete_at_index(0), Some(200));
        assert_eq!(list.len(), 0);

        let elems = [100, 200];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_at_index(1), Some(200));
        assert_eq!(list.len(), 1);

        let elems = [500, 400, 300];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_at_index(1), Some(400));
        assert_eq!(list.len(), 2);

        let elems = [500, 400, 300, 200, 100];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_at_index(2), Some(300));
        assert_eq!(list.len(), 4);

        let elems = [500, 400, 300, 200, 100];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list.delete_at_index(5), None);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn linkedlist_bubble_sort_test_1() {
        let elems = [200, 500, 300, 400, 100];
        let list = LinkedList::<i32>::from_slice(&elems);
        list.bubble_sort(false); //false for descending

        let elems = [500, 400, 300, 200, 100];
        let reversed = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list, reversed);

        let elems = [200, 500, 300, 400, 100];
        let list = LinkedList::<i32>::from_slice(&elems);
        list.bubble_sort(true); //true for ascending

        let elems = [100, 200, 300, 400, 500];
        let reversed = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list, reversed);

        let mut runs = 50;

        loop {
            let mut elems: [u16; 128] = [0; 128];
            rand::thread_rng().fill(&mut elems);
            let list = LinkedList::<u16>::from_slice(&elems);

            list.bubble_sort(false);
            assert!(list.is_sorted(false));

            let sorted = is_sorted(list.into_iter(), false);
            assert!(sorted);

            let mut elems: [i32; 128] = [0; 128];
            rand::thread_rng().fill(&mut elems);
            let list = LinkedList::<i32>::from_slice(&elems);

            list.bubble_sort(true);
            assert!(list.is_sorted(true));

            let sorted = is_sorted(list.into_iter(), true);
            assert!(sorted);

            runs -= 1;
            if runs == 0 {
                break;
            }
        }
    }

    #[test]
    fn linkedlist_link_iterator_test_1() {
        let elems = (1..5).collect::<Vec<_>>();
        let list = LinkedList::<i32>::from_slice(&elems);
        let itr = list.link_iterator();
        let mut elem = 1;
        for link in itr {
            assert_eq!(link.borrow_mut().take(), elem);
            elem += 1;
        }
    }

    #[test]
    fn linkedlist_size_test_1() {
        //let elems = (1..21750).collect::<Vec<_>>();
        let elems = (1..4000).collect::<Vec<_>>();
        let mut list = LinkedList::<i32>::from_slice(&elems);
        list.reverse();
        //let elems = (1..21750).rev().collect::<Vec<_>>();
        let elems = (1..4000).rev().collect::<Vec<_>>();
        let reversed = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list, reversed);
    }

    #[test]
    fn linkedlist_reverse_test_1() {
        let elems = [100, 200, 300, 400, 500];
        let mut list = LinkedList::<i32>::from_slice(&elems);
        list.reverse();
        let elems = [500, 400, 300, 200, 100];
        let reversed = LinkedList::<i32>::from_slice(&elems);
        assert_eq!(list, reversed);
    }

    #[test]
    fn list_push_front_test_1() {
        let mut list = LinkedList::default();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn linkedlist_pop_back_test_1() {
        //let elems = (1..21750).collect::<Vec<_>>();
        let elems = (1..4000).collect::<Vec<_>>();
        let mut list = LinkedList::<i32>::from_slice(&elems);
        //for num in (1..21750).rev() {
        for num in (1..4000).rev() {
            assert_eq!(list.pop_back(), Some(num as i32));
        }
        assert_eq!(list.pop_back(), None);
    }
}

///
///Implentation of a singly linked list with cossuming, ref and mutable iterator
///
pub mod iterable {
    use std::fmt::{Debug, Error, Formatter};
    use std::rc::Rc;

    type Link<T> = Option<Rc<Node<T>>>;
    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    pub struct LinkedList<T> {
        head: Link<T>,
        len: usize,
    }

    impl<T: Default> Node<T> {
        pub fn new(elem: T) -> Self {
            Self { elem, next: None }
        }

        fn with_link(elem: T, link: Rc<Node<T>>) -> Rc<Node<T>> {
            Rc::new(Self {
                elem,
                next: Some(link),
            })
        }

        pub fn push_back(&mut self, elem: T) {
            match self.next {
                None => self.next = Some(Rc::new(Self::new(elem))),
                Some(ref mut next) => {
                    if let Some(node) = Rc::get_mut(next) {
                        node.push_back(elem);
                    }
                }
            }
        }
        //Would pop until this node
        pub fn pop_back(&mut self) -> Option<T> {
            match self.next {
                None => None,
                Some(ref mut next) => {
                    let next_is_none = next.next.is_none();
                    if let Some(node) = Rc::get_mut(next) {
                        if next_is_none {
                            let result = Some(node.take());
                            let _ = self.next.take();
                            return result;
                        } else {
                            return node.pop_back();
                        }
                    }
                    None
                }
            }
        }

        fn take(&mut self) -> T {
            std::mem::take(&mut self.elem)
        }
    }

    impl<T: Default> Default for Node<T> {
        fn default() -> Self {
            Self {
                elem: T::default(),
                next: None,
            }
        }
    }

    impl<T: PartialEq> Eq for Node<T> {}
    impl<T: PartialEq> PartialEq for Node<T> {
        fn eq(&self, other: &Self) -> bool {
            match (&self.elem, &self.next, &other.elem, &other.next) {
                (elem, _, other_elem, _) if *elem != *other_elem => false,
                (elem, None, other_elem, None) if *elem == *other_elem => true,
                (_, None, _, Some(_)) => false,
                (_, Some(_), _, None) => false,
                (elem, Some(this), other_elem, Some(that)) if *elem == *other_elem => this.eq(that),
                (_, _, _, _) => false,
            }
        }
    }

    impl<T: Debug> Debug for Node<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            match self.next {
                None => write!(f, "{:?}", self.elem),
                Some(ref next) => {
                    let _ = write!(f, "{:?} -> ", self.elem);
                    next.fmt(f)
                }
            }
        }
    }

    impl<T: Default> LinkedList<T> {
        pub fn new(elem: T) -> Self {
            Self {
                head: Some(Rc::new(Node::new(elem))),
                len: 1,
            }
        }

        //Create a list from from clonable types
        pub fn from_slice<U: Clone + Default>(elems: &[U]) -> LinkedList<U> {
            assert!(!elems.is_empty());
            let mut node = Node::<U>::new(elems[0].clone());
            elems[1..]
                .iter()
                .for_each(|elem| node.push_back(elem.clone()));

            LinkedList {
                head: Some(Rc::new(node)),
                len: elems.len(),
            }
        }

        pub fn front(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.elem)
        }

        pub fn push_front(&mut self, elem: T) {
            match self.head.take() {
                Some(as_link) => self.head = Some(Node::with_link(elem, as_link)),
                None => self.head = Some(Rc::new(Node::new(elem))),
            }
            self.len += 1;
        }

        pub fn push_back(&mut self, elem: T) {
            match self.head.as_mut().and_then(Rc::get_mut) {
                None => self.head = Some(Rc::new(Node::new(elem))),
                Some(node) => node.push_back(elem),
            }
            self.len += 1;
        }

        pub fn pop_front(&mut self) -> Option<T> {
            match self.head.take() {
                Some(taken) => match Rc::into_inner(taken) {
                    None => None,
                    Some(mut node) => {
                        self.head = node.next.take();
                        self.len -= 1;
                        Some(node.take())
                    }
                },
                None => None,
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            self.head.as_ref()?;
            if let Some(head) = self.head.as_mut() {
                if let Some(node) = Rc::get_mut(head) {
                    if self.len == 1 {
                        let result = Some(node.take());
                        self.len -= 1;
                        let _ = self.head.take();
                        return result;
                    } else {
                        let result = node.pop_back();
                        if result.is_some() {
                            self.len -= 1;
                        }
                        return result;
                    }
                }
            }
            None
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        //Update a value at the given index
        pub fn update(&mut self, index: usize, elem: T) -> Option<T> {
            if index >= self.len {
                return None;
            }
            self.iter_mut()
                .enumerate()
                .find(|(idx, _)| idx == &index)
                .map(|(_, t)| t)
                .map(|t| std::mem::replace(t, elem))
        }

        pub fn reverse(&mut self) {
            if self.len < 2 {
                return;
            }
            let mut previous = None;
            let mut current = self.head.take();
            while let Some(ref mut curr_inner) = current.as_mut().and_then(Rc::get_mut) {
                let next = curr_inner.next.take();
                curr_inner.next = previous.take();
                previous = current.take();
                current = next;
            }
            self.head = previous;
        }

        pub fn iter(&self) -> Iter<'_, T> {
            Iter {
                link: self.head.as_ref().map(|rc_node| rc_node.as_ref()),
            }
        }

        ///Returns a mut iterator for the elements of the list. Mutating the
        ///elems would change the backing list
        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            IterMut {
                link: self.head.as_mut().and_then(Rc::get_mut),
            }
        }
    }

    impl<T> Drop for LinkedList<T> {
        fn drop(&mut self) {
            let mut head = self.head.take();
            while let Some(node) = head.as_mut().and_then(Rc::get_mut) {
                head = node.next.take();
            }
        }
    }

    impl<T> Default for LinkedList<T> {
        fn default() -> Self {
            Self { head: None, len: 0 }
        }
    }

    impl<T: Debug> Debug for LinkedList<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            match self.head {
                None => write!(f, "Empty linked list"),
                Some(ref node) => {
                    let _ = write!(f, "{{");
                    let _ = node.fmt(f);
                    let _ = write!(f, "}}, size: ");
                    write!(f, "{}", self.len)
                }
            }
        }
    }

    impl<T: PartialEq> Eq for LinkedList<T> {}
    impl<T: PartialEq> PartialEq for LinkedList<T> {
        fn eq(&self, other: &Self) -> bool {
            self.len == other.len && self.head == other.head
        }
    }

    pub struct IntoIter<T> {
        link: Option<Node<T>>,
    }

    impl<T> Iterator for IntoIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<Self::Item> {
            self.link.take().map(|node| {
                self.link = node.next.and_then(Rc::into_inner);
                node.elem
            })
        }
    }

    impl<T> IntoIterator for LinkedList<T> {
        type Item = T;
        type IntoIter = IntoIter<Self::Item>;

        fn into_iter(mut self) -> Self::IntoIter {
            //let mut head = self.head.take();
            IntoIter {
                link: self.head.take().and_then(Rc::into_inner),
            }
        }
    }

    pub struct Iter<'a, T> {
        link: Option<&'a Node<T>>,
    }

    impl<'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            self.link.map(|node| {
                self.link = node.next.as_ref().map(|next| next.as_ref()); //next = &Rc<Node<T>>
                                                                          //self.node = node.next.as_ref().map(|next| next.deref());
                                                                          //self.node = node.next.as_deref();
                &node.elem
            })
        }
    }

    pub struct IterMut<'a, T> {
        link: Option<&'a mut Node<T>>,
    }

    impl<'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;
        fn next(&mut self) -> Option<Self::Item> {
            self.link.take().map(|node| {
                self.link = node.next.as_mut().and_then(|next| Rc::get_mut(next)); //next = &mut Rc<Node<T>>
                &mut node.elem
            })
        }
    }
    #[cfg(test)]
    mod iterable_tests {
        use super::*;
        #[test]
        fn linklist_iter_test() {
            let elems = [1, 2, 3, 4, 5];
            let list = LinkedList::<i32>::from_slice(&elems);
            let mut iter = list.iter();
            for num in 1..=5 {
                assert_eq!(iter.next(), Some(&num as &i32));
            }
            assert_eq!(iter.next(), None);
        }
        #[test]
        fn linklist_iter_mut_test() {
            let elems = [1, 2, 3, 4, 5];
            let mut list = LinkedList::<i32>::from_slice(&elems);
            let mut iter = list.iter_mut();
            for _ in 0..5 {
                if let Some(elem) = iter.next() {
                    *elem *= 100;
                }
            }
            let elems = [100, 200, 300, 400, 500];
            let expected = LinkedList::<i32>::from_slice(&elems);
            assert_eq!(list, expected);
        }

        #[test]
        fn linklist_into_iter_test() {
            let elems = [1, 2, 3, 4, 5];
            let list = LinkedList::<i32>::from_slice(&elems);
            let mut iter = list.into_iter();
            for num in 1..=5 {
                assert_eq!(iter.next(), Some(num));
            }
            assert_eq!(iter.next(), None);

            let elems = [1, 2, 3, 4, 5];
            let list = LinkedList::<i32>::from_slice(&elems);

            let mut num = 1;
            for i in list {
                assert_eq!(i, num);
                num += 1;
            }
        }
        #[test]
        fn linkedlist_push_back_test() {
            let mut list = LinkedList::<i32>::default();
            list.push_back(1);
            list.push_back(2);
            list.push_back(3);
            let mut iter = list.into_iter();
            for num in 1..=3 {
                assert_eq!(iter.next(), Some(num));
            }
            assert_eq!(iter.next(), None);

            let mut list = LinkedList::<i32>::new(1);
            list.push_back(2);
            list.push_back(3);
            let mut iter = list.into_iter();
            for num in 1..=3 {
                assert_eq!(iter.next(), Some(num));
            }
            assert_eq!(iter.next(), None);
        }

        #[test]
        fn node_pop_back_test_2() {
            let mut node = Node::new(1);
            node.push_back(2);
            node.push_back(3);
            node.push_back(4);
            assert_eq!(node.pop_back(), Some(4));
            assert_eq!(node.pop_back(), Some(3));
            assert_eq!(node.pop_back(), Some(2));
            assert_eq!(node.pop_back(), None);
        }

        #[test]
        fn linkedlist_pop_back_test_2() {
            let mut list = LinkedList::new(1);
            list.push_back(2);
            list.push_back(3);
            list.push_back(4);
            assert_eq!(list.pop_back(), Some(4));
            assert_eq!(list.pop_back(), Some(3));
            assert_eq!(list.pop_back(), Some(2));
            assert_eq!(list.pop_back(), Some(1));
            assert_eq!(list.pop_back(), None);
        }

        #[test]
        fn linkedlist_reverse_test_2() {
            let elems = [100, 200, 300, 400, 500];
            let mut list = LinkedList::<i32>::from_slice(&elems);
            list.reverse();
            let elems = [500, 400, 300, 200, 100];
            let reversed = LinkedList::<i32>::from_slice(&elems);
            assert_eq!(list, reversed);
        }

        #[test]
        fn linkedlist_update_test() {
            let elems = [100, 200, 300, 400, 500];
            let mut list = LinkedList::<i32>::from_slice(&elems);
            let result = list.update(4, 1000);
            assert_eq!(result, Some(500));

            let result = list.update(0, 999);
            assert_eq!(result, Some(100));
        }
    }
}
