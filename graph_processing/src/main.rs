use graph_processing::graphs::Graph;

pub fn main() {

    let mut graph = Graph::new(4);
    println!("{}", graph.to_string());
    graph.add_edge(0,1);
    graph.add_edge(2,1);
    graph.add_edge(2,3);
    println!("{}", graph.to_string());
}
