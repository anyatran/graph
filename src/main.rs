//! provides a path query interface to a graph from a file provided on the command line
//! ----------------------------------------------------------------------------------------------
//! the problem statement was ambiguous as to whether the given graph is directed or undirected.
//! we assumed a directed graph.
//! ----------------------------------------------------------------------------------------------
#![allow(unstable)]

use std::os;
use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

type Graph = HashMap<String, Vec<String>>;

static PROMPT: &'static str = "-> ";

#[cfg(not(test))]
fn main() {

    let file = match os::args().tail().first() {
        Some(arg) => io::File::open(&Path::new(arg)),
        None      => panic!("Must provide a file"),   
    };

    let file_buff = io::BufferedReader::new(file);
    let graph: Box<Graph> = match load_graph(file_buff) {
        Ok(g) => Box::new(g),
        Err(e) => panic!("{}", e),
    };

    io::stdio::print(PROMPT);
    for line in io::stdin().lock().lines() {

        let input_line: String = line.unwrap();
        let input: Vec<&str> = input_line.as_slice()
                                         .trim_matches('\n')
                                         .split(' ')
                                         .filter(|&x| x.len() > 0)
                                         .collect();

        if input.clone().len() != 2 {
            println!("Must provide only start and end");
            io::stdio::print(PROMPT);
            continue;
        }

        let path: String = match find_path(&*graph, input[0].to_string(), input[1].trim_matches('\n').to_string()) {
            Some(p) => p,
            None => { 
                println!("No path from {} to {}", input[0], input[1]); 
                io::stdio::print(PROMPT);
                continue;
            },  
        };

        println!("{}", path);

        io::stdio::print(PROMPT);
    }
}

/// build a graph from the given buffer. Returns Ok(HashMap<String, Vec<String>>) on success,
/// otherwise returns Err(String) containing an error message.
fn load_graph<'a, R: Reader>(mut content: io::BufferedReader<R>) -> Result<Graph, String> {
    let mut graph_result: Graph = HashMap::new();
    for line in content.lines() {
        match line {
            Ok(l) => {
                // line format: <node><space><neighbor1><space><neighbor2>...<neighborN>
                let mut node: Vec<&str> = l.as_slice().split(' ').collect();
                let node_name: &str = node.remove(0).trim_matches('\n');
                let neighbors: Vec<String> = node.iter()
                                                 .map(|&x| x.trim_matches('\n').to_string())
                                                 .collect();
                match graph_result.entry(node_name.to_string()) {
                    Vacant(entry) => { entry.insert(neighbors); },
                    Occupied(_) => {
                        return Err(format!("Duplicate entry: {}", node_name));
                    },
                }
            },
            Err(_) => { return Err("Unrecoverable error while reading graph file".to_string()); },
        };
    }
    let boxed_graph: Box<Graph> = Box::new(graph_result);
    if !valid_graph(&*boxed_graph) {
        return Err("Some node neighbor listed without corresponding entry in graph!".to_string());
    }
    Ok(*boxed_graph)
}

#[cfg(test)]
mod load_graph_tests {
    use super::load_graph;
    use std::collections::HashMap;
    use std::io;

    #[test]
    fn builds_graph_from_input() {
        let mut expected: HashMap<String, Vec<String>> = HashMap::new();
        expected.insert("a".to_string(), vec!["b".to_string()]);
        expected.insert("b".to_string(), vec![]);
        let s: &str = "a b\nb";
        if let Ok(g) = load_graph(mk_reader(s)) { assert_eq!(g, expected); }
    }

    #[test]
    fn returns_err_for_duplicate_entry() {
        let s: &str = "a b\nb\na c\nc";
        let expected: String = "Duplicate entry: a".to_string();
        let result = load_graph(mk_reader(s));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().as_slice(), expected.as_slice());
    }

    #[test]
    fn returns_err_for_invalid_graph() {
        let s: &str = "a b c\nb";
        let expected: String =
            "Some node neighbor listed without corresponding entry in graph!".to_string();
        let result = load_graph(mk_reader(s));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().as_slice(), expected.as_slice());
    }

    fn mk_reader(s: &str) -> io::BufferedReader<io::MemReader> {
        let bytes = s.to_string().into_bytes();
        io::BufferedReader::new(io::MemReader::new(bytes))
    }

}

