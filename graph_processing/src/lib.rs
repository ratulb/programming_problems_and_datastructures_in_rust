pub mod dfs;
pub mod graphs;

#[cfg(test)]
mod tests {
    use crate::graphs::Graph;
    #[test]
    fn it_works_within_lib() {
        Graph::new(1);
    }
}
