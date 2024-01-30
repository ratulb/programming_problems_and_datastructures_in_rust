use std::alloc::{self, Layout};
use std::fmt;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

struct RawEntryIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RawEntryIter<T> {
    fn new(slice: &[T]) -> Self {
        Self {
            start: slice.as_ptr(),
            end: if mem::size_of::<T>() == 0 {
                ((slice.as_ptr() as usize) + slice.len()) as *const T
            } else if slice.len() == 0 {
                slice.as_ptr()
            } else {
                unsafe { slice.as_ptr().add(slice.len()) }
            },
        }
    }
}

impl<T> Iterator for RawEntryIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.start = (self.start as usize + 1) as *const T;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    let old_ptr = self.start;
                    self.start = self.start.offset(1);
                    Some(ptr::read(old_ptr))
                }
            }
        }
    }
}

impl<T> DoubleEndedIterator for RawEntryIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if mem::size_of::<T>() == 0 {
                    self.end = (self.end as usize - 1) as *const T;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    self.end = self.end.offset(-1);
                    Some(ptr::read(self.end))
                }
            }
        }
    }
}
struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T> {
    fn new() -> Self {
        let cap = if mem::size_of::<T>() == 0 {
            usize::MAX
        } else {
            0
        };
        Self {
            ptr: NonNull::dangling(),
            cap,
        }
    }

    fn with_capacity(cap: usize) -> Self {
        assert!(mem::size_of::<T>() != 0, "TODO: Implement  ZST support");
        if cap == 0 {
            return Self::new();
        }
        let layout = match Layout::array::<T>(cap) {
            Ok(layout) => layout,
            Err(err) => panic!("Allocation too large! {}", err),
        };
        let ptr = unsafe { alloc::alloc(layout) };
        let ptr = match NonNull::new(ptr as *mut T) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(layout),
        };
        Self { ptr, cap }
    }

    fn grow(&mut self) {
        assert!(mem::size_of::<T>() != 0, "capacity overflow");
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let new_layout = match Layout::array::<T>(new_cap) {
            Ok(layout) => layout,
            Err(err) => {
                //Free up if allocated before
                mem::take(self);
                panic!("Allocation too large! {}", err);
            }
        };

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }

    fn shrink(&mut self, new_cap: usize) {
        //Shrink only when current cap >= 2 * new_cap
        if new_cap == 0 || 2 * new_cap > self.cap {
            return;
        }
        let new_layout = match Layout::array::<T>(new_cap) {
            Ok(layout) => layout,
            Err(err) => {
                //Free up if allocated before
                mem::take(self);
                panic!("Allocation too large! {}", err);
            }
        };

        let old_layout = Layout::array::<T>(self.cap).unwrap();
        let old_ptr = self.ptr.as_ptr() as *mut u8;
        let new_ptr = unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let size_of_t = mem::size_of::<T>();
        if self.cap != 0 && size_of_t != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) }
            self.cap = 0;
        }
    }
}
impl<T> Default for RawVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
pub struct IntoIter<T> {
    _buf: RawVec<T>,
    iter: RawEntryIter<T>,
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

impl<T> IntoIterator for MiniVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            let iter = RawEntryIter::new(&self);
            let buf = ptr::read(&self.buf);
            mem::forget(self);
            IntoIter { _buf: buf, iter }
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

pub struct MiniVec<T> {
    buf: RawVec<T>,
    len: usize,
}

unsafe impl<T: Send> Send for MiniVec<T> {}
unsafe impl<T: Sync> Sync for MiniVec<T> {}

impl<T> MiniVec<T> {
    pub fn new() -> Self {
        Self {
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            buf: RawVec::with_capacity(cap),
            len: 0,
        }
    }

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn grow(&mut self) {
        self.buf.grow();
    }

    fn shrink(&mut self) {
        if cfg!(feature = "shrink") {
            self.buf.shrink(self.len);
        }
    }

