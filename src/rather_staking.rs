#![no_std]

multiversx_sc::imports!();

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait RatherStakingContract {
    #[init]
    fn init(&self, reward_rate: u64) {
        require!(reward_rate > 0, "Reward rate can't be less or equal to zero");
        self.reward_rate().set(reward_rate);
    }

    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self) {
        let payment_amount = self.call_value().egld_value().clone_value();

        require!(payment_amount > 0, "Must pay more than 0");

        // update total stake
        let stake_amount = self.call_value().egld_value().clone_value();
        self.total_stake().update(|total_stake| *total_stake += stake_amount);

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        self.last_claim_timestamp(&caller).set(&current_timestamp);
        self.staking_position(&caller)
            .update(|current_amount| *current_amount += payment_amount);
        
        self.staked_addresses().insert(caller);
    }

    #[endpoint]
    fn unstake(&self, opt_unstake_amount: OptionalValue<BigUint>) {
        let caller = self.blockchain().get_caller();
        let stake_mapper = self.staking_position(&caller);
        let unstake_amount = match opt_unstake_amount {
            OptionalValue::Some(amt) => amt,
            OptionalValue::None => stake_mapper.get(),
        };
        
        let remaining_stake = stake_mapper.update(|staked_amount| {
            require!(
                unstake_amount > 0 && unstake_amount <= *staked_amount,
                "Invalid unstake amount"
            );
            *staked_amount -= &unstake_amount;
    
            staked_amount.clone()
        });

        if remaining_stake == 0 {
            self.staked_addresses().swap_remove(&caller);
        }

        // update total stake
        self.total_stake().update(|total_stake| *total_stake -= &unstake_amount);
        
        self.send().direct_egld(&caller, &unstake_amount);
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();

        let caller_stake = self.staking_position(&caller).get();
        require!(caller_stake > 0, "Need stake more than 0");

        let caller_rewards: BigUint = self.calculate_rewards(&caller, &caller_stake, &current_timestamp);

        if caller_rewards > BigUint::zero() {
            self.last_claim_timestamp(&caller).set(current_timestamp);
            self.send().direct_egld(&caller, &caller_rewards);
        }
    }

    fn calculate_rewards(&self, caller: &ManagedAddress, caller_stake: &BigUint, current_timestamp: &u64) -> BigUint {
        let last_claim_timestamp: u64 = self.last_claim_timestamp(caller).get();

        let total_stake: BigUint = self.total_stake().get();
        let reward_rate: BigUint = BigUint::from(self.reward_rate().get());
        let time_increment: BigUint = BigUint::from(*current_timestamp-last_claim_timestamp);

        let total_rewards: BigUint  = time_increment.mul(&reward_rate);
        let caller_rewards: BigUint = caller_stake * &total_rewards / &total_stake;
        caller_rewards
    }

    #[view(getStakedAddresses)]
    #[storage_mapper("stakedAddresses")]
    fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getRewardRate)]
    #[storage_mapper("rewardRate")]
    fn reward_rate(&self) -> SingleValueMapper<u64>;

    #[view(getTotalStake)]
    #[storage_mapper("totalStake")]
    fn total_stake(&self) -> SingleValueMapper<BigUint>;
    
    #[view(getLastClaimTimestamp)]
    #[storage_mapper("lastClaimTimestamp")]
    fn last_claim_timestamp(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;
}
