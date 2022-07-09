# LRU cache

Here we implement a cache that can hold key -> value mappings in memory. The maximum 
capacity of cache is specified during instantiation and when storage reaches maximun 
capacity - least recently used entry is evicted.

The cache exposes the follwoing APIs:
 - `put` - inserts or updates
 - `get` - get a value corresponding to a key
 - `delete` - delete an entry from the cache

> **Note**: The cache implementation depends on the doubly linked list implementation 
discussed in the previous section. Another noticeable thing is that - because our LRU 
cache implementation uses doulby linked list APIs, the implementation is quite concise, 
its about sixty odd lines when comments are ignored.
### Following is the complete implementation:
```rust, ignore
/// LRU cache implementation
use doubly_linked_list::{List, Node};
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};
//LRU Cache struct
pub struct LRUCache<K: Eq + Hash, V: std::fmt::Debug + Default + Clone + PartialEq> {
    keys: HashMap<K, Weak<RefCell<Node<V>>>>,
    entries: List<V>,
    capacity: usize,
}

impl<K: Eq + Hash, V: std::fmt::Debug + Default + Clone + PartialEq> LRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            keys: HashMap::new(),
            entries: List::new(),
            capacity,
        }
    }
    //Insert (key, value)  to the cache
    //If key is already present - its value will be updated
    //If backing storage (doubly linked list) size goes beyond cache capacity,
    //least recently used entries will be evicted
    pub fn put(&mut self, key: K, v: V) {
        match self.keys.get(&key).and_then(|key| key.upgrade()) {
            //Insert if not present
            None => {
                self.entries.push_front(v);
                if let Some(front) = self.front() {
                    self.keys.insert(key, front);
                }
                self.purge_least_recently_used();
            }
            //Update if already exists
            Some(ref mut entry) => {
                entry.borrow_mut().replace(v);
                self.entries.to_front(entry);
            }
        }
    }
    //Called internally to get rid of least recently used entries
    fn purge_least_recently_used(&mut self) {
        if self.entries.size() > self.capacity {
            for _ in 0..(self.entries.size() - self.capacity) {
                let _ = self.entries.pop_back();
            }
        }
    }
    //Get a value corresponding to a key
    //Accessed entry, if found, moves to the front of backing storage
    pub fn get(&mut self, key: &K) -> Option<V> {
        match self.keys.get(key).and_then(|key| key.upgrade()) {
            None => None,
            Some(ref entry) => {
                self.entries.to_front(entry);
                Some(entry.borrow().key().clone())
            }
        }
    }
    //Delete a cache entry
    pub fn delete(&mut self, key: &K) -> Option<V> {
        match self.keys.get(key).and_then(|key| key.upgrade()) {
            None => None,
            Some(ref entry) => {
                self.keys.remove(key);
                self.entries.delete_target(entry)
            }
        }
    }
    //Get a weak reference to newly inserted value in the backing store
    //This value is stored in the lookup HashMap against its key
    fn front(&self) -> Option<Weak<RefCell<Node<V>>>> {
        self.entries.head().as_ref().map(Rc::downgrade)
    }
}
```
[Source](https://github.com/ratulb/programming_problems_in_rust/blob/master/lru_cache/src/lib.rs)
