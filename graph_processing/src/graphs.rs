#[derive(Debug, PartialEq, Clone)]
pub struct Node<Item> {
    item: Item,
    next: Option<Box<Node<Item>>>,
}

impl<Item> Node<Item> {
    pub fn new(item: Item) -> Self {
        Node {
            item: item,
            next: None,
        }
    }
    pub fn new_with(item: Item, node: Option<Box<Node<Item>>>) -> Self {
        Node {
            item: item,
            next: node,
        }
    }
    pub fn get_item(&self) -> Option<&Item> {
        Some(&self.item)
    }
    pub fn set_item(&mut self, new_item: Item) {
        self.item = new_item;
    }
}

use std::fmt;
impl<Item: fmt::Display> fmt::Display for Node<Item> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.item)
    }
}

#[derive(Clone, Debug)]
pub struct Bag<Item> {
    first: Option<Box<Node<Item>>>,
    element_count: usize,
}

impl<Item> Bag<Item> {
    pub fn new() -> Self {
        Bag {
            first: None,
            element_count: 0,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.first.is_none()
    }
    pub fn size(&self) -> usize {
        self.element_count
    }
    pub fn add_item(&mut self, new_item: Item) {
        self.first = Some(Box::new(Node::new_with(new_item, self.first.take())));
        self.element_count += 1;
    }
    pub fn iter<'a>(&'a self) -> Iter<'a, Item> {
        //Iter {next: self.first.as_ref().map(|node|&**node)}
        //Iter {next: self.first.as_deref()}
        Iter {
            next: self.first.as_ref().map::<&Node<Item>, _>(|node| &node),
        }
    }
}

impl<Item> Drop for Bag<Item> {
    fn drop(&mut self) {
        let mut curr_first = self.first.take();
        while let Some(mut boxed_node) = curr_first {
            curr_first = boxed_node.next.take();
        }
    }
}

pub struct Iter<'a, Item> {
    next: Option<&'a Node<Item>>,
}

impl<'a, ItemType> Iterator for Iter<'a, ItemType> {
    type Item = &'a ItemType;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.item
        })
    }
}

const NEWLINE: char = '\n';

#[derive(Debug)]
pub struct Graph {
    vertices: usize,
    edges: usize,
    neighbours: Vec<Bag<usize>>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        Graph {
            vertices: size,
            edges: 0,
            neighbours: vec![Bag::new(); size],
        }
    }
    pub fn vertices(&self) -> usize {
        self.vertices
    }
    pub fn edges(&self) -> usize {
        self.edges
    }
    fn validate_vertex(&self, vertex: usize) -> Result<bool, String> {
        if vertex >= self.vertices {
            let err = format!(
                "Vertex {} is greater than graph size {}",
                vertex, self.vertices
            );
            println!("{}", err);
            Err(err)
        } else {
            Ok(true)
        }
    }
    pub fn adjacents<'a>(&'a self, vertex: usize) -> Option<Iter<'a, usize>> {
        match self.validate_vertex(vertex) {
            Err(_) => None,
            Ok(_) => Some(self.neighbours[vertex].iter()),
        }
    }
    pub fn add_edge(&mut self, u: usize, v: usize) -> bool {
        if self.validate_vertex(u).and(self.validate_vertex(v)).is_ok() {
            self.neighbours[u].add_item(v);
            self.neighbours[v].add_item(u);
            self.edges += 1;
            true
        } else {
            false
        }
    }
    pub fn degree(&self, vertex: usize) -> Option<usize> {
        match self.validate_vertex(vertex) {
            Ok(_) => Some(self.neighbours[vertex].size()),
            Err(_) => None,
        }
    }
    pub fn to_string(&self) -> String {
        let mut graph = String::from("Vertices: ");
        graph.push_str(&self.vertices.to_string());
        graph.push_str(" and edges: ");
        graph.push_str(&self.edges.to_string());
        graph.push(NEWLINE);
        for v in 0..self.vertices {
            let vs = v.to_string() + " -> ";
            graph.push_str(&vs);
            for e in self.neighbours[v].iter() {
                let es = e.to_string() + " ";
                graph.push_str(&es);
            }
            graph.push(NEWLINE);
        }
        graph
    }
}

#[cfg(test)]
mod tests {
    use super::Node;
    #[test]
    fn test_node_creation() {
        assert_eq!(Node::new(1), Node::new(1));
    }
    #[test]
    fn test_node_creation_with() {
        let node1 = Node::new(1);
        let node2 = Node::new_with(2, Some(Box::new(node1)));
        assert_eq!(node2, Node::new_with(2, Some(Box::new(Node::new(1)))));
    }
    #[test]
    fn test_node_get_item() {
        let mut node = Node::new(1);
        assert_eq!(node.get_item(), Some(&1));
        node.set_item(2);
        assert_eq!(node.get_item(), Some(&2));
    }
}
