pub mod eip1559;
pub mod legacy;

use std::io;

use crate::{client::evm as client_evm, key};

#[derive(Clone, Debug)]
pub struct Evm<T>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
{
    pub inner: crate::client::wallet::Wallet<T>,

    /// Either "C" or subnet_evm chain Id.
    pub chain_id_alias: String,
    pub chain_rpc_url_path: String,
    pub chain_id: primitive_types::U256,
}

impl<T> Evm<T>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
{
    /// Fetches the current balance of the wallet owner from the specified HTTP endpoint.
    pub async fn balance_with_endpoint(&self, http_rpc: &str) -> io::Result<primitive_types::U256> {
        let resp = client_evm::get_balance(http_rpc, &self.chain_id_alias, &self.inner.eth_address)
            .await?;
        let cur_balance = resp.result;
        Ok(cur_balance)
    }

    /// Fetches the current balance of the wallet owner from all endpoints
    /// in the same order of "self.http_rpcs".
    pub async fn balances(&self) -> io::Result<Vec<primitive_types::U256>> {
        let mut balances = Vec::new();
        for http_rpc in self.inner.http_rpcs.iter() {
            let balance = self.balance_with_endpoint(http_rpc).await?;
            balances.push(balance);
        }
        Ok(balances)
    }

    /// Fetches the current balance of the wallet owner.
    pub async fn balance(&self) -> io::Result<primitive_types::U256> {
        self.balance_with_endpoint(&self.inner.pick_http_rpc().1)
            .await
    }

    /// Fetches the current balance of the wallet owner.
    pub async fn latest_nonce(&self) -> io::Result<primitive_types::U256> {
        let resp = client_evm::get_latest_transaction_count(
            &self.inner.pick_http_rpc().1,
            &self.chain_id_alias,
            &self.inner.eth_address,
        )
        .await?;
        Ok(resp.result)
    }

    #[must_use]
    pub fn legacy(&self) -> legacy::Tx<T> {
        legacy::Tx::new(self)
    }

    #[must_use]
    pub fn eip1559(&self) -> eip1559::Tx<T> {
        eip1559::Tx::new(self)
    }
}
