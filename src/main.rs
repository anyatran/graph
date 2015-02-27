//! provides a path query interface to a graph from a file provided on the command line
#![allow(unstable)]

use std::os;
use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn main() {

    let file = match os::args().tail().first() {
        Some(arg) => io::File::open(&Path::new(arg)),
        None      => panic!("Must provide a file"),   
    };

    let file_buff = io::BufferedReader::new(file);
    let graph: Box<HashMap<String, Vec<String>>> = match load_graph(file_buff) {
        Ok(g) => Box::new(g),
        Err(e) => panic!("{}", e),
    };

    io::stdio::print("-> ");
    for line in io::stdin().lock().lines() {

        let input_line: String = line.unwrap();
        let input_split = input_line.as_slice().split(' ');

        if input_split.clone().count() != 2 {
            println!("Must provide only start and end");
            io::stdio::print("-> ");
            continue;
        }

        let input: Vec<&str> = input_split.collect();
        
        let path: String = match find_path(&*graph, input[0].to_string(), input[1].trim_matches('\n').to_string()) {
            Some(p) => p,
            None => { 
                println!("No path from {} to {}", input[0], input[1]); 
                io::stdio::print("-> ");
                continue;
            },  
        };

        println!("{}", path);

        io::stdio::print("-> ");
    }
    
}



/// build a graph from the given buffer. Returns Ok(HashMap<String, Vec<String>>) on success,
/// otherwise returns Err(String) containing an error message.
fn load_graph<'a, R: Reader> (mut content: io::BufferedReader<R>) -> Result<HashMap<String, Vec<String>>, String> {
    let mut graph_result: HashMap<String, Vec<String>> = HashMap::new();
    for line in content.lines() {
        match line {
            Ok(l) => {
                // line format: <node><space><neighbor1><space><neighbor2>...<neighborN>
                let mut node: Vec<&str> = l.as_slice().split(' ').collect();
                let node_name: &str = node.remove(0);
                let neighbors: Vec<String> = node.iter().map(|&x| x.trim_matches('\n').to_string()).collect();
                match graph_result.entry(node_name.to_string()) {
                    Vacant(entry) => { entry.insert(neighbors); },
                    Occupied(_) => {
                        return Err(format!("Duplicate entry: {}", node_name));
                    },
                }
            },
            Err(_) => { return Err("Unrecoverable error whiel reading graph file".to_string()); },
        };
    }
    Ok(graph_result)
}

/// Attempts to find a path in the given graph from the starting position to the end position via
/// depth-first search. If a path is found, `Some(String)` containing the path is returned.
/// Otherwise, `None` is returned.
fn find_path(graph: &HashMap<String, Vec<String>>, start: String, end: String) -> Option<String> {
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
        for n in neighbors.iter() {
            if visited.as_slice().contains(n) {
                continue;
            }
            todo.push(n.clone());
        }
        if !visited.as_slice().contains(&current) {
            visited.push(current); 
        }    
        current  = todo.remove(0);
        
    }
    let path_string: String = visited.as_slice().connect(" ");
    Some(path_string)
}
