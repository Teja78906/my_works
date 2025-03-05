// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            7
// Async Callback (empty):               1
// Total number of exported functions:   9

#![no_std]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    first_contract
    (
        init => init
        transferToSecondContractFull => transfer_to_second_contract_full
        transferToSecondContractHalf => transfer_to_second_contract_half
        transferToSecondContractRejected => transfer_to_second_contract_rejected
        transferToSecondContractRejectedWithTransferAndExecute => transfer_to_second_contract_rejected_with_transfer_and_execute
        transferToSecondContractFullWithTransferAndExecute => transfer_to_second_contract_full_with_transfer_and_execute
        getdcdtTokenName => get_contract_dcdt_token_identifier
        getSecondContractAddress => get_second_contract_address
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
