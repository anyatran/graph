use std::os;
use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::binary_heap::BinaryHeap;
fn main() {
    let file = match os::args().tail().first() {
        Some(arg) => io::File::open(&Path::new(arg)),
        None      => panic!("Must provide a file"),   
    };

    let mut file_buff = io::BufferedReader::new(file);
    let mut graph: Box<HashMap<String, Vec<String>>> = Box::new(load_graph(file_buff)); //////
    print_graph(&*graph);
    io::stdio::print("->");
     for line in io::stdin().lock().lines() {
        //io::stdio::print("->");
        println!("enter");
        let input_line: String = line.unwrap();
        
        let input: Vec<&str> = input_line.as_slice().split(' ').collect();
        
        let path = find_path(&*graph, input[0].to_string(), input[1].trim_matches('\n').to_string());
        println!("{:?}", path);
        io::stdio::print("->");
    }
    
}

fn print_graph(graph: &HashMap<String, Vec<String>>) {
    for (node, neighbors) in graph.iter() {
        let mut neighbor_string: String = String::new();
        for neighbor in neighbors.iter() {
            if neighbor_string.len() > 0 {
                neighbor_string = format!("{}, {}", neighbor_string, neighbor);
            } else {
                neighbor_string = format!("{}", neighbor);
            }
        }
        println!("{}->[{}]", node, neighbor_string);
    }
}


// read the file and load the graph
// TODO: instead of panic!, we could change function to return a
// Result<HashMap<String, Vec<String>>, so on a good load, it returns
// Ok(graph), on bad it returns Err("some error message"). Then in main() we could
// match over loading the graph and handle the error there, allowing us to test
// the error message output.
fn load_graph<R: Reader> (mut content: io::BufferedReader<R>) -> HashMap<String, Vec<String>> {
    let mut graph_result: HashMap<String, Vec<String>> = HashMap::new();
    for line in content.lines() {
        match line {
            Ok(l) => {
                // TODO: could we iterate over this while building graph?
                let mut node: Vec<&str> = l.as_slice().split(' ').collect();
                let node_name: &str = node.iter().nth(0).unwrap().trim_matches('\n');
                let n = node.as_slice().slice_from(1);
                let neighbors: Vec<String> = n.iter().map(|&x| x.trim_matches('\n').to_string()).collect();
                match graph_result.entry(node_name.to_string()) {
                    Vacant(entry) => { entry.insert(neighbors); },
                    Occupied(entry) => panic!("Duplicate entry: {}", node_name),
                }
            },
            Err(_) => println!("Unrecoverable error while reading graph file"),
        };
    }
    graph_result
}

///find a path in a given graph
fn find_path(mut graph: &HashMap<String, Vec<String>>, start: String, end: String) -> Vec<String> {
    println!("in find_path");
    let mut current: String = start;
    let mut todo: Vec<String> = vec![];
    let mut visited: Vec<String> = vec![];
    loop {
        
        println!("current: {}", current);
        println!("todo: {:?}", todo);
        println!("visited: {:?}", visited);
        if (current.as_slice() == end.as_slice()) {
            visited.push(current);
            break;
        }
        let neighbors = graph.get(&current).unwrap();
        for n in neighbors.iter() {
        /////// duplicates
            if (visited.as_slice().contains(n)) {
                continue;
            }
            todo.push(n.clone());
        }
        if (!visited.as_slice().contains(&current)) {
            visited.push(current); 
        }    
        current  = todo.remove(0);
        
    }
    return visited;
}
  








