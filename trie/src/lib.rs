/*** Implement Trie data structure that holds string ***/

use std::collections::HashMap;
struct Node {
    children: HashMap<char, Node>,
    is_word: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
            is_word: false,
        }
    }
}
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: Node::new() }
    }
    //Insert a word into the Trie
    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        let mut chars = word.chars();
        while let Some(c) = chars.next() {
            current = current.children.entry(c).or_insert(Node::new());
        }
        current.is_word = true;
    }
    //Seach for word in the Trie
    pub fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        let mut chars = word.chars();
        while let Some(c) = chars.next() {
            match current.children.get(&c) {
                Some(ref node) => current = node,
                None => return false,
            }
        }
        current.is_word
    }
    //Is there a word that starts with the given prefix
    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        let mut chars = prefix.chars();
        while let Some(c) = chars.next() {
            match current.children.get(&c) {
                Some(ref node) => current = node,
                None => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;
    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert!(trie.search("hello"));
        assert!(!trie.search("world"));
        assert!(trie.starts_with("hell"));
        assert!(!trie.starts_with("helo"));
    }
}
