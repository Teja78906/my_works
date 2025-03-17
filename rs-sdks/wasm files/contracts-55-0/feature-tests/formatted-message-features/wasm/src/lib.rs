// Code generated by the dharitri-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           19
// Async Callback (empty):               1
// Total number of exported functions:  21

#![no_std]

dharitri_sc_wasm_adapter::allocator!(static64k);
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    formatted_message_features
    (
        init => init
        static_message => static_message
        dynamic_message => dynamic_message
        dynamic_message_hex => dynamic_message_hex
        dynamic_message_multiple => dynamic_message_multiple
        dynamic_message_ascii => dynamic_message_ascii
        decode_error_message => decode_error_message
        print_message => print_message
        print_message_hex => print_message_hex
        print_message_binary => print_message_binary
        print_message_codec => print_message_codec
        print_message_bytes => print_message_bytes
        print_message_hex_bytes => print_message_hex_bytes
        print_message_binary_bytes => print_message_binary_bytes
        format_message_one_part => format_message_one_part
        format_message_multiple_parts => format_message_multiple_parts
        format_message_big_int => format_message_big_int
        format_message_i64 => format_message_i64
        format_message_managed_buffer => format_message_managed_buffer
        format_message_managed_buffer_hex => format_message_managed_buffer_hex
    )
}

dharitri_sc_wasm_adapter::async_callback_empty! {}
