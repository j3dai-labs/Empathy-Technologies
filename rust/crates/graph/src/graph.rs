// Datei: crates/graph/src/graph.rs
use std::collections::{BinaryHeap, HashMap};

pub struct Graph {
    pub adjacency_list: HashMap<String, Vec<(String, i32)>>,
}

impl Graph {
    pub fn new(adjacency_list: HashMap<String, Vec<(String, i32)>>) -> Self {
        Graph { adjacency_list }
    }

    // Dijkstra-Algorithmus
    pub fn dijkstra(&self, start: &str) -> HashMap<String, i32> {
        let mut distances = HashMap::new();
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
                        pq.push((new_distance, neighbor.clone()));
                    }
                }
            }
        }
        distances
    }

    // Breitensuche für ungewichtete Graphen
    pub fn breadth_first_search(&self, start: &str) -> HashMap<String, i32> {
        let mut distances = HashMap::new();
        let mut queue = vec![start.to_string()];
        
        distances.insert(start.to_string(), 0);

        while let Some(current_node) = queue.remove(0) {
            if let Some(neighbors) = self.adjacency_list.get(&current_node) {
                for (neighbor, _) in neighbors {
                    if !distances.contains_key(neighbor) {
                        distances.insert(neighbor.clone(), distances[&current_node] + 1);
                        queue.push(neighbor.clone());
                    }
                }
            }
        }
        distances
    }
}

