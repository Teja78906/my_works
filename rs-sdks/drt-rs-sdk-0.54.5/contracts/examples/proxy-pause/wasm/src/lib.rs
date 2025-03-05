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
    proxy_pause
    (
        init => init
        addContracts => add_contracts
        removeContracts => remove_contracts
        addOwners => add_owners
        removeOwners => remove_owners
        pause => pause
        unpause => unpause
        owners => owners
        contracts => contracts
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
