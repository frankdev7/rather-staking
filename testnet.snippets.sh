PROJECT="/Users/frank/Documents/blockchain/rust/multiversx/rather-staking"
WALLET_PEM="/Users/frank/multiversx-sdk/testwallets/latest/users/alice.pem"
DEPLOY_OUTFILE="./logs/testnet/testnet-deploy-outfile.interaction.json"
ARGUMENT_U64_REWARD_RATE=1000000000000000
VALUE=2000000000000000000
OWNER_ADDRESS="erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th"
CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqp6a99wvr0eyup3tzgfwlh98l7smh9cxxd8ss6v2xds"
OUTFILE="./logs/testnet/testnet-outfile.interaction.json"

deploySC() {
    mxpy --verbose contract deploy --recall-nonce \
    --project ${PROJECT} \
    --pem ${WALLET_PEM} \
    --gas-limit=60000000 \
    --arguments ${ARGUMENT_U64_REWARD_RATE} \
    --outfile ${DEPLOY_OUTFILE} \
    --send || return
}
# deploySC

stakeEndpoint() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --function="stake" \
        --value ${VALUE} \
        --outfile ${OUTFILE} \
        --wait-result --send || return
}
# stakeEndpoint

unstakeEndpoint() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --function="unstake" \
        --outfile ${OUTFILE} \
        --wait-result --send || return
}
# unstakeEndpoint

claimRewardsEndpoint() {
    mxpy --verbose contract call ${CONTRACT_ADDRESS} --recall-nonce \
        --pem=${WALLET_PEM} \
        --gas-limit=6000000 \
        --function="claimRewards" \
        --outfile ${OUTFILE} \
        --wait-result --send || return
}
claimRewardsEndpoint


##
## VIEW METHODS
##

getBlockchainTimeEndpoint() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
        --function="getBlockchainTime"
}
# getBlockchainTimeEndpoint

getTimeIncrementStakeEndpoint() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
        --function="getTimeIncrement"
}
# getTimeIncrementStakeEndpoint

getTotalStakeEndpoint() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
        --function="getTotalStake"
}
# getTotalStakeEndpoint

getStakingPositionEndpoint() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
        --arguments ${OWNER_ADDRESS} \
        --function="getStakingPosition"
}
# getStakingPositionEndpoint

getLastClaimTimestampEndpoint() {
    mxpy --verbose contract query ${CONTRACT_ADDRESS} \
        --arguments ${OWNER_ADDRESS} \
        --function="getLastClaimTimestamp"
}
# getLastClaimTimestampEndpoint