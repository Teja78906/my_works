use dharitri_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain.register_contract(
        "drtsc:output/linked-list-repeat.drtsc.json",
        linked_list_repeat::ContractBuilder,
    );
    blockchain
}

#[test]
fn linked_list_repeat_rs() {
    world().run("scenarios/linked_list_repeat.scen.json");
}

#[test]
fn linked_list_repeat_struct_rs() {
    world().run("scenarios/linked_list_repeat_struct.scen.json");
}
