use numbat_wasm_debug::*;

fn world() -> BlockchainMock {
    let mut blockchain = BlockchainMock::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/proxy-pause");

    blockchain
        .register_contract_builder("file:output/proxy-pause.wasm", proxy_pause::ContractBuilder);

    blockchain.register_contract_builder(
        "file:../../feature-tests/use-module/output/use-module.wasm",
        use_module::ContractBuilder,
    );
    blockchain
}

#[test]
fn pause_rs() {
    numbat_wasm_debug::denali_rs("denali/pause-and-unpause.scen.json", world());
}
