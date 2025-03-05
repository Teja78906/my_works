// Code generated by the dharitri-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use dharitri_sc::proxy_imports::*;

pub struct ForwarderProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for ForwarderProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = ForwarderProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        ForwarderProxyMethods { wrapped_tx: tx }
    }
}

pub struct ForwarderProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> ForwarderProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> ForwarderProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn send_rewa<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("send_rewa")
            .argument(&to)
            .argument(&amount)
            .original_result()
    }

    pub fn echo_arguments_sync<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        to: Arg0,
        args: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("echo_arguments_sync")
            .argument(&to)
            .argument(&args)
            .original_result()
    }

    pub fn echo_arguments_sync_twice<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        to: Arg0,
        args: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("echo_arguments_sync_twice")
            .argument(&to)
            .argument(&args)
            .original_result()
    }

    pub fn forward_sync_accept_funds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        to: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_sync_accept_funds")
            .argument(&to)
            .original_result()
    }

    pub fn forward_sync_accept_funds_with_fees<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        percentage_fees: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_sync_accept_funds_with_fees")
            .argument(&to)
            .argument(&percentage_fees)
            .original_result()
    }

    pub fn forward_sync_accept_funds_then_read<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        to: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, usize> {
        self.wrapped_tx
            .raw_call("forward_sync_accept_funds_then_read")
            .argument(&to)
            .original_result()
    }

    pub fn forward_sync_retrieve_funds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<RewaOrDcdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token: Arg1,
        token_nonce: Arg2,
        amount: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("forward_sync_retrieve_funds")
            .argument(&to)
            .argument(&token)
            .argument(&token_nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn forward_sync_retrieve_funds_with_accept_func<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token: Arg1,
        amount: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_sync_retrieve_funds_with_accept_func")
            .argument(&to)
            .argument(&token)
            .argument(&amount)
            .original_result()
    }

    pub fn accept_funds_func(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("accept_funds_func")
            .original_result()
    }

    pub fn forward_sync_accept_funds_multi_transfer<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, MultiValue3<TokenIdentifier<Env::Api>, u64, BigUint<Env::Api>>>>,
    >(
        self,
        to: Arg0,
        token_payments: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("forward_sync_accept_funds_multi_transfer")
            .argument(&to)
            .argument(&token_payments)
            .original_result()
    }

    pub fn echo_args_async<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        to: Arg0,
        args: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("echo_args_async")
            .argument(&to)
            .argument(&args)
            .original_result()
    }

    pub fn forward_async_accept_funds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        to: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_async_accept_funds")
            .argument(&to)
            .original_result()
    }

    pub fn forward_async_accept_funds_half_payment<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        to: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_async_accept_funds_half_payment")
            .argument(&to)
            .original_result()
    }

    pub fn forward_async_accept_funds_with_fees<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        percentage_fees: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_async_accept_funds_with_fees")
            .argument(&to)
            .argument(&percentage_fees)
            .original_result()
    }

    pub fn forward_async_retrieve_funds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<RewaOrDcdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token: Arg1,
        token_nonce: Arg2,
        amount: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("forward_async_retrieve_funds")
            .argument(&to)
            .argument(&token)
            .argument(&token_nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn send_funds_twice<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<RewaOrDcdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token_identifier: Arg1,
        amount: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("send_funds_twice")
            .argument(&to)
            .argument(&token_identifier)
            .argument(&amount)
            .original_result()
    }

    pub fn send_async_accept_multi_transfer<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, MultiValue3<TokenIdentifier<Env::Api>, u64, BigUint<Env::Api>>>>,
    >(
        self,
        to: Arg0,
        token_payments: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("send_async_accept_multi_transfer")
            .argument(&to)
            .argument(&token_payments)
            .original_result()
    }

    pub fn callback_data(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, CallbackData<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("callback_data")
            .original_result()
    }

    pub fn callback_data_at_index<
        Arg0: ProxyArg<usize>,
    >(
        self,
        index: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue5<ManagedBuffer<Env::Api>, RewaOrDcdtTokenIdentifier<Env::Api>, u64, BigUint<Env::Api>, MultiValueManagedVec<Env::Api, ManagedBuffer<Env::Api>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("callback_data_at_index")
            .argument(&index)
            .original_result()
    }

    pub fn clear_callback_data(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("clear_callback_data")
            .original_result()
    }

    pub fn forward_transf_exec_accept_funds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        to: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_transf_exec_accept_funds")
            .argument(&to)
            .original_result()
    }

    pub fn forward_transf_execu_accept_funds_with_fees<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        percentage_fees: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_transf_execu_accept_funds_with_fees")
            .argument(&to)
            .argument(&percentage_fees)
            .original_result()
    }

    pub fn forward_transf_exec_accept_funds_twice<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        to: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("forward_transf_exec_accept_funds_twice")
            .argument(&to)
            .original_result()
    }

    /// Test that the default gas provided to the transfer_execute call 
    /// leaves enough in the transaction for finish to happen. 
    pub fn forward_transf_exec_accept_funds_return_values<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        to: Arg0,
    ) -> TxTypedCall<Env, From, To, (), Gas, MultiValue4<u64, u64, BigUint<Env::Api>, RewaOrDcdtTokenIdentifier<Env::Api>>> {
        self.wrapped_tx
            .raw_call("forward_transf_exec_accept_funds_return_values")
            .argument(&to)
            .original_result()
    }

    pub fn transf_exec_multi_accept_funds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, MultiValue3<TokenIdentifier<Env::Api>, u64, BigUint<Env::Api>>>>,
    >(
        self,
        to: Arg0,
        token_payments: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("transf_exec_multi_accept_funds")
            .argument(&to)
            .argument(&token_payments)
            .original_result()
    }

    pub fn forward_transf_exec_reject_funds_multi_transfer<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, MultiValue3<TokenIdentifier<Env::Api>, u64, BigUint<Env::Api>>>>,
    >(
        self,
        to: Arg0,
        token_payments: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("forward_transf_exec_reject_funds_multi_transfer")
            .argument(&to)
            .argument(&token_payments)
            .original_result()
    }

    pub fn transf_exec_multi_reject_funds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, MultiValue3<TokenIdentifier<Env::Api>, u64, BigUint<Env::Api>>>>,
    >(
        self,
        to: Arg0,
        token_payments: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("transf_exec_multi_reject_funds")
            .argument(&to)
            .argument(&token_payments)
            .original_result()
    }

    pub fn change_owner<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
    >(
        self,
        child_sc_address: Arg0,
        new_owner: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("changeOwnerAddress")
            .argument(&child_sc_address)
            .argument(&new_owner)
            .original_result()
    }

    pub fn deploy_contract<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<OptionalValue<ManagedBuffer<Env::Api>>>,
    >(
        self,
        code: Arg0,
        opt_arg: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue2<ManagedAddress<Env::Api>, OptionalValue<ManagedBuffer<Env::Api>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("deploy_contract")
            .argument(&code)
            .argument(&opt_arg)
            .original_result()
    }

    pub fn deploy_two_contracts<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        code: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue2<ManagedAddress<Env::Api>, ManagedAddress<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("deploy_two_contracts")
            .argument(&code)
            .original_result()
    }

    pub fn deploy_vault_from_source<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<OptionalValue<ManagedBuffer<Env::Api>>>,
    >(
        self,
        source_address: Arg0,
        opt_arg: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue2<ManagedAddress<Env::Api>, OptionalValue<ManagedBuffer<Env::Api>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("deploy_vault_from_source")
            .argument(&source_address)
            .argument(&opt_arg)
            .original_result()
    }

    pub fn upgrade_vault<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<OptionalValue<ManagedBuffer<Env::Api>>>,
    >(
        self,
        child_sc_address: Arg0,
        new_code: Arg1,
        opt_arg: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("upgradeVault")
            .argument(&child_sc_address)
            .argument(&new_code)
            .argument(&opt_arg)
            .original_result()
    }

    pub fn upgrade_vault_from_source<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedAddress<Env::Api>>,
        Arg2: ProxyArg<OptionalValue<ManagedBuffer<Env::Api>>>,
    >(
        self,
        child_sc_address: Arg0,
        source_address: Arg1,
        opt_arg: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("upgrade_vault_from_source")
            .argument(&child_sc_address)
            .argument(&source_address)
            .argument(&opt_arg)
            .original_result()
    }

    pub fn get_fungible_dcdt_balance<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getFungibleDcdtBalance")
            .argument(&token_identifier)
            .original_result()
    }

    pub fn get_current_nft_nonce<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getCurrentNftNonce")
            .argument(&token_identifier)
            .original_result()
    }

    pub fn send_dcdt<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token_id: Arg1,
        amount: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("send_dcdt")
            .argument(&to)
            .argument(&token_id)
            .argument(&amount)
            .original_result()
    }

    pub fn send_dcdt_with_fees<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        percentage_fees: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("send_dcdt_with_fees")
            .argument(&to)
            .argument(&percentage_fees)
            .original_result()
    }

    pub fn send_dcdt_twice<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token_id: Arg1,
        amount_first_time: Arg2,
        amount_second_time: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("send_dcdt_twice")
            .argument(&to)
            .argument(&token_id)
            .argument(&amount_first_time)
            .argument(&amount_second_time)
            .original_result()
    }

    pub fn send_dcdt_direct_multi_transfer<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<MultiValueEncoded<Env::Api, MultiValue3<TokenIdentifier<Env::Api>, u64, BigUint<Env::Api>>>>,
    >(
        self,
        to: Arg0,
        token_payments: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("send_dcdt_direct_multi_transfer")
            .argument(&to)
            .argument(&token_payments)
            .original_result()
    }

    pub fn issue_fungible_token<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
        initial_supply: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("issue_fungible_token")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .argument(&initial_supply)
            .original_result()
    }

    pub fn local_mint<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
        amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("local_mint")
            .argument(&token_identifier)
            .argument(&amount)
            .original_result()
    }

    pub fn local_burn<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
        amount: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("local_burn")
            .argument(&token_identifier)
            .argument(&amount)
            .original_result()
    }

    pub fn get_dcdt_local_roles<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("get_dcdt_local_roles")
            .argument(&token_id)
            .original_result()
    }

    pub fn get_dcdt_token_data<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        address: Arg0,
        token_id: Arg1,
        nonce: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, MultiValue9<DcdtTokenType, BigUint<Env::Api>, bool, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, ManagedBuffer<Env::Api>, ManagedAddress<Env::Api>, BigUint<Env::Api>, ManagedVec<Env::Api, ManagedBuffer<Env::Api>>>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("get_dcdt_token_data")
            .argument(&address)
            .argument(&token_id)
            .argument(&nonce)
            .original_result()
    }

    pub fn is_dcdt_frozen<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        address: Arg0,
        token_id: Arg1,
        nonce: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("is_dcdt_frozen")
            .argument(&address)
            .argument(&token_id)
            .argument(&nonce)
            .original_result()
    }

    pub fn is_dcdt_paused<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("is_dcdt_paused")
            .argument(&token_id)
            .original_result()
    }

    pub fn is_dcdt_limited_transfer<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("is_dcdt_limited_transfer")
            .argument(&token_id)
            .original_result()
    }

    pub fn validate_token_identifier<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
    >(
        self,
        token_id: Arg0,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("validate_token_identifier")
            .argument(&token_id)
            .original_result()
    }

    pub fn sft_issue<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("sft_issue")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .original_result()
    }

    pub fn get_nft_balance<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("get_nft_balance")
            .argument(&token_identifier)
            .argument(&nonce)
            .original_result()
    }

    pub fn buy_nft<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        nft_id: Arg0,
        nft_nonce: Arg1,
        nft_amount: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .raw_call("buy_nft")
            .argument(&nft_id)
            .argument(&nft_nonce)
            .argument(&nft_amount)
            .original_result()
    }

    pub fn nft_issue<
        Arg0: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg1: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("nft_issue")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .original_result()
    }

    pub fn nft_create<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
        Arg4: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg5: ProxyArg<Color>,
        Arg6: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
        amount: Arg1,
        name: Arg2,
        royalties: Arg3,
        hash: Arg4,
        color: Arg5,
        uri: Arg6,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("nft_create")
            .argument(&token_identifier)
            .argument(&amount)
            .argument(&name)
            .argument(&royalties)
            .argument(&hash)
            .argument(&color)
            .argument(&uri)
            .original_result()
    }

    pub fn nft_create_compact<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<Color>,
    >(
        self,
        token_identifier: Arg0,
        amount: Arg1,
        color: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("nft_create_compact")
            .argument(&token_identifier)
            .argument(&amount)
            .argument(&color)
            .original_result()
    }

    pub fn nft_add_uris<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
        uris: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("nft_add_uris")
            .argument(&token_identifier)
            .argument(&nonce)
            .argument(&uris)
            .original_result()
    }

    pub fn nft_update_attributes<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<Color>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
        new_attributes: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("nft_update_attributes")
            .argument(&token_identifier)
            .argument(&nonce)
            .argument(&new_attributes)
            .original_result()
    }

    pub fn nft_decode_complex_attributes<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<BigUint<Env::Api>>,
        Arg2: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
        Arg4: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg5: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg6: ProxyArg<MultiValue5<BigUint<Env::Api>, ManagedBuffer<Env::Api>, TokenIdentifier<Env::Api>, bool, ManagedBuffer<Env::Api>>>,
    >(
        self,
        token_identifier: Arg0,
        amount: Arg1,
        name: Arg2,
        royalties: Arg3,
        hash: Arg4,
        uri: Arg5,
        attrs_arg: Arg6,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("nft_decode_complex_attributes")
            .argument(&token_identifier)
            .argument(&amount)
            .argument(&name)
            .argument(&royalties)
            .argument(&hash)
            .argument(&uri)
            .argument(&attrs_arg)
            .original_result()
    }

    pub fn nft_add_quantity<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
        amount: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("nft_add_quantity")
            .argument(&token_identifier)
            .argument(&nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn nft_burn<
        Arg0: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        token_identifier: Arg0,
        nonce: Arg1,
        amount: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("nft_burn")
            .argument(&token_identifier)
            .argument(&nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn transfer_nft_via_async_call<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
    >(
        self,
        to: Arg0,
        token_identifier: Arg1,
        nonce: Arg2,
        amount: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("transfer_nft_via_async_call")
            .argument(&to)
            .argument(&token_identifier)
            .argument(&nonce)
            .argument(&amount)
            .original_result()
    }

    pub fn transfer_nft_and_execute<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<BigUint<Env::Api>>,
        Arg4: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg5: ProxyArg<MultiValueEncoded<Env::Api, ManagedBuffer<Env::Api>>>,
    >(
        self,
        to: Arg0,
        token_identifier: Arg1,
        nonce: Arg2,
        amount: Arg3,
        function: Arg4,
        arguments: Arg5,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("transfer_nft_and_execute")
            .argument(&to)
            .argument(&token_identifier)
            .argument(&nonce)
            .argument(&amount)
            .argument(&function)
            .argument(&arguments)
            .original_result()
    }

    pub fn create_and_send<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
        Arg3: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg4: ProxyArg<BigUint<Env::Api>>,
        Arg5: ProxyArg<ManagedBuffer<Env::Api>>,
        Arg6: ProxyArg<Color>,
        Arg7: ProxyArg<ManagedBuffer<Env::Api>>,
    >(
        self,
        to: Arg0,
        token_identifier: Arg1,
        amount: Arg2,
        name: Arg3,
        royalties: Arg4,
        hash: Arg5,
        color: Arg6,
        uri: Arg7,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("create_and_send")
            .argument(&to)
            .argument(&token_identifier)
            .argument(&amount)
            .argument(&name)
            .argument(&royalties)
            .argument(&hash)
            .argument(&color)
            .argument(&uri)
            .original_result()
    }

    pub fn set_local_roles<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<MultiValueEncoded<Env::Api, DcdtLocalRole>>,
    >(
        self,
        address: Arg0,
        token_identifier: Arg1,
        roles: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("setLocalRoles")
            .argument(&address)
            .argument(&token_identifier)
            .argument(&roles)
            .original_result()
    }

    pub fn unset_local_roles<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<TokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<MultiValueEncoded<Env::Api, DcdtLocalRole>>,
    >(
        self,
        address: Arg0,
        token_identifier: Arg1,
        roles: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("unsetLocalRoles")
            .argument(&address)
            .argument(&token_identifier)
            .argument(&roles)
            .original_result()
    }

    pub fn last_issued_token(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("lastIssuedToken")
            .original_result()
    }

    pub fn last_error_message(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedBuffer<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("lastErrorMessage")
            .original_result()
    }
}

#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct CallbackData<Api>
where
    Api: ManagedTypeApi,
{
    pub callback_name: ManagedBuffer<Api>,
    pub token_identifier: RewaOrDcdtTokenIdentifier<Api>,
    pub token_nonce: u64,
    pub token_amount: BigUint<Api>,
    pub args: ManagedVec<Api, ManagedBuffer<Api>>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
