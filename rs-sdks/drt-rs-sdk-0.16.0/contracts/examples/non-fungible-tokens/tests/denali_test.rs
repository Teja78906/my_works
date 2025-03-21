use numbat_wasm::*;
use numbat_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
	let mut contract_map = ContractMap::new();
	contract_map.register_contract(
		"file:../output/non-fungible-tokens.wasm",
		Box::new(|context| Box::new(non_fungible_tokens::contract_obj(context))),
	);
	contract_map
}

#[test]
fn nft_init() {
	parse_execute_denali("denali/nft-init.scen.json", &contract_map());
}

#[test]
fn mint_more_tokens_receiver_owner() {
	parse_execute_denali(
		"denali/nft-mint-more-tokens-receiver-owner.scen.json",
		&contract_map(),
	);
}

#[test]
fn mint_more_tokens_receiver_acc1() {
	parse_execute_denali(
		"denali/nft-mint-more-tokens-receiver-acc1.scen.json",
		&contract_map(),
	);
}
#[test]
fn mint_more_tokens_caller_not_owner() {
	parse_execute_denali(
		"denali/nft-mint-more-tokens-caller-not-owner.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_token_ok() {
	parse_execute_denali("denali/nft-transfer-token-ok.scen.json", &contract_map());
}

#[test]
fn transfer_token_steal() {
	parse_execute_denali(
		"denali/nft-transfer-token-not-owner-no-approval-to-caller.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_token_without_approval() {
	parse_execute_denali(
		"denali/nft-transfer-token-not-owner-no-approval-to-other.scen.json",
		&contract_map(),
	);
}

#[test]
fn approve_ok() {
	parse_execute_denali("denali/nft-approve-ok.scen.json", &contract_map());
}

#[test]
fn approve_non_owned_token() {
	parse_execute_denali(
		"denali/nft-approve-non-owned-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn approve_non_existent_token() {
	parse_execute_denali(
		"denali/nft-approve-non-existent-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn revoke_ok() {
	parse_execute_denali("denali/nft-revoke-ok.scen.json", &contract_map());
}

#[test]
fn revoke_non_approved() {
	parse_execute_denali("denali/nft-revoke-non-approved.scen.json", &contract_map());
}

#[test]
fn transfer_ok() {
	parse_execute_denali("denali/nft-transfer-ok.scen.json", &contract_map());
}

#[test]
fn transfer_non_existent_token() {
	parse_execute_denali(
		"denali/nft-transfer-non-existent-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_non_owned_without_approval() {
	parse_execute_denali(
		"denali/nft-transfer-not-owned-not-approved-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_approved_token() {
	parse_execute_denali(
		"denali/nft-transfer-approved-token.scen.json",
		&contract_map(),
	);
}

#[test]
fn transfer_after_revoked() {
	parse_execute_denali(
		"denali/nft-transfer-token-after-revoked.scen.json",
		&contract_map(),
	);
}
