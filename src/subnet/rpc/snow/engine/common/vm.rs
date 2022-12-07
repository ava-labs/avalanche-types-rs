use std::{collections::HashMap, io::Result};

use crate::{
    ids,
    subnet::rpc::{
        context::Context,
        database::manager::Manager,
        health::Checkable,
        snow::engine::common::{
            appsender::AppSender, engine::AppHandler, http_handler::HttpHandler, message::Message,
        },
        snow::State,
    },
};
use tokio::sync::mpsc::Sender;

// TODO: gate definition based on features. Ie state_sync is + StateSync then
// define a matrix which clearly defines each features. This will allow devs to
// customize methods they want to implement on the application level. While
// not needing to reason about rpcchaimvm impl.

/// Vm describes the interface that all consensus VMs must implement
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/common#Vm
#[tonic::async_trait]
pub trait Vm: AppHandler + Connector + Checkable {
    async fn initialize(
        &mut self,
        ctx: Option<Context>,
        db_manager: Box<dyn Manager + Send + Sync>,
        genesis_bytes: &[u8],
        upgrade_bytes: &[u8],
        config_bytes: &[u8],
        to_engine: Sender<Message>,
        fxs: &[Fx],
        app_sender: Box<dyn AppSender + Send + Sync>,
    ) -> Result<()>;
    async fn set_state(&self, state: State) -> Result<()>;
    async fn shutdown(&self) -> Result<()>;
    async fn version(&self) -> Result<String>;
    async fn create_static_handlers(&mut self) -> Result<HashMap<String, HttpHandler>>;
    async fn create_handlers(&mut self) -> Result<HashMap<String, HttpHandler>>;
}

// TODO: new location?
/// snow.validators.Connector
/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/validators#Connector
#[tonic::async_trait]
pub trait Connector {
    async fn connected(&self, id: &ids::node::Id) -> Result<()>;
    async fn disconnected(&self, id: &ids::node::Id) -> Result<()>;
}

/// TODO: Currently not implemented
pub type Fx = ();
