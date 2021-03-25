use crate::graphs::Graph;
#[derive(Debug)]

pub struct DfsRecursive<'a> {
    visited: Vec<bool>,
    predecessors: Vec<Option<usize>>,
    graph: &'a Graph,
}
impl<'a> DfsRecursive<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        DfsRecursive {
            visited: vec![false; graph.vertices()],
            predecessors: vec![None; graph.vertices()],
            graph: graph,
        }
    }

    pub fn dfs(&mut self, vertex: usize) -> Result<bool, String> {
        match self.validate_vertex(vertex) {
            Err(m) => return Err(m),
            Ok(_) => {
                for v in &mut self.visited {
                    *v = false;
                }
                for i in &mut self.predecessors {
                    *i = None;
                }
                self.do_dfs(vertex);
                return Ok(true);
            }
        }
    }

    fn do_dfs(&mut self, vertex: usize) {
        let adjacents = self.graph.adjacents(vertex);
        if adjacents.is_some() {
            self.visited[vertex] = true;
            for vert in adjacents.unwrap() {
                if !self.visited[*vert] {
                    self.predecessors[*vert] = Some(vertex);
                    self.do_dfs(*vert);
                }
            }
        }
    }
    //Dfs has to happen from some other vertex before this call
    pub fn has_path_to(&self, vertex: usize) -> bool {
        match self.validate_vertex(vertex) {
            Err(_m) => return false,
            Ok(_) => return self.visited[vertex],
        }
    }

    pub fn path(&mut self, from: usize, to: usize) -> Option<Vec<usize>> {
        match self.dfs(from) {
            Err(_) => return None,
            Ok(_) => {
                if !self.has_path_to(to) {
                    return None;
                } else {
                    let mut path = Vec::<usize>::new();
                    let mut p = from;
                    while p != to {
                        path.push(p);
                        p = self.predecessors[p].unwrap();
                    }
                    path.push(to);
                    Some(path)
                }
            }
        }
    }

    fn validate_vertex(&self, vertex: usize) -> Result<bool, String> {
        let vertex_count = self.graph.vertices();
        if vertex >= vertex_count {
            let msg = format!(
                "Given vertex {} is not one of 0..{}",
                vertex,
                vertex_count - 1
            );
            println!("{}", msg);
            Err(msg)
        } else {
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DfsRecursive;
    use crate::graphs::Graph;
    #[test]
    fn it_works_in_dfs() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(2, 1);
        graph.add_edge(2, 3);
        graph.add_edge(2, 30);
        let mut dfsr = DfsRecursive::new(&graph);
        assert_eq!(dfsr.dfs(0), Ok(true));
    }
}
