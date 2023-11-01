use super::{mock::*, testing_utils::*};
use crate::*;
use common_primitives::{capacity::StakingType::ProviderBoost, msa::MessageSourceId};
use frame_support::{
	assert_noop, assert_ok,
	traits::{Len, WithdrawReasons},
};

#[test]
fn provider_boost_extrinsic_works() {
	new_test_ext().execute_with(|| {
		let account = 600;
		let target: MessageSourceId = 1;
		let amount = 200;
		let capacity = 1;
		register_provider(target, String::from("Foo"));
		set_era_and_reward_pool_at_block(10,100,0);
		assert_ok!(Capacity::provider_boost(RuntimeOrigin::signed(account), target, amount));

		// Check that StakingAccountLedger is updated.
		let boost_account: BoostingAccountDetails<Test> =
			Capacity::get_boost_details_for(account).unwrap();

		assert_eq!(boost_account.staking_details.total, 200);
		assert_eq!(boost_account.staking_details.active, 200);
		assert_eq!(boost_account.last_rewards_claimed_at, None);

		let expected_history = StakingHistory { reward_era: 10, total_staked: 200 };
		let actual_history = boost_account.boost_history.get(0).unwrap();
		assert_eq!(actual_history, &expected_history);

		let events = staking_events();
		assert_eq!(
			events.first().unwrap(),
			&Event::ProviderBoosted { account, target, amount, capacity }
		);

		assert_eq!(Balances::locks(&account)[0].amount, amount);
		assert_eq!(Balances::locks(&account)[0].reasons, WithdrawReasons::all().into());

		let target_details = Capacity::get_target_for(account, target).unwrap();
		assert_eq!(target_details.amount, amount);
		assert_eq!(target_details.staking_type, ProviderBoost);
	});
}

// for tests of Capacity::increase_stake_and_issue_capacity
// increases stake and issues capacity, then asserts expected amounts.
fn assert_successful_increase_stake_and_issue_boost(
	target: MessageSourceId,
	staked: u64,
	expected_target_token: u64,
	expected_capacity: u64,
) {
	set_era_and_reward_pool_at_block(10, 100, 0);
	let staker = 10_000; // has 10_000 token
	let mut boost_account = BoostingAccountDetails::<Test>::default();

	assert_ok!(Capacity::increase_stake_and_issue_boost(
		&staker,
		&mut boost_account,
		&target,
		&staked,
	));

	assert_eq!(boost_account.staking_details.total, staked);
	assert_eq!(boost_account.staking_details.active, staked);
	assert_eq!(boost_account.staking_details.unlocking.len(), 0);

	assert_capacity_and_target_details(&target, expected_target_token, expected_capacity, &staker);
}

#[test]
fn increase_boost_and_issue_capacity_happy_path() {
	new_test_ext().execute_with(|| {
		assert_successful_increase_stake_and_issue_boost(2, 550, 550, 3);
		assert_successful_increase_stake_and_issue_boost(2, 6666, 7216, 36);
	})
}

#[test]
fn provider_boost_adjusts_reward_pool_total() {
	new_test_ext().execute_with(|| {
		let account = 600;
		let target: MessageSourceId = 1;
		let amount = 500;
		let current_era = 10;
		register_provider(target, String::from("Foo"));
		set_era_and_reward_pool_at_block(current_era, 100, 0);
		assert_ok!(Capacity::provider_boost(RuntimeOrigin::signed(account), target, amount));

		let reward_pool_info = Capacity::get_reward_pool_for_era(current_era).unwrap();
		assert_eq!(reward_pool_info, RewardPoolInfo {
			total_staked_token: 500,
			total_reward_pool: 0,
			unclaimed_balance: 0,
		});
		assert!(true);
	});
}

#[test]
fn reward_pool_for_previous_era_is_updated_at_next_era() {
	new_test_ext().execute_with(|| {
		let current_era = 99u32;
		let current_block = 500;
		let initial_rpi: RewardPoolInfo<BalanceOf<Test>> = RewardPoolInfo {
			total_staked_token: 500,
			total_reward_pool: 0,
			unclaimed_balance: 0,
		};
		StakingRewardPool::<Test>::insert(current_era, initial_rpi);
		CurrentEraInfo::<Test>::set(RewardEraInfo { era_index: current_era, started_at: 500 });
		System::set_block_number(current_block);
		system_run_to_block(509); // era length is 10
		run_to_block(510);

		let expected_rpi: RewardPoolInfo<BalanceOf<Test>> = RewardPoolInfo {
			total_staked_token: 500,
			total_reward_pool: 0,
			unclaimed_balance: 0,
		};
		assert_eq!(Capacity::get_reward_pool_for_era(current_era+1).unwrap(), expected_rpi);
	})
}

// test that stakers cause reward pool to be updated and then the totals are correct when the
// RewardEra rolls over
#[test]
fn reward_pool_boost_and_rollover() {
	new_test_ext().execute_with(|| {
		let current_era = 99u32;
		let current_block = 500u32;
		let initial_total_staked= 500u64;
		set_era_and_reward_pool_at_block(current_era, current_block, initial_total_staked);


		system_run_to_block(504);
		setup_provider(&10_000, &3, &200, ProviderBoost);

		system_run_to_block(505);
		setup_provider(&10_000, &4, &300, ProviderBoost);

		system_run_to_block(508);
		setup_provider(&10_000, &5, &400, ProviderBoost);

		assert_eq!(Capacity::get_reward_pool_for_era(current_era).unwrap(), RewardPoolInfo {
			total_staked_token: 1400,
			total_reward_pool: 0,
			unclaimed_balance: 0,
		});
		run_to_block(510);

		assert_eq!(Capacity::get_reward_pool_for_era(current_era).unwrap(), RewardPoolInfo {
			total_staked_token: 1400,
			total_reward_pool: 140,
			unclaimed_balance: 140,
		});
		assert_eq!(Capacity::get_reward_pool_for_era(current_era+1).unwrap(), RewardPoolInfo {
			total_staked_token: 1400,
			total_reward_pool: 0,
			unclaimed_balance: 0,
		});
	})
}


#[test]
fn calling_stake_on_provider_boost_target_errors() {
	new_test_ext().execute_with(|| {
		let account = 600;
		let target: MessageSourceId = 1;
		let amount = 200;
		register_provider(target, String::from("Bear"));
		set_era_and_reward_pool_at_block(10, 100, 0);

		assert_ok!(Capacity::provider_boost(RuntimeOrigin::signed(account), target, amount));
		assert_noop!(
			Capacity::stake(RuntimeOrigin::signed(account), target, 50),
			Error::<Test>::CannotChangeStakingType
		);
	})
}
#[test]
fn calling_provider_boost_on_staked_target_errors() {
	new_test_ext().execute_with(|| {
		let account = 600;
		let target: MessageSourceId = 1;
		let amount = 200;
		register_provider(target, String::from("Foobear"));
		set_era_and_reward_pool_at_block(10, 100, 0);
		assert_ok!(Capacity::stake(RuntimeOrigin::signed(account), target, amount));
		assert_noop!(
			Capacity::provider_boost(RuntimeOrigin::signed(account), target, 50),
			Error::<Test>::CannotChangeStakingType
		);
	})
}
