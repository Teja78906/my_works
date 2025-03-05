// Code generated by the dharitri-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use dharitri_sc::proxy_imports::*;

pub struct RewardsDistributionProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for RewardsDistributionProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = RewardsDistributionProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        RewardsDistributionProxyMethods { wrapped_tx: tx }
    }
}

pub struct RewardsDistributionProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> RewardsDistributionProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init<
        Arg0: ProxyArg<ManagedAddress<Env::Api>>,
        Arg1: ProxyArg<ManagedVec<Env::Api, Bracket>>,
    >(
        self,
        seed_nft_minter_address: Arg0,
        brackets: Arg1,
    ) -> TxTypedDeploy<Env, From, NotPayable, Gas, ()> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_deploy()
            .argument(&seed_nft_minter_address)
            .argument(&brackets)
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> RewardsDistributionProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn deposit_royalties(
        self,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("depositRoyalties")
            .original_result()
    }

    pub fn raffle(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, OperationCompletionStatus> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("raffle")
            .original_result()
    }

    pub fn claim_rewards<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<u64>,
        Arg2: ProxyArg<MultiValueEncoded<Env::Api, MultiValue2<RewaOrDcdtTokenIdentifier<Env::Api>, u64>>>,
    >(
        self,
        raffle_id_start: Arg0,
        raffle_id_end: Arg1,
        reward_tokens: Arg2,
    ) -> TxTypedCall<Env, From, To, (), Gas, ()> {
        self.wrapped_tx
            .raw_call("claimRewards")
            .argument(&raffle_id_start)
            .argument(&raffle_id_end)
            .argument(&reward_tokens)
            .original_result()
    }

    pub fn compute_claimable_amount<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<RewaOrDcdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<u64>,
    >(
        self,
        raffle_id: Arg0,
        reward_token_id: Arg1,
        reward_token_nonce: Arg2,
        nft_nonce: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("computeClaimableAmount")
            .argument(&raffle_id)
            .argument(&reward_token_id)
            .argument(&reward_token_nonce)
            .argument(&nft_nonce)
            .original_result()
    }

    pub fn raffle_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRaffleId")
            .original_result()
    }

    pub fn completed_raffle_id_count(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getCompletedRaffleIdCount")
            .original_result()
    }

    pub fn royalties<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<RewaOrDcdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
    >(
        self,
        raffle_id: Arg0,
        reward_token_id: Arg1,
        reward_token_nonce: Arg2,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getRoyalties")
            .argument(&raffle_id)
            .argument(&reward_token_id)
            .argument(&reward_token_nonce)
            .original_result()
    }

    pub fn nft_reward_percent<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<u64>,
    >(
        self,
        raffle_id: Arg0,
        nft_nonce: Arg1,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, BigUint<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftRewardPercent")
            .argument(&raffle_id)
            .argument(&nft_nonce)
            .original_result()
    }

    pub fn was_claimed<
        Arg0: ProxyArg<u64>,
        Arg1: ProxyArg<RewaOrDcdtTokenIdentifier<Env::Api>>,
        Arg2: ProxyArg<u64>,
        Arg3: ProxyArg<u64>,
    >(
        self,
        raffle_id: Arg0,
        reward_token_id: Arg1,
        reward_token_nonce: Arg2,
        nft_nonce: Arg3,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, bool> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getWasClaimed")
            .argument(&raffle_id)
            .argument(&reward_token_id)
            .argument(&reward_token_nonce)
            .argument(&nft_nonce)
            .original_result()
    }

    pub fn seed_nft_minter_address(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedAddress<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getSeedNftMinterAddress")
            .original_result()
    }

    pub fn brackets(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, ManagedVec<Env::Api, Bracket>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getBrackets")
            .original_result()
    }

    pub fn last_raffle_epoch(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, u64> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getLastRaffleEpoch")
            .original_result()
    }

    pub fn nft_token_id(
        self,
    ) -> TxTypedCall<Env, From, To, NotPayable, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .payment(NotPayable)
            .raw_call("getNftTokenId")
            .original_result()
    }
}

#[type_abi]
#[derive(ManagedVecItem, NestedEncode, NestedDecode)]
pub struct Bracket {
    pub index_percent: u64,
    pub bracket_reward_percent: u64,
}
