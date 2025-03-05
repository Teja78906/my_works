use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "drtsc:output/basic-features.drtsc.json",
        basic_features::ContractBuilder,
    );

    blockchain
}

#[test]
fn is_builtin_function_test() {
    world().run("scenarios/is_builtin_function.scen.json");
}
