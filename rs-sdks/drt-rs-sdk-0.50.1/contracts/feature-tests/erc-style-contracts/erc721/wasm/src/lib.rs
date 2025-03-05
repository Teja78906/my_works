// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    erc721
    (
        init => init
        mint => mint
        approve => approve
        revoke => revoke
        transfer => transfer
        totalMinted => total_minted
        tokenOwner => token_owner
        tokenCount => token_count
        approval => approval
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
