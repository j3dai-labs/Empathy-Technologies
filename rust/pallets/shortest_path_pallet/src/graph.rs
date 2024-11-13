// Datei: pallets/shortest_path_pallet/src/lib.rs

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use graph; // Importiert die Graph-Bibliothek
    use sp_std::vec::Vec;
    use sp_std::collections::btree_map::BTreeMap;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ShortestPathComputed(BTreeMap<Vec<u8>, i32>),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn compute_shortest_path(
            origin: OriginFor<T>,
            start_node: Vec<u8>, // Startknoten
            algorithm: u8 // 0 = Dijkstra, 1 = Breitensuche
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            // Beispielhafter statischer Graph
            let mut adjacency_list = BTreeMap::new();
            adjacency_list.insert(b"A".to_vec(), vec![(b"B".to_vec(), 10), (b"C".to_vec(), 15)]);
            adjacency_list.insert(b"B".to_vec(), vec![(b"D".to_vec(), 12)]);
            adjacency_list.insert(b"C".to_vec(), vec![(b"D".to_vec(), 10)]);
            adjacency_list.insert(b"D".to_vec(), vec![]);

            // Initialisiere den Graphen
            let graph = graph::Graph::new(adjacency_list);

            // Kürzeste Pfade berechnen, je nach gewähltem Algorithmus
            let distances = match algorithm {
                0 => graph.dijkstra(&String::from_utf8(start_node.clone()).unwrap()),
                1 => graph.breadth_first_search(&String::from_utf8(start_node.clone()).unwrap()),
                _ => return Err(Error::<T>::InvalidAlgorithm.into()),
            };

            // Konvertiere die Ergebnisse für Events und schicke sie zurück
            let result: BTreeMap<Vec<u8>, i32> = distances
                .into_iter()
                .map(|(node, dist)| (node.into_bytes(), dist))
                .collect();

            Self::deposit_event(Event::ShortestPathComputed(result));
            Ok(())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidAlgorithm,
    }
}

