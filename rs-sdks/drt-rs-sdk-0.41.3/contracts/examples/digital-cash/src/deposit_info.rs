use dharitri_sc::{
    api::ManagedTypeApi,
    types::{BigUint, DcdtTokenPayment, ManagedAddress, ManagedVec},
};

dharitri_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct DepositInfo<M: ManagedTypeApi> {
    pub depositor_address: ManagedAddress<M>,
    pub dcdt_funds: ManagedVec<M, DcdtTokenPayment<M>>,
    pub rewa_funds: BigUint<M>,
    pub valability: u64,
    pub expiration_round: u64,
    pub fees: Fee<M>,
}

impl<M> DepositInfo<M>
where
    M: ManagedTypeApi,
{
    pub fn get_num_tokens(&self) -> usize {
        (self.rewa_funds != BigUint::zero()) as usize + self.dcdt_funds.len()
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Default)]
pub struct Fee<M: ManagedTypeApi> {
    pub num_token_to_transfer: usize,
    pub value: BigUint<M>,
}
