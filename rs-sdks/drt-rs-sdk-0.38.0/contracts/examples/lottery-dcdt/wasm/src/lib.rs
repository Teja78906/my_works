// Code generated by the numbat-wasm multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Total number of exported functions:   9

#![no_std]

numbat_wasm_node::wasm_endpoints! {
    lottery_dcdt
    (
        start
        createLotteryPool
        buy_ticket
        determine_winner
        status
        getLotteryInfo
        getLotteryWhitelist
    )
}

numbat_wasm_node::wasm_empty_callback! {}
