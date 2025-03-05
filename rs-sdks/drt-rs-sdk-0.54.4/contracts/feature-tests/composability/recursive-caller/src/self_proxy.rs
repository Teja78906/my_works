// Code generated by the dharitri-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use dharitri_sc::proxy_imports::*;

pub struct RecursiveCallerProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for RecursiveCallerProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = RecursiveCallerProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        RecursiveCallerProxyMethods { wrapped_tx: tx }
    }
}

pub struct RecursiveCallerProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> RecursiveCallerProxyMethods<Env, From, (), Gas>
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
impl<Env, From, To, Gas> RecursiveCallerProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn recursive_send_funds<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<RewaOrDcdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<BigUint<Env::Api>>,
        Arg3: ProxyArg<u32>,
    >(
        self,
        to: Arg0,
        token_identifier: Arg1,
        amount: Arg2,
        counter: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("recursive_send_funds")
            .argument(&to)
            .argument(&token_identifier)
            .argument(&amount)
            .argument(&counter)
            .original_result()
    }
}
