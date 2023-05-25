#[allow(dead_code)]

/// The chain alias method name
const ALIAS_METHOD: &str = "admin.aliasChain";

/// The request to alias a chain via the admin API.
/// Ref: https://docs.avax.network/apis/avalanchego/apis/admin#adminaliaschain
struct ChainAliasRequest {
    /// Jsonrpc version
    pub jsonrpc: String,
    /// Id of request
    pub id: u32,
    /// Method (admin.aliasChain)
    pub method: String,
    /// Alias parameters
    pub params: Option<ChainAliasParams>,
}

impl Default for ChainAliasRequest {
    fn default() -> Self {
        Self {
            jsonrpc: String::from(super::DEFAULT_VERSION),
            id: super::DEFAULT_ID,
            method: ALIAS_METHOD.to_string(),
            params: None,
        }
    }
}

/// Parameters for the alias request.
struct ChainAliasParams {
    /// The long-form chain ID
    pub chain: String,
    /// The newly issues alias
    pub alias: String,
}

/// Response for the alias request.
struct ChainAliasResponse {
    /// Jsonrpc version
    pub jsonrpc: String,
    /// Id of request
    pub id: u32,
    /// Result (empty)
    pub result: (),
}
