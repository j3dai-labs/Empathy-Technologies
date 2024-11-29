use super::*;
use crate as pallet_dijkstra;
use frame_support::parameter_types;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Dijkstra: pallet_dijkstra::{Pallet, Call, Storage, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type AccountId = u64;
    type Call = Call;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

impl Config for Test {
    type AuthorityId = frame_system::offchain::testing::TestAuthId;
}

pub type DijkstraModule = Pallet<Test>;

