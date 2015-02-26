use std::os;
use std::io;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

fn main() {
    let file = match os::args().tail().first() {
        Some(arg) => io::File::open(&Path::new(arg)),
        None      => panic!("Must provide a file"),   
    };

    let mut file_buff = io::BufferedReader::new(file);
    let mut graph: HashMap<String, Vec<String>> = load_graph(file_buff); //////
/*    for line in io::stdin().lock().lines() {
        let input = line.unwrap().as_slice().split(' ');
        let path = find_path(&input[0], &input[1]);
        println!("{}", path);
    }
*/  for (node, neighbors) in graph.iter() {
        println!("{}, {:?}", node, neighbors);  
    }  
}


// read the file and load the graph
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
