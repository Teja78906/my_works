USER_PEM=""
USER_ADDRESS_BECH32=
USER_ADDDRESS_HEX="0x$(drtpy wallet bech32 --decode ${USER_ADDRESS_BECH32})"

RECEIVER_ADDRESS_BECH32=
RECEIVER_ADDRESS_HEX="0x$(drtpy wallet bech32 --decode ${RECEIVER_ADDRESS_BECH32})"

TOKEN_ID=""
TOKEN_ID_HEX="0x$(echo -n ${TOKEN_ID} | xxd -p -u | tr -d '\n')"
TOKEN_AMOUNT_PER_TX=100

PROXY="https://testnet-gateway.dharitri.org"
CHAIN_ID="T"

SC_WITH_ROLE_ADDRESS_BECH32=
SC_WITH_ROLE_ADDRESS_HEX="0x$(drtpy wallet bech32 --decode ${SC_WITH_ROLE_ADDRESS_BECH32})"

SC_DEST_ADDRESS_BECH32=
SC_DEST_ADDRESS_HEX="0x$(drtpy wallet bech32 --decode ${SC_DEST_ADDRESS_BECH32})"

DCDT_SYSTEM_SC_ADDRESS=drt1yvesqqqqqqqqqqqqqqqqqqqqyvesqqqqqqqqqqqqqqqqqqqzlllsahl0pc
DCDT_TRANSFER_FUNC_NAME="DCDTTransfer"
DCDT_MULTI_TRANSFER_FUNC_NAME="MultiDCDTNFTTransfer"
TRANSFER_ROLE_NAME_HEX=0x444344545472616E73666572526F6C65

FORWARD_FUNC_NAME_ASCII="forwardPayments"
FORWARD_FUNC_NAME_HEX=0x666F72776172645061796D656E7473
ACCEPT_FUNDS_FUNC_NAME_ASCII="depositTokensForAction"
ACCEPT_FUNDS_FUNC_NAME_HEX=0x6465706F736974546F6B656E73466F72416374696F6E

deployTransferSc() {
    drtpy --verbose contract deploy --project=${PROJECT} \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --send --outfile="deploy-testnet.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

deployVault() {
    drtpy --verbose contract deploy --bytecode="../../vault/output/vault.wasm" \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --send --outfile="deploy-testnet.interaction.json" \
    --proxy=${PROXY} --chain=${CHAIN_ID} || return
}

setSpecialRoleForSc() {
    drtpy --verbose contract call ${DCDT_SYSTEM_SC_ADDRESS} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function="setSpecialRole" \
    --arguments ${TOKEN_ID_HEX} ${SC_WITH_ROLE_ADDRESS_HEX} ${TRANSFER_ROLE_NAME_HEX}
}

transferSingleToUser() {
    drtpy --verbose contract call ${SC_WITH_ROLE_ADDRESS_BECH32} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function=${DCDT_TRANSFER_FUNC_NAME} \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_AMOUNT_PER_TX} \
    ${FORWARD_FUNC_NAME_HEX} ${RECEIVER_ADDRESS_HEX} str:enjoy
}

transferMultipleToUser() {
    drtpy --verbose contract call ${USER_ADDRESS_BECH32} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function=${DCDT_MULTI_TRANSFER_FUNC_NAME} \
    --arguments ${SC_WITH_ROLE_ADDRESS_HEX} 2 \
    ${TOKEN_ID_HEX} 0 ${TOKEN_AMOUNT_PER_TX} \
    ${TOKEN_ID_HEX} 0 ${TOKEN_AMOUNT_PER_TX} \
    ${FORWARD_FUNC_NAME_HEX} ${RECEIVER_ADDRESS_HEX} str:enjoy
}

transferSingleToSmartContractSuccess() {
    drtpy --verbose contract call ${SC_WITH_ROLE_ADDRESS_BECH32} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function=${DCDT_TRANSFER_FUNC_NAME} \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_AMOUNT_PER_TX} \
    ${FORWARD_FUNC_NAME_HEX} ${SC_DEST_ADDRESS_HEX} \
    ${ACCEPT_FUNDS_FUNC_NAME_HEX}
}

transferSingleToSmartContractFail() {
    drtpy --verbose contract call ${SC_WITH_ROLE_ADDRESS_BECH32} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function=${DCDT_TRANSFER_FUNC_NAME} \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_AMOUNT_PER_TX} \
    ${FORWARD_FUNC_NAME_HEX} ${SC_DEST_ADDRESS_HEX} \
    ${ACCEPT_FUNDS_FUNC_NAME_HEX} str:evil_argument
}

transferToWalletDirectlyError() {
    drtpy --verbose contract call ${RECEIVER_ADDRESS_BECH32} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function=${DCDT_TRANSFER_FUNC_NAME} \
    --arguments ${TOKEN_ID_HEX} ${TOKEN_AMOUNT_PER_TX} str:enjoy
}

transferMultiToScSuccess() {
    drtpy --verbose contract call ${USER_ADDRESS_BECH32} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function=${DCDT_MULTI_TRANSFER_FUNC_NAME} \
    --arguments ${SC_WITH_ROLE_ADDRESS_HEX} 2 \
    ${TOKEN_ID_HEX} 0 ${TOKEN_AMOUNT_PER_TX} \
    ${TOKEN_ID_HEX} 0 ${TOKEN_AMOUNT_PER_TX} \
    ${FORWARD_FUNC_NAME_HEX} ${SC_DEST_ADDRESS_HEX} ${ACCEPT_FUNDS_FUNC_NAME_HEX}
}

transferMultiToScFail() {
    drtpy --verbose contract call ${USER_ADDRESS_BECH32} \
    --proxy=${PROXY} --chain=${CHAIN_ID} --send \
    --recall-nonce --pem=${USER_PEM} \
    --gas-limit=100000000 \
    --function=${DCDT_MULTI_TRANSFER_FUNC_NAME} \
    --arguments ${SC_WITH_ROLE_ADDRESS_HEX} 2 \
    ${TOKEN_ID_HEX} 0 ${TOKEN_AMOUNT_PER_TX} \
    ${TOKEN_ID_HEX} 0 ${TOKEN_AMOUNT_PER_TX} \
    ${FORWARD_FUNC_NAME_HEX} ${SC_DEST_ADDRESS_HEX} ${ACCEPT_FUNDS_FUNC_NAME_HEX} str:evil_argument
}
