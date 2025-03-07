use crate::data::{
    account::{Account, AccountResponse},
    account_storage::AccountStorageResponse,
    address::Address,
    dcdt::{DcdtBalance, DcdtBalanceResponse, DcdtRolesResponse},
};
use anyhow::{anyhow, Result};
use std::collections::HashMap;

use super::GatewayProxy;

const ACCOUNT_ENDPOINT: &str = "address/";
const KEYS_ENDPOINT: &str = "/keys/";

impl GatewayProxy {
    // get_account retrieves an account info from the network (nonce, balance)
    pub async fn get_account(&self, address: &Address) -> Result<Account> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str();
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<AccountResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.account),
        }
    }

    // get_account_dcdt_roles retrieves an all dcdt roles of an account from the network
    pub async fn get_account_dcdt_roles(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, Vec<String>>> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str() + "/dcdts/roles";
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<DcdtRolesResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.roles),
        }
    }

    // get_account_dcdt_tokens retrieves an all dcdt token of an account from the network
    pub async fn get_account_dcdt_tokens(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, DcdtBalance>> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str() + "/dcdt";
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<DcdtBalanceResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.dcdts),
        }
    }

    // get_account_dcdt_tokens retrieves an all dcdt token of an account from the network
    pub async fn get_account_storage_keys(
        &self,
        address: &Address,
    ) -> Result<HashMap<String, String>> {
        if !address.is_valid() {
            return Err(anyhow!("invalid address"));
        }

        let endpoint = ACCOUNT_ENDPOINT.to_string() + address.to_string().as_str() + KEYS_ENDPOINT;
        let endpoint = self.get_endpoint(endpoint.as_str());
        let resp = self
            .client
            .get(endpoint)
            .send()
            .await?
            .json::<AccountStorageResponse>()
            .await?;

        match resp.data {
            None => Err(anyhow!("{}", resp.error)),
            Some(b) => Ok(b.pairs),
        }
    }
}
