```rust
    #[endpoint]
    fn unstake(&self, unstake_amount: BigUint) {
        let caller = self.blockchain().get_caller();
        let remaining_stake = self.staking_position(&caller).update(|staked_amount| {
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

        self.send().direct_egld(&caller, &unstake_amount);
    }


    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
    }
```