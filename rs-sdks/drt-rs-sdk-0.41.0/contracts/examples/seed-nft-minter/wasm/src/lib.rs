// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            9
// Async Callback:                       1
// Total number of exported functions:  11

#![no_std]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    seed_nft_minter
    (
        createNft
        claimAndDistribute
        getMarketplaces
        getNftCount
        getDistributionRules
        issueToken
        buyNft
        getNftPrice
        getNftTokenId
        callBack
    )
}
