use dharitri_sdk::{
    blockchain::{CommunicationProxy, DEVNET_GATEWAY},
    data::address::Address,
};

#[tokio::main]
async fn main() {
    let addr = Address::from_bech32_string(
        "drt1932eft30w753xyvme8d49qejgkjc09n5e49w4mwdjtm0neld797spn6u9l",
    )
    .unwrap();

    let blockchain = CommunicationProxy::new(DEVNET_GATEWAY.to_string());
    let account_storage = blockchain.get_account_storage_keys(&addr).await.unwrap();

    println!("Account Storage: {account_storage:#?}");
}
