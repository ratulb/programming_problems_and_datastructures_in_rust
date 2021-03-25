use graph_processing::dfs::DfsRecursive;
use graph_processing::graphs::Graph;

pub fn main() {
    let mut graph = Graph::new(4);
    graph.add_edge(0, 1);
    graph.add_edge(2, 1);
    graph.add_edge(2, 3);
    //Adding edge with unknown  node would be ignored
    graph.add_edge(2, 30);
    let mut dfsr = DfsRecursive::new(&graph);

    match dfsr.dfs(0) {
        Err(e) => println!("{}", e),
        Ok(o) => println!("{}", o),
    };
}
