// Code generated by the numbat-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           13
// Async Callback:                       1
// Total number of exported functions:  15

#![no_std]

numbat_wasm_node::wasm_endpoints! {
    erc1155
    (
        safeTransferFrom
        safeBatchTransferFrom
        setApprovalForAll
        createToken
        mint
        burn
        balanceOf
        balanceOfBatch
        getTokenOwner
        getTokenTypeCreator
        getTokenTypeUri
        isFungible
        isApprovedForAll
        callBack
    )
}
