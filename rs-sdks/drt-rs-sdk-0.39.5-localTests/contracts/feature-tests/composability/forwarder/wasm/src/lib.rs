// Code generated by the dharitri-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           69
// Async Callback:                       1
// Total number of exported functions:  71

#![no_std]
#![feature(alloc_error_handler, lang_items)]

dharitri_sc_wasm_adapter::allocator!();
dharitri_sc_wasm_adapter::panic_handler!();

dharitri_sc_wasm_adapter::endpoints! {
    forwarder
    (
        send_rewa
        echo_arguments_sync
        echo_arguments_sync_twice
        forward_sync_accept_funds
        forward_sync_accept_funds_with_fees
        forward_sync_accept_funds_then_read
        forward_sync_retrieve_funds
        forward_sync_retrieve_funds_with_accept_func
        accept_funds_func
        forward_sync_accept_funds_multi_transfer
        echo_args_async
        forward_async_accept_funds
        forward_async_accept_funds_half_payment
        forward_async_accept_funds_with_fees
        forward_async_retrieve_funds
        send_funds_twice
        send_async_accept_multi_transfer
        callback_data
        callback_data_at_index
        clear_callback_data
        forward_transf_exec_accept_funds
        forward_transf_execu_accept_funds_with_fees
        forward_transf_exec_accept_funds_twice
        forward_transf_exec_accept_funds_return_values
        transf_exec_multi_accept_funds
        forward_transf_exec_reject_funds_multi_transfer
        transf_exec_multi_reject_funds
        queued_calls
        add_queued_call
        forward_queued_calls
        changeOwnerAddress
        deploy_contract
        deploy_two_contracts
        deploy_vault_from_source
        upgradeVault
        upgrade_vault_from_source
        getFungibleDcdtBalance
        getCurrentNftNonce
        send_dcdt
        send_dcdt_with_fees
        send_dcdt_twice
        send_dcdt_direct_multi_transfer
        issue_fungible_token
        local_mint
        local_burn
        get_dcdt_local_roles
        get_dcdt_token_data
        is_dcdt_frozen
        is_dcdt_paused
        is_dcdt_limited_transfer
        validate_token_identifier
        sft_issue
        get_nft_balance
        buy_nft
        nft_issue
        nft_create
        nft_create_compact
        nft_add_uris
        nft_update_attributes
        nft_decode_complex_attributes
        nft_add_quantity
        nft_burn
        transfer_nft_via_async_call
        transfer_nft_and_execute
        create_and_send
        setLocalRoles
        unsetLocalRoles
        lastIssuedToken
        lastErrorMessage
        callBack
    )
}
