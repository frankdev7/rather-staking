PROJECT="/Users/frank/Documents/blockchain/rust/multiversx/rather-staking"
WALLET_PEM="/Users/frank/multiversx-sdk/testwallets/latest/users/carol.pem"
DEPLOY_OUTFILE="./logs/testnet/testnet-deploy-outfile.interaction.json"
ARGUMENT_U64_REWARD_RATE=10000000000000000
VALUE=3000000000000000000
VALUE_UNSTAKE=1000000000000000000
OWNER_ADDRESS="erd1k2s324ww2g0yj38qn2ch2jwctdy8mnfxep94q9arncc6xecg3xaq6mjse8"
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

getLastClaimTimestampEndpoint
getStakingPositionEndpoint
getTotalStakeEndpoint


#     {
#         "base64": "ZH587g==",
#         "hex": "647e7cee",
#         "number": 1686011118
#     }
#     {
#         "base64": "KaIkGvYsAAA=",
#         "hex": "29a2241af62c0000",
#         "number": 3000000000000000000
#     }
#     {
#         "base64": "U0RINexYAAA=",
#         "hex": "53444835ec580000",
#         "number": 6000000000000000000
#     }




# [
#     {
#         "base64": "ZH5+XA==",
#         "hex": "647e7e5c",
#         "number": 1686011484
#     }
# ]
# [
#     {
#         "base64": "KaIkGvYsAAA=",
#         "hex": "29a2241af62c0000",
#         "number": 3000000000000000000
#     }
# ]
# [
#     {
#         "base64": "U0RINexYAAA=",
#         "hex": "53444835ec580000",
#         "number": 6000000000000000000
#     }
# ]



# [
#     {
#         "base64": "ZH6AMA==",
#         "hex": "647e8030",
#         "number": 1686011952
#     }
# ]
# [
#     {
#         "base64": "KaIkGvYsAAA=",
#         "hex": "29a2241af62c0000",
#         "number": 3000000000000000000
#     }
# ]
# [
#     {
#         "base64": "poiQa9iwAAA=",
#         "hex": "a688906bd8b00000",
#         "number": 12000000000000000000
#     }
# ]