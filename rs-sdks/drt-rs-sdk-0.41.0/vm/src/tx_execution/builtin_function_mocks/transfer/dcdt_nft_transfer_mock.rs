use crate::{
    tx_execution::builtin_function_mocks::builtin_func_trait::BuiltinFunctionDcdtTransferInfo,
    tx_mock::{BlockchainUpdate, TxCache, TxInput, TxResult},
};
use dharitri_sc::{api::DCDT_NFT_TRANSFER_FUNC_NAME, codec::TopDecode, types::heap::Address};

use super::{
    super::builtin_func_trait::BuiltinFunction,
    transfer_common::{
        execute_transfer_builtin_func, extract_transfer_info, ParsedTransferBuiltinFunCall,
        RawDcdtTransfer,
    },
};

pub struct DCDTNftTransfer;

impl BuiltinFunction for DCDTNftTransfer {
    fn name(&self) -> &str {
        DCDT_NFT_TRANSFER_FUNC_NAME
    }

    fn extract_dcdt_transfers(&self, tx_input: &TxInput) -> BuiltinFunctionDcdtTransferInfo {
        if let Ok(parsed_tx) = try_parse_input(tx_input) {
            extract_transfer_info(parsed_tx)
        } else {
            BuiltinFunctionDcdtTransferInfo::empty(tx_input)
        }
    }

    fn execute(&self, tx_input: TxInput, tx_cache: TxCache) -> (TxResult, BlockchainUpdate) {
        match try_parse_input(&tx_input) {
            Ok(parsed_tx) => {
                execute_transfer_builtin_func(parsed_tx, self.name(), tx_input, tx_cache)
            },
            Err(message) => {
                let err_result = TxResult::from_vm_error(message);
                (err_result, BlockchainUpdate::empty())
            },
        }
    }
}

fn try_parse_input(tx_input: &TxInput) -> Result<ParsedTransferBuiltinFunCall, &'static str> {
    if tx_input.args.len() < 4 {
        return Err("DCDTNFTTransfer too few arguments");
    }
    if tx_input.to != tx_input.from {
        // TODO: not sure what the real error message would be, certainly not this
        return Err("DCDTNFTTransfer expects that to == from");
    }

    let token_identifier = tx_input.args[0].clone();
    let nonce_bytes = tx_input.args[1].clone();
    let value_bytes = tx_input.args[2].clone();
    let destination = Address::top_decode(tx_input.args[3].as_slice()).unwrap();

    let func_name = tx_input.func_name_from_arg_index(4);
    let args = if tx_input.args.len() > 5 {
        tx_input.args[5..].to_vec()
    } else {
        Vec::new()
    };

    Ok(ParsedTransferBuiltinFunCall {
        destination,
        raw_dcdt_transfers: vec![RawDcdtTransfer {
            token_identifier,
            nonce_bytes,
            value_bytes,
        }],
        func_name,
        args,
    })
}
