// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            8
// Async Callback (empty):               1
// Total number of exported functions:  10

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    crowdfunding_dcdt
    (
        init => init
        fund => fund
        status => status
        getCurrentFunds => get_current_funds
        claim => claim
        getTarget => target
        getDeadline => deadline
        getDeposit => deposit
        getCrowdfundingTokenIdentifier => cf_token_identifier
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
