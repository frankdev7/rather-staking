PROJECT="/Users/frank/Documents/blockchain/rust/multiversx/rather-staking"
WALLET_PEM="/Users/frank/multiversx-sdk/testwallets/latest/users/alice.pem"
DEPLOY_OUTFILE="./logs/testnet/testnet-deploy-outfile.interaction.json"
ARGUMENT_U64_REWARD_RATE=1000000000000000
OWNER_ADDRESS="erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6te"


deploySC() {
    mxpy --verbose contract deploy --recall-nonce \
    --project ${PROJECT} \
    --pem ${WALLET_PEM} \
    --gas-limit=60000000 \
    --arguments ${ARGUMENT_U64_REWARD_RATE} \
    --outfile ${DEPLOY_OUTFILE} \
    --send || return
}
deploySC


CONTRACT_ADDRESS="erd1qqqqqqqqqqqqqpgqgnap0yusjqmdp5p7xr0y25dawwu2ccqyd8ss72ck5a"
OUTFILE="./logs/testnet/testnet-outfile.interaction.json"


VALUE=1000000000000000000
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


# depositEndpoint() {
#     mxpy --verbose contract query ${CONTRACT_ADDRESS} \
#         --function="getDeposit" \
#         --arguments ${OWNER_ADDRESS}
# }
# # depositEndpoint


# deadlineEndpoint() {
#     mxpy --verbose contract query ${CONTRACT_ADDRESS} \
#         --function="getDeadline"
# }
# deadlineEndpoint