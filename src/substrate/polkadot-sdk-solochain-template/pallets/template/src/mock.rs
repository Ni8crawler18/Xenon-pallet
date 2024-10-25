//! Mock runtime for pallet testing.
//! This file sets up a mock runtime environment for unit testing the pallet.
#![cfg(test)]

use crate as pallet_template;
use frame_support::{
    parameter_types,
    traits::{ConstU16, ConstU32, ConstU64, ReservableCurrency},
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

/// Mock Block type
type Block = frame_system::mocking::MockBlock;

// Configure a mock runtime to test the pallet
frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        Balances: pallet_balances,
        TemplateModule: pallet_template,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Block = Block;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ConstU64<1>;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = ConstU32<10>;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type FreezeIdentifier = ();
    type MaxHolds = ();
    type MaxFreezes = ();
}

// Define constants for the pallet configuration
parameter_types! {
    pub const MaxLinkedChains: u32 = 10;
    pub const MaxPublicKeys: u32 = 5;
    pub const MaxServices: u32 = 10;
}

impl pallet_template::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type MaxLinkedChains = MaxLinkedChains;
    type MaxPublicKeys = MaxPublicKeys;
    type MaxServices = MaxServices;
    type WeightInfo = ();
}

/// Helper function to build test externalities
/// This function creates a new test environment with pre-configured accounts
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage()
        .unwrap();
    
    // Initialize balances for test accounts
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 100), // Alice
            (2, 100), // Bob
            (3, 100), // Charlie
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    
    t.into()
}

// Helper constants for better test readability
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLIE: u64 = 3;