#![no_std]

numbat_wasm::imports!();

use hex_literal::hex;

static HARDCODED_ADDRESS: [u8; 32] =
    hex!("fefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefefe");

mod pay_me_proxy {
    numbat_wasm::imports!();

    #[numbat_wasm::proxy]
    pub trait PayMe {
        #[payable("REWA")]
        #[endpoint(payMe)]
        fn pay_me(&self, #[payment] payment: Self::BigUint, arg1: i64);

        #[payable("REWA")]
        #[endpoint(payMeWithResult)]
        fn pay_me_with_result(&self, #[payment] payment: Self::BigUint, arg1: i64);
    }
}

mod message_me_proxy {
    numbat_wasm::imports!();

    #[numbat_wasm::proxy]
    pub trait MessageMe {
        #[init]
        #[payable("REWA")]
        fn init(&self, #[payment] payment: Self::BigUint, init_arg: i32);

        #[endpoint(messageMe)]
        fn message_me(&self, arg1: i64, arg2: &Self::BigUint, arg3: Vec<u8>, arg4: &Address);
    }
}

#[numbat_wasm::contract]
pub trait ProxyTestFirst {
    #[proxy]
    fn pay_me_proxy(&self) -> pay_me_proxy::Proxy<Self::SendApi>;

    #[proxy]
    fn message_me_proxy(&self) -> message_me_proxy::Proxy<Self::SendApi>;

    #[storage_get("other_contract")]
    fn get_other_contract(&self) -> Address;

    #[storage_set("other_contract")]
    fn set_other_contract(&self, other_contract: &Address);

    #[storage_set("callback_info")]
    fn set_callback_info(&self, callback_info: i64);

    #[init]
    fn init(&self, other_contract_addr: &Address) {
        self.set_other_contract(other_contract_addr);
    }

    #[payable("REWA")]
    #[endpoint(deploySecondContract)]
    fn deploy_second_contract(
        &self,
        #[payment] payment: Self::BigUint,
        code: BoxedBytes,
    ) -> SCResult<()> {
        let address = self
            .message_me_proxy()
            .init(payment, 123)
            .deploy_contract(&code, CodeMetadata::DEFAULT)
            .ok_or("Deploy failed")?;
        self.set_other_contract(&address);
        Ok(())
    }

    #[payable("REWA")]
    #[endpoint(forwardToOtherContract)]
    fn forward_to_other_contract(
        &self,
        #[payment] payment: Self::BigUint,
    ) -> AsyncCall<Self::SendApi> {
        let other_contract = self.get_other_contract();
        self.pay_me_proxy()
            .contract(other_contract)
            .pay_me(payment, 0x56)
            .async_call()
    }

    #[payable("REWA")]
    #[endpoint(forwardToOtherContractWithCallback)]
    fn forward_to_other_contract_with_callback(
        &self,
        #[payment] payment: Self::BigUint,
    ) -> AsyncCall<Self::SendApi> {
        let other_contract = self.get_other_contract();
        self.pay_me_proxy()
            .contract(other_contract)
            .pay_me_with_result(payment, 0x56)
            .async_call()
            .with_callback(self.callbacks().pay_callback())
    }

    #[endpoint(messageOtherContract)]
    fn message_other_contract(&self) -> AsyncCall<Self::SendApi> {
        let other_contract = self.get_other_contract();
        self.message_me_proxy()
            .contract(other_contract)
            .message_me(
                0x01,
                &Self::BigUint::from(0x02u64),
                [3u8; 3].to_vec(),
                &HARDCODED_ADDRESS.into(),
            )
            .async_call()
    }

    #[endpoint(messageOtherContractWithCallback)]
    fn message_other_contract_with_callback(&self) -> AsyncCall<Self::SendApi> {
        let other_contract = self.get_other_contract();
        self.message_me_proxy()
            .contract(other_contract)
            .message_me(
                0x01,
                &Self::BigUint::from(0x02u64),
                [3u8; 3].to_vec(),
                &HARDCODED_ADDRESS.into(),
            )
            .async_call()
            .with_callback(self.callbacks().message_callback())
    }

    #[callback(payCallback)] // although uncommon, custom callback names are possible
    fn pay_callback(&self, #[call_result] call_result: AsyncCallResult<i64>) {
        match call_result {
            AsyncCallResult::Ok(cb_arg) => {
                self.set_callback_info(cb_arg);
            },
            AsyncCallResult::Err(_) => {},
        }
    }

    #[callback]
    fn message_callback(&self, #[call_result] _call_result: AsyncCallResult<()>) {
        self.set_callback_info(0x5555);
    }
}