    pub fn cap(&self) -> usize {
        self.buf.cap
    }
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> MiniVec<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.grow();
        }
        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            let result = unsafe { Some(ptr::read(self.ptr().add(self.len))) };
            self.shrink();
            result
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            None
        } else {
            unsafe { Some(&*self.ptr().add(self.len - 1)) }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(
            index <= self.len,
            "insertion index (is {}) should be <= len(is {})",
            index,
            self.len
        );
        if self.len == self.cap() {
            self.grow();
        }
        unsafe {
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(
            index < self.len,
            "removal index (is {}) should be < len (is {})",
            index,
            self.len
        );
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            self.shrink();
            result
        }
    }

    #[cfg(feature = "shuffle")]
    pub fn shuffle(&mut self)
    where
        T: PartialOrd,
    {
        use rand::Rng;
        if self.len < 2 {
            return;
        }
        let mut rng = rand::thread_rng();
        let mut last_index = self.len - 1;
        while last_index > 0 {
            let rand_index = rng.gen_range(0..=last_index);
            self.swap(rand_index, last_index);
            last_index -= 1;
        }
    }
}

impl<T> Drop for MiniVec<T> {
    fn drop(&mut self) {
        if self.cap() != 0 {
            while self.pop().is_some() {}
        }
    }
}

impl<T> Deref for MiniVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for MiniVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

impl<T> Default for MiniVec<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Extend<T> for MiniVec<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for elem in iter {
            self.push(elem);
        }
    }
}
impl<T> FromIterator<T> for MiniVec<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut minivec: MiniVec<T>;
        if cfg!(feature = "shrink") {
            minivec = MiniVec::with_capacity(16);
        } else {
            minivec = MiniVec::new();
        }
        for t in iter {
            minivec.push(t);
        }
        if cfg!(feature = "shrink") {
            minivec.shrink();
        }
        minivec
    }
}

impl<T: fmt::Debug> fmt::Debug for MiniVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.deref(), f)
    }
}

