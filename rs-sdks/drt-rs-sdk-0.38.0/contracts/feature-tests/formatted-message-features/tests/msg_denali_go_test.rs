#[test]
fn managed_error_message_go() {
    numbat_wasm_debug::denali_go("denali/managed_error_message.scen.json");
}

#[test]
fn sc_format_go() {
    numbat_wasm_debug::denali_go("denali/sc_format.scen.json");
}
