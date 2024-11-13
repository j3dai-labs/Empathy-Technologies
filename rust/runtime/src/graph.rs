// runtime/src/lib.rs
use shortest_path_pallet;

impl shortest_path_pallet::Config for Runtime {
    type Event = Event;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
        ShortestPathPallet: shortest_path_pallet::{Pallet, Call, Storage, Event<T>},
    }
);