/// Validates the given graph by checking that each neighbor mentioned has an entry in the graph.
fn valid_graph(graph: &Graph) -> bool {
    for neighbors in graph.values() {
        if neighbors.len() > 0 {
            for neighbor in neighbors.iter() {
                if !graph.contains_key(neighbor) {
                    println!("Graph missing entry for {}", neighbor);
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod valid_graph_tests {
    use super::valid_graph;
    use std::collections::HashMap;

    #[test]
    fn returns_true_for_valid_graph() {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        graph.insert("a".to_string(), vec!["b".to_string()]);
        graph.insert("b".to_string(), vec![]);
        let boxed_graph: Box<HashMap<String, Vec<String>>> = Box::new(graph);
        assert!(valid_graph(&*boxed_graph));
        let other_graph: Box<HashMap<String, Vec<String>>> = Box::new(example_graph());
        assert!(valid_graph(&*other_graph));
    }

    #[test]
    fn returns_false_for_invalid_graph() {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();
        graph.insert("a".to_string(), vec!["b".to_string(), "c".to_string()]);
        graph.insert("b".to_string(), vec![]);
        let boxed_graph: Box<HashMap<String, Vec<String>>> = Box::new(graph);
        assert!(!valid_graph(&*boxed_graph));
    }

    fn example_graph() -> HashMap<String, Vec<String>> {
        let mut g: HashMap<String, Vec<String>> = HashMap::new();
        g.insert("a".to_string(), vec!["b".to_string(), "d".to_string()]);
        g.insert("b".to_string(), vec!["a".to_string(), "d".to_string()]);
        g.insert("c".to_string(), vec![]);
        g.insert("d".to_string(), vec!["c".to_string()]);
        g
    }
}

/// Attempts to find a path in the given graph from the starting position to the end position via
/// depth-first search. If a path is found, `Some(String)` containing the path is returned.
/// Otherwise, `None` is returned.
fn find_path(graph: &Graph, start: String, end: String) -> Option<String> {
    let mut current: String = start;
    let mut todo: Vec<String> = vec![];
    let mut visited: Vec<String> = vec![];
    loop {
        if current.as_slice() == end.as_slice() {
            visited.push(current);
            break;
        }
        let neighbors = match graph.get(&current) {
            Some(n) => n,
            None => {
                return None;
            }
        };
        for neighbor in neighbors.iter() {
            if visited.as_slice().contains(neighbor) {
                continue;
            }
            todo.push(neighbor.clone());
        }
        if !visited.as_slice().contains(&current) {
            visited.push(current); 
        }
        if todo.len() == 0 { return None; }
        current = todo.remove(0);
    }
    let path_string: String = visited.as_slice().connect(" ");
    Some(path_string)
}

#[cfg(test)]
mod find_path_tests {
    use super::find_path;
    use std::collections::HashMap;

    #[test]
    fn finds_path_from_start_to_end() {
        let graph: Box<HashMap<String, Vec<String>>> = Box::new(example_graph());
        let a_d_result: Option<String> = find_path(&*graph, "a".to_string(), "d".to_string());
        assert!(a_d_result.is_some());
        assert_eq!(a_d_result.unwrap().as_slice(), "a b d");

        let a_b_result: Option<String> = find_path(&*graph, "a".to_string(), "b".to_string());
        assert!(a_b_result.is_some());
        assert_eq!(a_b_result.unwrap().as_slice(), "a b");

        let a_c_result: Option<String> = find_path(&*graph, "a".to_string(), "c".to_string());
        assert!(a_c_result.is_some());
        assert_eq!(a_c_result.unwrap().as_slice(), "a b d c");
    }

    #[test]
    fn returns_none_when_no_path_exists() {
        let graph: Box<HashMap<String, Vec<String>>> = Box::new(example_graph());
        let result: Option<String> = find_path(&*graph, "c".to_string(), "b".to_string());
        assert_eq!(result.is_none(), true);
    }

    fn example_graph() -> HashMap<String, Vec<String>> {
        let mut g: HashMap<String, Vec<String>> = HashMap::new();
        g.insert("a".to_string(), vec!["b".to_string(), "d".to_string()]);
        g.insert("b".to_string(), vec!["a".to_string(), "d".to_string()]);
        g.insert("c".to_string(), vec![]);
        g.insert("d".to_string(), vec!["c".to_string()]);
        g
    }
}