impl<T: PartialEq> PartialEq for MiniVec<T> {
    fn eq(&self, other: &MiniVec<T>) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter()
            .zip(other.iter())
            .all(|(this, that)| this == that)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(PartialEq)]
    struct Empty;

    #[derive(PartialEq, Debug, Default)]
    struct NonEmpty(Vec<i32>);

    #[test]
    fn minivec_new_test_1() {
        let v = MiniVec::<Empty>::new();
        assert!(v.cap() == usize::MAX);
    }

    #[test]
    #[cfg(feature = "shuffle")]
    fn minivec_shuffle_test_1() {
        let mut v = mv![1, 2, 3, 4, 5];
        v.shuffle();
        assert_ne!(v, mv![1, 2, 3, 4, 5]);
    }

    #[test]
    fn minivec_peek_test_1() {
        let v: MiniVec<bool> = mv![];
        let v1 = mv![1, 2, 3, 4, 5, 6];
        assert!(v.peek() == None);
        assert!(v1.peek() == Some(&6));
    }

    #[test]
    fn minivec_partial_eq_test_1() {
        let v = mv![1, 2, 3, 4, 5, 6];
        let v1 = mv![1, 2, 3, 4, 5, 6];
        assert!(v == v1);
    }

    #[test]
    #[cfg(feature = "shrink")]
    fn minivec_from_iter_test_1() {
        let v = mv![1, 2, 3, 4, 5, 6];
        let v = v.into_iter().collect::<Vec<_>>();
        let v = MiniVec::from_iter(v);
        assert!(v.cap() == 6);
        assert!(v.len() == 6);
    }

    #[test]
    fn minivec_macro_test_1() {
        let v: MiniVec<bool> = mv![];
        assert!(v.cap() == 0);
        let v: MiniVec<Empty> = mv![];
        assert!(v.cap() == usize::MAX);
    }
    #[test]
    fn minivec_macro_test_2() {
        let v = mv![1, 2, 3];
        assert!(v.len() == 3);
    }
    #[test]
    fn minivec_macro_test_3() {
        let v = mv![1; 3];
        assert!(v.len() == 3);
        for i in v {
            assert!(i == 1);
        }
    }

    #[test]
    fn minivec_iter_into_rev_test_1() {
        let mut v = MiniVec::<Empty>::new();
        v.insert(0, Empty);
        v.insert(1, Empty);
        v.insert(2, Empty);
        let mut iter = v.into_iter().rev();
        assert!(iter.next() == Some(Empty));
        assert!(iter.next() == Some(Empty));
        assert!(iter.next() == Some(Empty));
        assert!(iter.next().is_none());
    }

    #[test]
    fn minivec_new_test_2() {
        let v = MiniVec::<NonEmpty>::new();
        assert!(v.cap() == 0);
        assert!(v.len == 0);
    }

    #[test]
    fn minivec_with_capacity_test_1() {
        let mut v = MiniVec::<bool>::with_capacity(0);
        v.insert(0, true);
        v.push(false);
        assert!(v.cap() == 2);
        assert!(v.len() == 2);
        assert!(v.remove(0) == true);
    }

    #[test]
    fn minivec_with_capacity_test_2() {
        let v = MiniVec::<NonEmpty>::with_capacity(3);
        assert!(v.cap() == 3);
        assert!(v.len == 0);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn minivec_with_capacity_test_3() {
        let v = MiniVec::<NonEmpty>::with_capacity(3);
        assert!(v.cap() == 3);
        assert!(v.len == 0);
        let _ = &v[5];
    }

    #[test]
    fn minivec_with_capacity_test_4() {
        let mut v = MiniVec::<bool>::with_capacity(3);
        assert!(v.cap() == 3);
        assert!(v.len == 0);
        v.insert(0, true);
        assert!(v[0] == true);
    }

    #[test]
    #[should_panic(expected = "insertion index (is 1) should be <= len(is 0)")]
    fn minivec_with_capacity_test_5() {
        let mut v = MiniVec::<String>::with_capacity(3);
        assert!(v.cap() == 3);
        assert!(v.len == 0);
        v.insert(1, "one".to_string());
        assert_eq!(v[1], "one".to_string());
    }
    #[test]
    fn minivec_deref_debug_test_1() {
        let mut v = MiniVec::<&str>::new();
        v.push("1");
        v.push("2");
        v.push("3");
        println!("Debug = {:?}", v);
    }
    #[test]
    fn minivec_deref_mut_test_1() {
        let mut v = MiniVec::<&str>::new();
        v.push("1");
        v.push("2");
        v.push("3");
        let value = v.deref_mut();
        value[2] = "altered";
        assert!(v.last() == Some(&"altered"));
        assert!(v.iter().nth(2) == Some(&"altered"));
        assert!(v.iter().nth(2) == Some(&"altered"));
        assert!(v.iter_mut().nth(2) == Some(&mut "altered"));
    }

    #[test]
    fn minivec_iter_test_1() {
        let mut v = MiniVec::<i32>::new();
        v.push(1);
        v.push(2);
        v.push(3);
        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    #[cfg(feature = "shrink")]
    fn minivec_push_test_1() {
        let mut v = MiniVec::<NonEmpty>::new();
        assert!(v.cap() == 0);
        assert!(v.len == 0);
        v.push(NonEmpty(Vec::new()));
        assert!(v.cap() == 1);
        assert!(v.len == 1);
        v.push(NonEmpty(Vec::new()));
        assert!(v.cap() == 2);
        assert!(v.len == 2);

        v.push(NonEmpty(Vec::new()));
        assert!(v.cap() == 4);
        assert!(v.len == 3);

        v.push(NonEmpty(Vec::new()));
        assert!(v.cap() == 4);
        assert!(v.len == 4);

        v.push(NonEmpty(Vec::new()));
        assert!(v.cap() == 8);
        assert!(v.len == 5);
    }
    #[test]
    #[cfg(feature = "shrink")]
    fn minivec_pop_test_1() {
        let mut v = MiniVec::<NonEmpty>::new();
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        assert!(v.cap() == 8);
        assert!(v.len == 5);

        assert!(v.pop() == Some(NonEmpty(Vec::new())));
        assert!(v.cap() == 4);
        assert!(v.len == 4);
        assert!(v.pop() == Some(NonEmpty(Vec::new())));
        assert!(v.pop() == Some(NonEmpty(Vec::new())));
        assert!(v.pop() == Some(NonEmpty(Vec::new())));
        assert!(v.pop() == Some(NonEmpty(Vec::new())));
        assert!(v.pop() == None);
        assert!(v.cap() == 1);
        assert!(v.len == 0);
        v.push(NonEmpty(Vec::new()));
        assert!(v.cap() == 1);
        assert!(v.len == 1);
    }

    #[test]
    fn minivec_insert_test_1() {
        let mut v = MiniVec::<&str>::new();
        v.push("1");
        v.push("2");
        v.push("4");
        v.insert(2, "3");
        assert!(v.len == 4);
        assert!(v.pop() == Some(&"4"));
        assert!(v.pop() == Some(&"3"));
        assert!(v.pop() == Some(&"2"));
        assert!(v.pop() == Some(&"1"));
        assert!(v.pop() == None);
        assert!(v.len == 0);
    }

    #[test]
    #[cfg(feature = "shrink")]
    fn minivec_remove_test_1() {
        let mut v = MiniVec::<&str>::new();
        v.push("1");
        v.push("2");
        v.push("3");
        v.push("4");
        assert!(v.cap() == 4);
        assert!(v.remove(2) == "3");
        assert!(v.len == 3);
        assert!(v.remove(0) == "1");
        assert!(v.len == 2);
        assert!(v.remove(1) == "4");
        assert!(v.len == 1);
        assert!(v.cap() == 1);
        assert!(v.remove(0) == "2");
        assert!(v.len == 0);
        assert!(v.cap() == 1);
    }
    #[test]
    #[should_panic(expected = "removal index (is 1) should be < len (is 1)")]
    fn minivec_remove_test_2() {
        let mut v = MiniVec::<&str>::new();
        v.insert(0, "zero");
        let _ = v.remove(1);
    }

    #[test]
    fn minivec_iter_mut_test_1() {
        let mut v = MiniVec::<i32>::new();
        v.push(1);
        v.push(2);
        v.push(3);

        if let Some(elem) = v.iter_mut().last() {
            *elem *= 100;
        }
        assert_eq!(v.pop(), Some(300));
    }

    #[test]
    fn minivec_index_test_1() {
        let mut v = MiniVec::<String>::new();
        v.push("one".to_string());
        let one = &mut v[0];
        one.push_str("two");
        assert!(v.pop() == Some("onetwo".to_string()));
        assert!(v.len == 0);
    }

    #[test]
    fn minivec_first_last_test_1() {
        let mut v = MiniVec::<&str>::new();
        v.push("1");
        v.push("2");
        v.push("3");
        assert!(v.first() == Some(&"1"));
        assert!(v.last() == Some(&"3"));
    }

    #[test]
    fn minivec_deref_test_1() {
        let mut v = MiniVec::<&str>::new();
        v.push("1");
        v.push("2");
        v.push("3");
        assert!("3" == v.deref()[2]);
    }

    #[test]
    fn minivec_into_iter_test_1() {
        let mut vec = MiniVec::<i32>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let mut value = 1;
        for v in vec {
            assert!(v == value);
            value += 1;
        }
    }

    #[test]
    fn minivec_into_iter_next_back_test_1() {
        let mut vec = MiniVec::<i32>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let mut value = 3;
        for v in vec.into_iter().rev() {
            assert!(v == value);
            value -= 1;
        }

        let mut vec = MiniVec::<i32>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        let mut iter = vec.into_iter();
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), None);
    }
}

#[macro_export]
macro_rules! mv {
    [] => {
        $crate::MiniVec::new()
    };
    [ $($elem:expr ),+ ] => {
        {
            let mut count = 0;
            $(
                count += 1;
                let _ = &$elem;
            )+
            let mut mini_vec = $crate::MiniVec::with_capacity(count);
            $(
                mini_vec.push($elem);
            )+
            mini_vec
        }
    };
    [$elem:expr; $count:expr] => {
        {
          let count = $count;
          let mut mini_vec = $crate::MiniVec::with_capacity(count);
          let elem = $elem;
            mini_vec.extend(std::iter::repeat(elem).take(count));
          mini_vec
        }
    };
}
