use super::*;
use crate as pallet_capacity;

use common_primitives::{
	node::AccountId,
	schema::{SchemaId, SchemaValidator},
};
use frame_support::{
	parameter_types,
	traits::{ConstU16, ConstU32, ConstU64},
};
use sp_core::{ConstU8, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, Convert, IdentityLookup},
	AccountId32,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Msa: pallet_msa::{Pallet, Call, Storage, Event<T>},
		Capacity: pallet_capacity::{Pallet, Call, Storage, Event<T>},
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
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
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type MaxLocks = ConstU32<10>;
	type Balance = u64;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const MaxSchemaGrantsPerDelegation: u32 = 30;
}
impl Clone for MaxSchemaGrantsPerDelegation {
	fn clone(&self) -> Self {
		MaxSchemaGrantsPerDelegation {}
	}
}

impl Eq for MaxSchemaGrantsPerDelegation {
	fn assert_receiver_is_total_eq(&self) -> () {}
}

impl PartialEq for MaxSchemaGrantsPerDelegation {
	fn eq(&self, _other: &Self) -> bool {
		true
	}
}

impl sp_std::fmt::Debug for MaxSchemaGrantsPerDelegation {
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}
pub struct TestAccountId;

impl Convert<u64, AccountId> for TestAccountId {
	fn convert(_x: u64) -> AccountId32 {
		AccountId32::new([1u8; 32])
	}
}
pub struct Schemas;
impl SchemaValidator<SchemaId> for Schemas {
	fn are_all_schema_ids_valid(_schema_id: &Vec<SchemaId>) -> bool {
		true
	}

	fn set_schema_count(_n: SchemaId) {}
}

impl pallet_msa::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type ConvertIntoAccountId32 = TestAccountId;
	type MaxPublicKeysPerMsa = ConstU8<255>;
	type MaxSchemaGrantsPerDelegation = MaxSchemaGrantsPerDelegation;
	type MaxProviderNameSize = ConstU32<16>;
	type SchemaValidator = Schemas;
	type MortalityWindowSize = ConstU32<100>;
	type MaxSignaturesPerBucket = ConstU32<4000>;
	type NumberOfBuckets = ConstU32<2>;
	/// This MUST ALWAYS be MaxSignaturesPerBucket * NumberOfBuckets.
	type MaxSignaturesStored = ConstU32<8000>;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Currency = pallet_balances::Pallet<Self>;
	type TargetValidator = Msa;
	type MinimumStakingAmount = ConstU64<5>;
	type MaxUnlockingChunks = ConstU32<4>;

	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = Msa;

	type UnstakingThawPeriod = ConstU16<2>;
	type MaxEpochLength = ConstU64<10>;
	type EpochNumber = u32;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(100, 10), (200, 20), (300, 30), (400, 40), (500, 50), (600, 60)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}