///Optimized and efficient implementation of union find

pub struct UnionFind {
    id: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    pub fn new(nodes: usize) -> Self {
        let mut id = vec![0; nodes];
        for i in 0..nodes {
            id[i] = i;
        }
        UnionFind {
            id: id,
            sizes: vec![1; nodes],
        }
    }

    pub fn root(&mut self, node: usize) -> Option<usize> {
        if node >= self.id.len() {
            None
        } else {
            let mut i = node;
            while i != self.id[i] {
                self.id[i] = self.id[self.id[i]];
                i = self.id[i];
            }
            Some(i)
        }
    }

    pub fn connected(&mut self, node1: usize, node2: usize) -> bool {
        if node1 >= self.id.len() || node2 >= self.id.len() {
            false
        } else {
            self.root(node1) == self.root(node2)
        }
    }
    pub fn union(&mut self, node1: usize, node2: usize) -> bool {
        if node1 >= self.id.len() || node2 >= self.id.len() {
            false
        } else {
            let i = self.root(node1).unwrap();
            let j = self.root(node2).unwrap();
            if i == j {
                true
            } else {
                if self.sizes[i] > self.sizes[j] {
                    self.id[j] = i;
                    self.sizes[i] += self.sizes[j];
                } else {
                    self.id[i] = j;
                    self.sizes[j] += self.sizes[i]
                }
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;
    #[test]
    fn zero_size_union_find_test() {
        let mut uf = UnionFind::new(0);
        assert_eq!(uf.root(0), None);
    }
    #[test]
    fn one_size_union_find_test() {
        let mut uf = UnionFind::new(1);
        assert_eq!(uf.root(0), Some(0));
        assert_eq!(uf.connected(0, 0), true);
    }
    #[test]
    fn union_find_test() {
        let mut uf = UnionFind::new(10);
        uf.union(1, 9);
        uf.union(9, 7);
        uf.union(3, 7);
        assert_eq!(uf.connected(1, 3), true);
    }
}
