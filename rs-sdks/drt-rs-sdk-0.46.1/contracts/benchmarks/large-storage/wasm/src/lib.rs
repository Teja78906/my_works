// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            2
// Async Callback (empty):               1
// Total number of exported functions:   4

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    large_storage
    (
        init => init
        saveStructure => save_structure
        savedStructure => structure
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
