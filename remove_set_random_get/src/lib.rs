use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

pub struct SetRemoveAndrandomGet<T> {
    entries: HashMap<Rc<T>, usize>,
    positions: Vec<Rc<T>>,
}
impl<T: Eq + Hash + std::fmt::Debug> SetRemoveAndrandomGet<T> {
    pub fn new() -> Self {
        Self {
            entries: HashMap::<Rc<T>, usize>::new(),
            positions: Vec::<Rc<T>>::new(),
        }
    }

    pub fn set(&mut self, value: T) -> bool {
        let rc = Rc::new(value);
        if self.entries.contains_key(&rc) {
            return false;
        }
        self.positions.push(rc.clone());
        self.entries.insert(rc, self.positions.len() - 1);
        println!("positions = {:?}", self.positions);
        println!("entries = {:?}", self.entries);
        true
    }

    pub fn random_get(&self) -> Option<&T> {
        if self.positions.is_empty() {
            return None;
        }
        let index = rand::thread_rng().gen_range(0..self.positions.len());
        Some(self.positions[index].borrow())
    }

    pub fn remove(&mut self, value: T) -> Option<T> {
        println!("Remving positions = {:?}, {:?}", self.positions, value);
        println!("Remving entries = {:?}, {:?}", self.entries, value);

        let rc = Rc::new(value);
        let removed = self.entries.remove(&rc);
        let len = self.positions.len();
        if let Some(index) = removed {
            self.positions.swap(index, len - 1);
            let swapped = &self.positions[index];
            self.entries.get_mut(swapped).map(|idx| *idx = index);
            let rc_value = self.positions.remove(len - 1);
            match Rc::try_unwrap(rc_value) {
                Ok(value) => return Some(value),
                Err(err) => {
                    eprintln!("Error removing {:?}", err);
                    return None;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::SetRemoveAndrandomGet;
    #[test]
    fn set_test1() {
        let mut srarg = SetRemoveAndrandomGet::new();
        srarg.set("test1");
        srarg.set("test2");
        srarg.set("test3");
        srarg.set("test4");
    }
    #[test]
    fn get_test1() {
        let mut srarg = SetRemoveAndrandomGet::new();
        srarg.set("test1");
        srarg.set("test2");
        srarg.set("test3");
        srarg.set("test4");

        match srarg.random_get() {
            Some(v) => println!("Value 1 = {:?}", v),
            None => panic!(),
        }
        match srarg.random_get() {
            Some(v) => println!("Value 2 = {:?}", v),
            None => panic!(),
        }
        match srarg.random_get() {
            Some(v) => println!("Value 3 = {:?}", v),
            None => panic!(),
        }
    }
    #[test]
    fn remove_test1() {
        let mut srarg = SetRemoveAndrandomGet::new();
        srarg.set("test1");
        srarg.set("test2");
        srarg.set("test3");
        srarg.set("test4");
        assert_eq!(srarg.remove("test5"), None);
        assert_eq!(srarg.remove("test1"), Some("test1"));
        assert_eq!(srarg.remove("test2"), Some("test2"));
        assert_eq!(srarg.remove("test3"), Some("test3"));
        assert_eq!(srarg.remove("test4"), Some("test4"));
        assert_eq!(srarg.remove("test1"), None);
    }
}
