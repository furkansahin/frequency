use super::mock::*;
use frame_support::{assert_ok, traits::Hooks};

use common_primitives::capacity::{StakingType, StakingType::MaximumCapacity};
#[allow(unused)]
use sp_runtime::traits::SignedExtension;

use crate::*;
use common_primitives::msa::MessageSourceId;

pub fn staking_events() -> Vec<Event<Test>> {
	let result = System::events()
		.into_iter()
		.map(|r| r.event)
		.filter_map(|e| if let RuntimeEvent::Capacity(inner) = e { Some(inner) } else { None })
		.collect::<Vec<_>>();

	System::reset_events();
	result
}

pub fn run_to_block(n: u32) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Capacity::on_initialize(System::block_number());
	}
}

// Remove capacity on_initialize, needed to emulate pre-existing block height
pub fn system_run_to_block(n: u32) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
	}
}

pub fn register_provider(target_id: MessageSourceId, name: String) {
	let name = Vec::from(name).try_into().expect("error");
	assert_ok!(Msa::create_registered_provider(target_id.into(), name));
}

/// Create Capacity account and set remaining and available amounts.
pub fn create_capacity_account_and_fund(
	target_msa_id: MessageSourceId,
	remaining: BalanceOf<Test>,
	available: BalanceOf<Test>,
	last_replenished: <Test as Config>::EpochNumber,
) -> CapacityDetails<BalanceOf<Test>, <Test as Config>::EpochNumber> {
	let mut capacity_details =
		CapacityDetails::<BalanceOf<Test>, <Test as Config>::EpochNumber>::default();

	capacity_details.remaining_capacity = remaining;
	capacity_details.total_tokens_staked = available;
	capacity_details.total_capacity_issued = available;
	capacity_details.last_replenished_epoch = last_replenished;

	Capacity::set_capacity_for(&target_msa_id, &capacity_details);

	capacity_details
}

pub fn setup_provider(
	staker: &u64,
	target: &MessageSourceId,
	amount: &u64,
	staking_type: StakingType,
) {
	let provider_name = String::from("Cst-") + target.to_string().as_str();
	register_provider(*target, provider_name);
	if amount.gt(&0u64) {
		if staking_type == MaximumCapacity {
			assert_ok!(Capacity::stake(RuntimeOrigin::signed(staker.clone()), *target, *amount,));
		} else {
			assert_ok!(Capacity::provider_boost(
				RuntimeOrigin::signed(staker.clone()),
				*target,
				*amount
			));
		}
		let target = Capacity::get_target_for(staker, target).unwrap();
		assert_eq!(target.amount, *amount);
		assert_eq!(target.staking_type, staking_type);
	}
}


pub fn assert_capacity_and_target_details(
	target: &MessageSourceId,
	expected_target_amount: u64,
	expected_capacity: u64,
	staker: &u64,
) {
	let capacity_details = Capacity::get_capacity_for(&target).unwrap();

	assert_eq!(capacity_details.remaining_capacity, expected_capacity);
	assert_eq!(capacity_details.total_capacity_issued, expected_capacity);
	assert_eq!(capacity_details.last_replenished_epoch, 0);

	let target_details = Capacity::get_target_for(&staker, &target).unwrap();

	assert_eq!(target_details.amount, expected_target_amount);
	assert_eq!(target_details.capacity, expected_capacity);
}

pub fn set_era_and_reward_pool_at_block(era: u32, block: u32, amount: u64) {
	let rpi: RewardPoolInfo<BalanceOf<Test>> = RewardPoolInfo {
		total_staked_token: amount,
		total_reward_pool: 0,
		unclaimed_balance: 0,
	};
	StakingRewardPool::<Test>::insert(era, rpi);
	CurrentEraInfo::<Test>::set(RewardEraInfo { era_index: era, started_at: block });
	System::set_block_number(block);
}