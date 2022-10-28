pub mod transfer;

use std::io;

use crate::{client::x as client_x, key, txs};

#[derive(Clone, Debug)]
pub struct X<T>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
{
    pub inner: crate::client::wallet::Wallet<T>,
}

impl<T> X<T>
where
    T: key::secp256k1::ReadOnly + key::secp256k1::SignOnly + Clone,
{
    /// Fetches the current balance of the wallet owner from the specified HTTP endpoint.
    pub async fn balance_with_endpoint(&self, http_rpc: &str) -> io::Result<u64> {
        let resp = client_x::get_balance(http_rpc, &self.inner.x_address).await?;
        let cur_balance = resp
            .result
            .expect("unexpected None GetBalanceResult")
            .balance;
        Ok(cur_balance)
    }

    /// Fetches the current balance of the wallet owner from all endpoints
    /// in the same order of "self.http_rpcs".
    pub async fn balances(&self) -> io::Result<Vec<u64>> {
        let mut balances = Vec::new();
        for http_rpc in self.inner.http_rpcs.iter() {
            let balance = self.balance_with_endpoint(http_rpc).await?;
            balances.push(balance);
        }
        Ok(balances)
    }

    /// Fetches the current balance of the wallet owner.
    pub async fn balance(&self) -> io::Result<u64> {
        self.balance_with_endpoint(&self.inner.pick_http_rpc().1)
            .await
    }

    /// Fetches UTXOs for "X" chain.
    /// TODO: cache this like avalanchego
    pub async fn utxos(&self) -> io::Result<Vec<txs::utxo::Utxo>> {
        // ref. https://github.com/ava-labs/avalanchego/blob/v1.7.9/wallet/chain/p/builder.go
        // ref. https://github.com/ava-labs/avalanchego/blob/v1.7.9/vms/platformvm/add_validator_tx.go#L263
        // ref. https://github.com/ava-labs/avalanchego/blob/v1.7.9/vms/platformvm/spend.go#L39 "stake"
        // ref. https://github.com/ava-labs/subnet-cli/blob/6bbe9f4aff353b812822af99c08133af35dbc6bd/client/p.go#L355 "AddValidator"
        // ref. https://github.com/ava-labs/subnet-cli/blob/6bbe9f4aff353b812822af99c08133af35dbc6bd/client/p.go#L614 "stake"
        let resp =
            client_x::get_utxos(&self.inner.pick_http_rpc().1, &self.inner.p_address).await?;
        let utxos = resp
            .result
            .expect("unexpected None GetUtxosResult")
            .utxos
            .expect("unexpected None Utxos");
        Ok(utxos)
    }

    #[must_use]
    pub fn transfer(&self) -> transfer::Tx<T> {
        transfer::Tx::new(self)
    }
}
