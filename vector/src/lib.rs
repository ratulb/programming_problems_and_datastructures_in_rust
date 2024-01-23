use std::alloc::{self, Layout};
//use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

pub struct IntoIter<T> {
    buff: NonNull<T>,
    cap: usize,
    start: *const T,
    end: *const T,
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            for _ in &mut *self {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.buff.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> IntoIterator for Vector<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let vec = ManuallyDrop::new(self);
        let (ptr, cap, len) = (vec.ptr, vec.cap, vec.len);
        unsafe {
            IntoIter {
                buff: ptr,
                cap: cap,
                start: ptr.as_ptr(),
                end: if cap == 0 {
                    ptr.as_ptr()
                } else {
                    ptr.as_ptr().add(len)
                },
            }
        }
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = Some(ptr::read(self.start));
                self.start = self.start.offset(1);
                result
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

#[derive(Debug)]
pub struct Vector<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for Vector<T> {}
unsafe impl<T: Sync> Sync for Vector<T> {}

impl<T> Vector<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }
}

impl<T> Vector<T> {
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "Index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr.as_ptr().add(index));
            ptr::copy(
                self.ptr.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "Index out of bounds");
        if self.len == self.cap {
            self.grow();
        }
        unsafe {
            ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr.as_ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let new_layout = match Layout::array::<T>(new_cap) {
            Ok(layout) => layout,
            Err(err) => panic!("Allocation too large! {}", err),
        };

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}
impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while self.pop().is_some() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) }
            self.cap = 0;
        }
    }
}

impl<T> Deref for Vector<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Empty;
    #[derive(PartialEq, Debug)]
    struct NonEmpty(Vec<i32>);

    #[test]
    fn vector_into_iter_test_1() {
        let mut vec = Vector::<i32>::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        let mut value = 1;
        for v in vec {
            assert!(v == value);
            value += 1;
        }
    }

    #[test]
    fn vector_iter_mut_test_1() {
        let mut v = Vector::<i32>::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);

        if let Some(elem) = v.iter_mut().last() {
            *elem *= 100;
        }
        assert_eq!(v.pop(), Some(400));
    }

    #[test]
    fn vector_index_test_1() {
        let mut v = Vector::<String>::new();
        v.push("one".to_string());
        let one = &mut v[0];
        one.push_str("two");
        assert!(v.pop() == Some("onetwo".to_string()));
        assert!(v.len == 0);
    }

    #[test]
    fn vector_iter_test_1() {
        let mut v = Vector::<i32>::new();
        v.push(1);
        v.push(2);
        v.push(3);
        v.push(4);
        let mut iter = v.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vector_remove_test_1() {
        let mut v = Vector::<&str>::new();
        v.push("1");
        v.push("2");
        v.push("3");
        v.push("4");
        assert!(v.remove(2) == "3");
        assert!(v.len == 3);

        assert!(v.remove(0) == "1");
        assert!(v.len == 2);

        assert!(v.remove(1) == "4");
        assert!(v.len == 1);

        assert!(v.remove(0) == "2");
        assert!(v.len == 0);
    }

    #[test]
    fn vector_insert_test_1() {
        let mut v = Vector::<&str>::new();
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
    fn vector_deref_mut_test_1() {
        let mut v = Vector::<&str>::new();
        v.push("1");
        v.push("2");
        v.push("3");
        let value = v.deref_mut();
        value[2] = "altered";
        assert!(v.last() == Some(&"altered"));
    }

    #[test]
    #[should_panic(expected = "We're not ready to handle ZSTs")]
    fn vector_new_test_1() {
        let _v = Vector::<Empty>::new();
        unreachable!();
    }
    #[test]
    fn vector_first_last_test_1() {
        let mut v = Vector::<&str>::new();
        v.push("1");
        v.push("2");
        v.push("3");
        assert!(v.first() == Some(&"1"));
        assert!(v.last() == Some(&"3"));
    }

    #[test]
    fn vector_deref_test_1() {
        let mut v = Vector::<&str>::new();
        v.push("1");
        v.push("2");
        v.push("3");
        assert!("3" == v.deref()[2]);
    }

    #[test]
    fn vector_new_test_2() {
        let v = Vector::<NonEmpty>::new();
        assert!(v.cap == 0);
        assert!(v.len == 0);
        let lo = Layout::new::<NonEmpty>();
        println!("Align:{}", lo.align());
        println!("size:{}", lo.size());
        //let lo = Layout::array::<NonEmpty>((isize::MAX/12) as usize).unwrap();
    }
    #[test]
    fn vector_push_test_1() {
        let mut v = Vector::<NonEmpty>::new();
        assert!(v.cap == 0);
        assert!(v.len == 0);
        v.push(NonEmpty(Vec::new()));
        assert!(v.cap == 1);
        assert!(v.len == 1);
        v.push(NonEmpty(Vec::new()));
        assert!(v.cap == 2);
        assert!(v.len == 2);

        v.push(NonEmpty(Vec::new()));
        assert!(v.cap == 4);
        assert!(v.len == 3);

        v.push(NonEmpty(Vec::new()));
        assert!(v.cap == 4);
        assert!(v.len == 4);

        v.push(NonEmpty(Vec::new()));
        assert!(v.cap == 8);
        assert!(v.len == 5);
    }
    #[test]
    fn vector_pop_test_1() {
        let mut v = Vector::<NonEmpty>::new();
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        assert!(v.cap == 8);
        assert!(v.len == 5);

        assert!(v.pop() == Some(NonEmpty(Vec::new())));
        assert!(v.cap == 8);
        assert!(v.len == 4);
        assert!(v.pop() == Some(NonEmpty(Vec::new())));
        assert!(v.pop() == Some(NonEmpty(Vec::new())));
        assert!(v.pop() == Some(NonEmpty(Vec::new())));
        assert!(v.pop() == Some(NonEmpty(Vec::new())));
        assert!(v.pop() == None);
        assert!(v.cap == 8);
        assert!(v.len == 0);
    }

    #[test]
    fn vector_drop_test_1() {
        let mut v = Vector::<NonEmpty>::new();
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        v.push(NonEmpty(Vec::new()));
        assert!(v.cap == 8);
        assert!(v.len == 5);
        drop(v);
    }
}
