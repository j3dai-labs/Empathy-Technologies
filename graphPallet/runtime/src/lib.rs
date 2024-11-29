// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A minimal runtime that includes the template [`pallet`](`pallet_minimal_template`).


pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::{self as system, pallet_prelude::*};
    use sp_runtime::offchain::{storage, StorageValue};

    // Importiere deinen Off-Chain Worker Pallet
    pub use pallet_dijkstra;
}

#[cfg(test)]
mod tests {
    use frame_support::{assert_ok, assert_noop};
    use frame_system::mocking::*;
    use frame_support::test_helpers::*;
    use super::*;
    use crate::pallet::Pallet as DijkstraPallet;

    // Test für den Off-Chain Worker
    #[test]
    fn test_offchain_worker() {
        TestState::new_empty().execute_with(|| {
            // Setze die Dot-Datei und den Startknoten
            let dot_content = r#"
            "A" -> "B" [label="4"];
            "A" -> "C" [label="2"];
            "B" -> "C" [label="5"];
            "B" -> "D" [label="10"];
            "C" -> "D" [label="3"];
            "#;
            let start_node = "A";

            // Teste den Off-Chain Worker, indem du den Graphen erstellst und die kürzesten Pfade berechnest
            let result = DijkstraPallet::process_graph(dot_content, start_node);

            // Überprüfe die korrekten kürzesten Pfade
            assert_eq!(result["A"], 0);
            assert_eq!(result["B"], 4);
            assert_eq!(result["C"], 2);
            assert_eq!(result["D"], 5);
        })
    }
}


#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
    use frame_system::{pallet_prelude::*};
    use pallet_dijkstra::Pallet as DijkstraPallet;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index = 0]
        fn trigger_dijkstra(
            origin: OriginFor<T>,
            dot_file_data: Vec<u8>,
            start_node: String,
        ) -> DispatchResult {
            DijkstraPallet::<T>::execute_dijkstra_offchain(origin, dot_file_data, start_node)?;
            Ok(())
        }
    }
}

