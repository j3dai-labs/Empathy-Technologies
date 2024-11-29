use std::collections::{HashMap, BinaryHeap};

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

    pub fn print(&self) {
        let mut nodes: Vec<&String> = self.adjacency_list.keys().collect();
        nodes.sort();
        for node in nodes {
            if let Some(neighbors) = self.adjacency_list.get(node) {
                let neighbors_str: Vec<String> = neighbors
                    .iter()
                    .map(|(to, weight)| format!("{} (weight={})", to, weight))
                    .collect();
                println!("{} -> {}", node, neighbors_str.join(", "));
            }
        }
    }

    pub fn dijkstra(&self, start: &str) -> (HashMap<String, i32>, HashMap<String, Option<String>>) {
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

        (distances, previous_nodes)
    }
}

