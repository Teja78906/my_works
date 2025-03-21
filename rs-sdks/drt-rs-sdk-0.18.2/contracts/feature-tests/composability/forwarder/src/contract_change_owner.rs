numbat_wasm::imports!();

#[numbat_wasm::module]
pub trait ChangeOwnerModule {
    #[proxy]
    fn vault_proxy(&self) -> vault::Proxy<Self::SendApi>;

    #[endpoint(changeOwnerAddress)]
    fn change_owner(&self, child_sc_address: Address, new_owner: Address) -> Address {
        self.send()
            .change_owner_address(&child_sc_address, &new_owner);

        self.get_owner_of_vault_contract(child_sc_address)
    }

    fn get_owner_of_vault_contract(&self, address: Address) -> Address {
        self.vault_proxy()
            .contract(address)
            .get_owner_address()
            .execute_on_dest_context()
    }
}
