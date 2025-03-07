use dharitri_sc::{
    api::ManagedTypeApi,
    types::{BigUint, RewaOrDcdtTokenIdentifier, ManagedAddress},
};

dharitri_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub depositor_address: ManagedAddress<M>,
    pub expiration_round: u64,
    pub token_name: RewaOrDcdtTokenIdentifier<M>,
    pub nonce: u64,
}
