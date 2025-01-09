// lib.rs: Main file of the offchain worker pallet

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch, pallet_prelude::*, sp_std::vec::Vec};
use frame_system::{pallet_prelude::*, offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer}};
use sp_runtime::offchain::http;
use crate::graph::Graph;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    }

    #[pallet::storage]
    #[pallet::getter(fn shortest_paths)]
    pub(super) type ShortestPaths<T: Config> = StorageValue<_, Vec<(Vec<u8>, Vec<u8>, i32)>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ShortestPathCalculated,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn submit_graph(origin: OriginFor<T>, file_url: Vec<u8>, start_node: Vec<u8>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            log::info!("Graph submission received from {:?}", who);

            // Save data in an offchain storage
            let _ = Self::process_graph(file_url, start_node);
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn process_graph(file_url: Vec<u8>, start_node: Vec<u8>) -> dispatch::DispatchResult {
            let url = String::from_utf8(file_url).map_err(|_| "Invalid URL")?;
            let start = String::from_utf8(start_node).map_err(|_| "Invalid start node")?;

            // HTTP-request for the graph
            let response = http::Request::get(&url).send().map_err(|_| "HTTP Request failed")?;

            if response.code != 200 {
                log::error!("HTTP Response failed: {:?}", response.code);
                return Err(dispatch::DispatchError::Other("Failed to fetch .dot file"));
            }

            let body = response.body().collect::<Vec<u8>>();
            let graph_data = String::from_utf8(body).map_err(|_| "Invalid response data")?;

            // Create the graph
            let mut graph = Graph::new();
            graph.parse_dot(&graph_data)?;

            // Determine shortest paths beginning with the given start node
            let shortest_paths = graph.dijkstra(&start);
            log::info!("Shortest paths: {:?}", shortest_paths);

            ShortestPaths::<T>::put(shortest_paths);
            Self::deposit_event(Event::<T>::ShortestPathCalculated);
            Ok(())
        }
    }
}
