// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           18
// Async Callback (empty):               1
// Total number of exported functions:  20

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    payable_features
    (
        init => init
        echo_call_value_legacy => echo_call_value_legacy
        echo_call_value => echo_call_value
        payment_multiple => payment_multiple
        payable_all_transfers => payable_all_transfers
        payment_array_dcdt_3 => payment_array_dcdt_3
        payment_array_rewa_dcdt_3 => payment_array_rewa_dcdt_3
        payable_any_1 => payable_any_1
        payable_any_2 => payable_any_2
        payable_any_3 => payable_any_3
        payable_any_4 => payable_any_4
        payable_rewa_1 => payable_rewa_1
        payable_rewa_2 => payable_rewa_2
        payable_rewa_3 => payable_rewa_3
        payable_rewa_4 => payable_rewa_4
        payable_token_1 => payable_token_1
        payable_token_2 => payable_token_2
        payable_token_3 => payable_token_3
        payable_token_4 => payable_token_4
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
