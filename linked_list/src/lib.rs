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
        ll.print();
    }
}
