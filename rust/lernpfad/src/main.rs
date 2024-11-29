// regex = "1"
/*
Rust programme that reads in a given .dot file and displays it on the screen. The .dot file contains the adjacency list for a directed, weighted graph. The nodes of the graph can either be given as integer values or as character strings. 
The programme dynamically reads a .dot file, creates the graph and calculates the shortest paths from the given start node using the Dijkstra algorithm.
The start node is entered via the command line and the graph is loaded from the file.
*/

use std::env;
use std::fs;
use std::str::FromStr;
mod graph;  // Direkte Einbindung des graph.rs-Moduls
use graph::Graph;
use regex::Regex;

fn parse_dot_file(file_path: &str) -> Result<Graph, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let mut graph = Graph::new();

    // Aktualisierte Regex
    let re = Regex::new(r#""?([\w\s]+)"?\s*->\s*"?([\w\s]+)"?\s*\[label="?(\d+)"?\]"#)?;
    for caps in re.captures_iter(&content) {
        let from = caps[1].trim().to_string();
        let to = caps[2].trim().to_string();
        let weight: i32 = i32::from_str(&caps[3])?;

        graph.add_edge(from, to, weight);
    }

    Ok(graph)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <dot-file> <start-node>", args[0]);
        std::process::exit(1);
    }

    let dot_file = &args[1];
    let start_node = &args[2];

    let graph = match parse_dot_file(dot_file) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error parsing the .dot file: {}", e);
            std::process::exit(1);
        }
    };

    graph.print();
    graph.print_shortest_paths(start_node);
}

