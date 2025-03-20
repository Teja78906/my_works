// Code generated by the numbat-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           20
// Async Callback:                       1
// Total number of exported functions:  22

#![no_std]

numbat_wasm_node::wasm_endpoints! {
    multisig
    (
        deposit
        signed
        sign
        unsign
        discardAction
        getQuorum
        getNumBoardMembers
        getNumProposers
        getActionLastIndex
        proposeAddBoardMember
        proposeAddProposer
        proposeRemoveUser
        proposeChangeQuorum
        proposeTransferExecute
        proposeAsyncCall
        proposeSCDeployFromSource
        proposeSCUpgradeFromSource
        quorumReached
        performAction
        dnsRegister
        callBack
    )
}
