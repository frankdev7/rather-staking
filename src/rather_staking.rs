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

        let caller = self.blockchain().get_caller();
        self.staking_position(&caller)
            .update(|current_amount| *current_amount += payment_amount);
        self.staked_addresses().insert(caller);
    }

    #[endpoint]
    fn unstake(&self) {
        let caller = self.blockchain().get_caller();
        let stake_mapper = self.staking_position(&caller);

        let caller_stake = stake_mapper.get();
        require(caller_stake > 0, "Must unstake more than 0");

        self.staked_addresses().swap_remove(&caller);
        stake_mapper.clear();

        self.send().direct_egld(&caller, &caller_stake);
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();

        let stake_mapper = self.staking_position(&caller);
        let caller_stake = stake_mapper.get();

        let total_stake = self.total_stake();

        let reward_rate = self.reward_rate();
        let time_increment = self.time_increment();

        let total_rewards = reward_rate * time_increment;
        let caller_rewards = caller_stake * total_rewards / total_stake;

        self.send().direct_egld(&caller, &caller_rewards);
    }

    fn time_increment(&self) -> u64 {
        let caller = self.blockchain().get_caller();

        let current_timestamp = self.blockchain().get_block_timestamp();
        let last_claim_timestamp = self.last_claim_timestamp(&caller);

        self.last_claim_timestamp().set(current_timestamp);

        current_timestamp - last_claim_timestamp
    }

    #[view(getTotalStake)]
    fn total_stake(&self) -> BigUint {
        let total_stake = self.staked_addresses.iter().map(|address| {
            self.staking_position(&address).get().sum()
        });
        total_stake
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
    
    #[view(getLastClaimTimestamp)]
    #[storage_mapper("getLastClaimTimestamp")]
    fn last_claim_timestamp(&self, addr: &ManagedAddress) -> SingleValueMapper<u64>;
}