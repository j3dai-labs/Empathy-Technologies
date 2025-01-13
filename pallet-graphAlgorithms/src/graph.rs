// graph.rs: Single source algorithms. Here: Dijkstra

use std::collections::{BinaryHeap, HashMap}
use regex::Regex;


#[derive(Debug, Clone)]

pub struct Graph {
    adjacency_list: HashMap<String, Vec<(String, i32)>>,
}


impl Graph {

    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: String, to: String, weight: i32) {
        self.adjacency_list
            .entry(from)
            .or_insert_with(Vec::new)
            .push((to, weight));
    }

    pub fn parse_dot(&mut self, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        let re = Regex::new(r#"?([\w\s]+)"?\s*->\s*"?([\w\s]+)"?\s*\[label="?(\d+)"?\]"#)?;
        for caps in re.captures_iter(content) {
            let from = caps[1].trim().to_string();
            let to = caps[2].trim().to_string();
            let weight: i32 = caps[3].parse()?;
            self.add_edge(from, to, weight);
        }

        Ok(())
    }

    pub fn dijkstra(&self, start: &str) -> Vec<(String, String, i32)> {
        let mut distances: HashMap<String, i32> = HashMap::new();
        let mut previous_nodes: HashMap<String, Option<String>> = HashMap::new();
        let mut pq = BinaryHeap::new();

        distances.insert(start.to_string(), 0);
        pq.push((0, start.to_string()));

        while let Some((current_distance, current_node)) = pq.pop() {
            if current_distance > *distances.get(&current_node).unwrap_or(&i32::MAX) {
                continue;
            }

            if let Some(neighbors) = self.adjacency_list.get(&current_node) {
                for (neighbor, weight) in neighbors {
                    let new_distance = current_distance + weight;

                    if new_distance < *distances.get(neighbor).unwrap_or(&i32::MAX) {
                        distances.insert(neighbor.clone(), new_distance);
                        previous_nodes.insert(neighbor.clone(), Some(current_node.clone()));
                        pq.push((new_distance, neighbor.clone()));
                    }
                }
            }
        }

        // Prepare results in a Vec
        distances
            .iter()
            .map(|(node, &distance)| {
                let mut path = Vec::new();
                let mut current_node = Some(node.clone());

                while let Some(current) = current_node {
                    path.push(current.clone());
                    current_node = previous_nodes.get(&current).cloned().unwrap_or(None);
                }

                path.reverse();
                (node.clone(), path.join(" -> "), distance)
            })

            .collect()
    }
}
