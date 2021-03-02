#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T: std::fmt::Display> LinkedList<T> {
    pub fn new(value: T) -> LinkedList<T> {
        let link = Some(Box::new(LinkNode::new(value)));
        LinkedList { head: link }
    }
    pub fn empty() -> Self {
        LinkedList { head: None }
    }
    pub fn push_front(&mut self, value: T) -> &mut Self {
        match self.head.take() {
            Some(h) => {
                let new_head = LinkNode {
                    value: value,
                    next: Some(h),
                };
                self.head = Some(Box::new(new_head));
            }
            None => {
                self.head = Some(Box::new(LinkNode::new(value)));
            }
        }
        self
    }

    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            Some(h) => {
                self.head = h.next;
                Some(h.value)
            }
            None => None,
        }
    }

    pub fn print(&self) {
        match &self.head {
            None => println!("None"),
            Some(h) => h.print(),
        };
    }
}

type Link<T> = Option<Box<LinkNode<T>>>;
#[derive(Debug)]
struct LinkNode<T> {
    value: T,
    next: Option<Box<LinkNode<T>>>,
}
impl<T: std::fmt::Display> LinkNode<T> {
    fn new(value: T) -> LinkNode<T> {
        LinkNode {
            value: value,
            next: None,
        }
    }

    fn print(&self) {
        print!("{} ", self.value);
        if let Some(n) = &self.next {
            n.print();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let ll = LinkedList::new(100);
        //assert_eq!(LinkedList::new(100), ll);
    }
    #[test]
    fn test_print() {
        let ll: LinkedList<i32> = LinkedList::empty();
        ll.print();
        let ll = LinkedList::new(100);
        ll.print();
        let mut ll: LinkedList<i32> = LinkedList::empty();
        ll.push_front(200);
        ll.print();
        ll.push_front(300);
        ll.push_front(400);
        ll.push_front(500).push_front(600);
        ll.print();
    }
    #[test]
    fn test_pop_front() {
        let mut ll: LinkedList<i32> = LinkedList::empty();
        assert_eq!(ll.pop_front(), None);
        ll.push_front(100);
        assert_eq!(ll.pop_front(), Some(100));
        ll.push_front(1).push_front(2).push_front(3);
        println!("Printing the entries");
        ll.print();
        assert_eq!(ll.pop_front(), Some(3));
        assert_eq!(ll.pop_front(), Some(2));
        assert_eq!(ll.pop_front(), Some(1));
        assert_eq!(ll.pop_front(), None);
    }
}
