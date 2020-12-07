use petgraph::algo;
use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::IntoNodeReferences;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

enum ParseState {
    Parent,
    Child,
}

fn find_node_index(graph: &Graph<String, usize>, item: &str) -> Option<NodeIndex> {
    for (id, node) in graph.node_references() {
        if node.starts_with(item) {
            return Some(id);
        }
    }
    return None;
}

fn part1(file: &str) -> usize {
    let mut graph = Graph::<String, usize>::new();
    let reader = BufReader::new(File::open(file).expect("File::open failed"));
    for line in reader.lines() {
        let mut buf = String::new();
        let mut n_children = 1;
        let mut parent: Option<NodeIndex> = None;
        let mut state = ParseState::Parent;
        for word in line.unwrap().split_whitespace() {
            if word.starts_with("bag") {
                if !buf.starts_with("no other") {
                    match state {
                        ParseState::Parent => {
                            parent = find_node_index(&graph, &buf.trim_end());
                            if parent == None {
                                parent = Some(graph.add_node(String::from(buf.trim_end())));
                            }
                        }
                        ParseState::Child => {
                            let mut child = find_node_index(&graph, &buf.trim_end());
                            if child == None {
                                child = Some(graph.add_node(String::from(buf.trim_end())));
                            }
                            if let Some(parent) = parent {
                                if let Some(child) = child {
                                    graph.add_edge(parent, child, n_children);
                                }
                            }
                        }
                    }
                }
                if word.ends_with(".") {
                    state = ParseState::Parent;
                }
                buf.clear();
            } else if word.starts_with("contain") {
                // start of child
                state = ParseState::Child;
            } else if let Ok(n) = word.parse::<usize>() {
                // number of children
                n_children = n;
            } else {
                buf.push_str(word);
                buf.push_str(" ");
            }
        }
    }
    let start = find_node_index(&graph, "shiny gold");
    let mut n_bags = 0;
    if let Some(start) = start {
        for (id, n) in graph.node_references() {
            if id != start {
                if algo::has_path_connecting(&graph, id, start, None) {
                    n_bags += 1;
                }
            }
        }
    }
    //    println!("{}", Dot::new(&graph));
    println!("{}", n_bags);
    return 0;
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if &args[2] == "1" {
        println!("{}", part1(&args[1]));
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff() {}
}
