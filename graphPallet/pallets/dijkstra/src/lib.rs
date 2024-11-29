//! A shell pallet built with [`frame`].
//!
//! To get started with this pallet, try implementing the guide in
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::{ pallet_prelude::* };
    use sp_std::{vec::Vec, collections::HashMap};
    use sp_runtime::offchain::{storage, http};
    use sp_io::offchain;
    use regex::Regex;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[derive(Debug, Clone)]
    pub struct Graph {
        adjacency_list: HashMap<String, Vec<(String, i32)>>, // Graph mit Knotennamen als String und Gewicht als i32
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

        pub fn dijkstra(&self, start: &str) -> HashMap<String, i32> {
            use std::collections::{BinaryHeap, HashMap};
            let mut distances: HashMap<String, i32> = HashMap::new();
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
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index = 0]
        fn execute_dijkstra_offchain(
            origin: OriginFor<T>,
            dot_data: Vec<u8>,  // Dot-Datei als Eingabe (binär)
            start_node: String, // Startknoten
        ) -> DispatchResult {
            let graph = Self::parse_dot_file(dot_data);
            let result = graph.dijkstra(&start_node);

            log::info!("Dijkstra Result: {:?}", result);
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn parse_dot_file(dot_data: Vec<u8>) -> Graph {
            let content = sp_std::str::from_utf8(&dot_data).expect("Invalid UTF-8 string");
            let mut graph = Graph::new();
            let re = Regex::new(r#""?([\w\s]+)"?\s*->\s*"?([\w\s]+)"?\s*\[label="?(\d+)"?\]"#).unwrap();
            
            for caps in re.captures_iter(content) {
                let from = caps[1].trim().to_string();
                let to = caps[2].trim().to_string();
                let weight: i32 = caps[3].parse().unwrap();

                graph.add_edge(from, to, weight);
            }

            graph
        }
    }
}


