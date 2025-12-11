//! Puzzle 11: Reactor
//! ==================
//!
//! Ah, the "joys" of cable management...

use advent_2025::{read_file, Puzzle, AdventError};

use std::collections::hash_map::RandomState;

use petgraph::prelude::DiGraph;
use petgraph::algo::simple_paths::all_simple_paths;

#[derive(Clone, Debug)]
struct Rack(DiGraph<String, ()>);

impl Puzzle for Rack {
    fn parse_input(file: &str) -> Result<Self, AdventError> {
        let connections = file.lines().map(|line| {
            let items = line.split_ascii_whitespace().collect::<Vec<_>>();
            let Some((node, neighbors)) = items
                .split_first() else {
                    return Err(AdventError::Parse("line should not be empty".to_string()));
                };
            let Some(node) = node.strip_suffix(':') else {
                return Err(AdventError::Parse(format!("improper list format: {0}", line)));
            };
            // Handle lifetime issues.
            let neighbors = neighbors.iter().map(|s| s.to_string()).collect::<Vec<_>>();
            Ok((node, neighbors))
        }).collect::<Result<Vec<_>, AdventError>>()?;

        let mut graph: DiGraph<String, ()> = DiGraph::new();
        let nodes = connections.iter().map(|(node, _)| {
            let node = graph.add_node(node.to_string());
            node
        }).collect::<Vec<_>>();
        // `out` doesn't link to any nodes, so add it to the graph manually.
        graph.add_node("out".to_string());

        let neighbors = connections.iter().map(|(_, neighbors)| neighbors).collect::<Vec<_>>();
        for idx in 0..nodes.len() {
            let node = nodes[idx];
            let neighbors = neighbors[idx];

            let neighbors = graph.node_indices().filter(|i| neighbors.contains(&graph[*i]));
            let edges = neighbors.map(|idx| (node, idx, ())).collect::<Vec<_>>();
            graph.extend_with_edges(&edges);
        }

        Ok(Rack(graph))
    }

    fn part_one(&self) -> Result<String, AdventError> {
        let Some(you) = self.0.node_indices().find(|idx| self.0[*idx] == "you") else {
            return Err(AdventError::Data(format!("root node not found: {0:?}", self.0)));
        };
        let Some(out) = self.0.node_indices().find(|idx| self.0[*idx] == "out") else {
            return Err(AdventError::Data(format!("end node not found: {0:?}", self.0)));
        };
        
        let paths = all_simple_paths::<Vec<_>, _, RandomState>(&self.0, you, out, 1, None).count();
        Ok(paths.to_string())
    }

    fn part_two(&self) -> Result<String, AdventError> {
        todo!()
    }
}

fn main() -> Result<(), AdventError> {
    let file = read_file("src/input/puzzle11.txt")?;
    let data = Rack::parse_input(&file)?;

    println!("The number of paths from 'you' to 'out' is {0}", data.part_one()?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    use std::sync::LazyLock;

    static TEST_INPUT: LazyLock<String> = LazyLock::new(
        || read_file("src/input/puzzle11-test.txt").expect("could not read input file"));

    #[test]
    // This needs a better test, but I can't be bothered.
    fn parse_input() {
        let data = Rack::parse_input(&TEST_INPUT).expect("could not parse input file");

        let you = data.0.node_indices().find(|node| data.0[*node] == "you")
            .expect("graph should have 'you' node");
        assert_eq!(data.0.neighbors(you).count(), 2);
    }

    #[test]
    fn part_one() {
        let data = Rack::parse_input(&TEST_INPUT).expect("could not parse input file");

        let answer = data.part_one().unwrap();
        assert_eq!(answer, "5");
    }
}
