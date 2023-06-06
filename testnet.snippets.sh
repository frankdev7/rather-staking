PROJECT="/Users/frank/Documents/blockchain/rust/multiversx/rather-staking"
WALLET_PEM="/Users/frank/multiversx-sdk/testwallets/latest/users/alice.pem"
DEPLOY_OUTFILE="./logs/testnet/testnet-deploy-outfile.interaction.json"
ARGUMENT_U64_REWARD_RATE=1000000000000000
VALUE=2000000000000000000
VALUE_UNSTAKE=1000000000000000000
OWNER_ADDRESS="erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"
CONTRACT_ADDRESS=$(mxpy data load --key=contract-address-testnet)
OUTFILE="./logs/testnet/testnet-outfile.interaction.json"


##
## FUNCTION'S IMPLEMENTATIONS
##

deploySC() {
    mxpy --verbose contract deploy --recall-nonce \
    --project ${PROJECT} \
    --pem ${WALLET_PEM} \
    --gas-limit=60000000 \
    --arguments ${ARGUMENT_U64_REWARD_RATE} \
    --outfile ${DEPLOY_OUTFILE} \
    --send || return

    CONTRACT_ADDRESS=$(mxpy data parse --file="./logs/testnet/testnet-deploy-outfile.interaction.json" --expression="data['contractAddress']")
    mxpy data store --key=contract-address-testnet --value=${CONTRACT_ADDRESS}

    echo ""
    echo "Smart contract address: ${CONTRACT_ADDRESS}"
}

stakeEndpoint() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --function="stake" \
        --value ${VALUE} \
        --outfile ${OUTFILE} \
        --wait-result --send || return
}

unstakeEndpoint() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --function="unstake" \
        --arguments ${VALUE_UNSTAKE} \
        --outfile ${OUTFILE} \
        --wait-result --send || return
}

claimRewardsEndpoint() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --function="claimRewards" \
        --outfile ${OUTFILE} \
        --wait-result --send || return
}


##
## VIEW METHODS
##

getTotalStakeEndpoint() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
        --function="getTotalStake"
}


getStakingPositionEndpoint() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
        --arguments ${OWNER_ADDRESS} \
        --function="getStakingPosition"
}

getLastClaimTimestampEndpoint() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
        --arguments ${OWNER_ADDRESS} \
        --function="getLastClaimTimestamp"
}


##
## FUNCTIONS CALL
##

# deploySC
# stakeEndpoint
# unstakeEndpoint
# claimRewardsEndpoint


## 
## QUERIES
##

# getLastClaimTimestampEndpoint
# getStakingPositionEndpoint
# getTotalStakeEndpoint
