mod concurrency;

use std::{io::ErrorKind, time::Duration};

use crate::rpcchainvm::common::*;
use avalanche_types::subnet::rpc::database::{
    corruptabledb::Database as CorruptableDb,
    manager::{versioned_database::VersionedDatabase, DatabaseManager},
    memdb::Database as MemDb,
    rpcdb::{client::DatabaseClient, server::Server as RpcDb},
};
use semver::Version;
use tokio::net::TcpListener;
use tonic::transport::Channel;

#[tokio::test]
async fn rpcdb_mutation_test() {
    let bar_value = "bar".as_bytes().to_vec();
    let baz_value = "baz".as_bytes().to_vec();

    let db = MemDb::new();
    let server = RpcDb::new(db);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        serve_test_database(server, listener).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;

    let client_conn = Channel::builder(format!("http://{}", addr).parse().unwrap())
        .connect()
        .await
        .unwrap();

    let mut client = DatabaseClient::new(client_conn);

    let resp = client.put("foo".as_bytes(), "bar".as_bytes()).await;
    assert!(!resp.is_err());

    let resp = client.get("foo".as_bytes()).await;
    let value = resp.unwrap();
    assert_eq!(value, bar_value.clone());

    let mut client = client.clone();

    let resp = client.put("foo".as_bytes(), "baz".as_bytes()).await;
    assert!(!resp.is_err());

    let resp = client.get("foo".as_bytes()).await;
    let value = resp.unwrap();
    assert_eq!(value, baz_value.clone());

    let resp = client.get("fool".as_bytes()).await;
    assert!(resp.is_err());
    assert_eq!(resp.unwrap_err().kind(), ErrorKind::NotFound);

    let resp = client.close().await;
    assert!(!resp.is_err());

    let resp = client.get("foo".as_bytes()).await;
    assert!(resp.is_err());
    assert!(resp.unwrap_err().to_string().contains("database closed"));
}

#[tokio::test]
async fn corruptibledb_mutation_test() {
    let bar_value = "bar".as_bytes().to_vec();

    let memdb = MemDb::new();
    let mut corruptible = CorruptableDb::new(memdb);

    let resp = corruptible.put("foo".as_bytes(), "bar".as_bytes()).await;
    assert!(!resp.is_err());

    let resp = corruptible.get("foo".as_bytes()).await;
    let value = resp.unwrap();
    assert_eq!(value, bar_value.clone());

    let resp = corruptible.put("foo".as_bytes(), "baz".as_bytes()).await;
    assert!(!resp.is_err());

    let resp = corruptible.has("foo".as_bytes()).await;
    assert!(!resp.is_err());
    assert_eq!(resp.unwrap(), true);

    let resp = corruptible.get("fool".as_bytes()).await;
    assert!(resp.is_err());
    assert_eq!(resp.unwrap_err().kind(), ErrorKind::NotFound);

    let resp = corruptible.close().await;
    assert!(!resp.is_err());

    let resp = corruptible.put("foo".as_bytes(), "baz".as_bytes()).await;
    assert!(resp.is_err());
    assert!(resp.unwrap_err().to_string().contains("database closed"));
}

#[tokio::test]
async fn test_rpcdb_corruptible() {
    let bar_value = "bar".as_bytes().to_vec();

    let memdb = MemDb::new();
    let rpc_server = RpcDb::new(memdb);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        serve_test_database(rpc_server, listener).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;

    let client_conn = Channel::builder(format!("http://{}", addr).parse().unwrap())
        .connect()
        .await
        .unwrap();

    let db = DatabaseClient::new(client_conn);
    let mut client = CorruptableDb::new(db);

    let resp = client.put("foo".as_bytes(), "bar".as_bytes()).await;
    assert!(resp.is_ok());

    let resp = client.get("fool".as_bytes()).await;
    assert!(resp.is_err());
    assert_eq!(resp.unwrap_err().kind(), ErrorKind::NotFound);

    let resp = client.get("foo".as_bytes()).await;
    assert!(!resp.is_err());
    assert_eq!(resp.unwrap(), bar_value);
}

#[tokio::test]
async fn test_db_manager() {
    let memdb = MemDb::new();
    let rpc_server = RpcDb::new(memdb);

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        serve_test_database(rpc_server, listener).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(100)).await;

    let client_conn = Channel::builder(format!("http://{}", addr).parse().unwrap())
        .connect()
        .await
        .unwrap();

    let vdb = VersionedDatabase::new(DatabaseClient::new(client_conn), Version::new(0, 0, 1));

    let mut databases: Vec<VersionedDatabase> = Vec::new();
    databases.push(vdb);

    let manager = DatabaseManager::new_from_databases(databases);
    let current = manager.current().await.unwrap();

    let mut client = current.db;

    let resp = client.put("foo".as_bytes(), "bar".as_bytes()).await;
    assert!(!resp.is_err());

    let resp = client.get("foo".as_bytes()).await;
    assert!(!resp.is_err());

    let resp = manager.close().await;
    assert!(!resp.is_err());

    let resp = client.close().await;
    assert!(resp.unwrap_err().to_string().eq("database closed"));

    let resp = manager.close().await;
    assert!(resp.unwrap_err().to_string().eq("database closed"));

    let resp = client.get("foo".as_bytes()).await;
    assert!(resp.is_err());
    assert!(resp.unwrap_err().to_string().contains("database closed"));
}
