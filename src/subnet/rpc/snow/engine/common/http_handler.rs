use std::convert::{TryFrom, TryInto};

use jsonrpc_core::IoHandler;

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/common#LockOption
#[derive(Debug)]
pub enum LockOptions {
    WriteLock = 0,
    ReadLock,
    NoLock,
}

impl TryFrom<u8> for LockOptions {
    type Error = ();

    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            x if x == LockOptions::WriteLock as u8 => Ok(LockOptions::WriteLock),
            x if x == LockOptions::ReadLock as u8 => Ok(LockOptions::ReadLock),
            x if x == LockOptions::NoLock as u8 => Ok(LockOptions::NoLock),
            _ => Err(()),
        }
    }
}

impl TryFrom<u32> for LockOptions {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == LockOptions::WriteLock as u32 => Ok(LockOptions::WriteLock),
            x if x == LockOptions::ReadLock as u32 => Ok(LockOptions::ReadLock),
            x if x == LockOptions::NoLock as u32 => Ok(LockOptions::NoLock),
            _ => Err(()),
        }
    }
}

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/snow/engine/common#HTTPHandler
#[derive(Debug)]
pub struct HttpHandler {
    pub lock_option: LockOptions,
    pub handler: Option<IoHandler>,
    pub server_addr: Option<String>,
}

impl HttpHandler {
    pub fn new_from_u8(lock_option: u8, handler: IoHandler) -> Result<HttpHandler, &'static str> {
        let lock_option: LockOptions = match lock_option.try_into() {
            Ok(LockOptions::WriteLock) => Ok(LockOptions::WriteLock),
            Ok(LockOptions::ReadLock) => Ok(LockOptions::ReadLock),
            Ok(LockOptions::NoLock) => Ok(LockOptions::NoLock),
            _ => Err("Invalid lock option"),
        }?;

        Ok(HttpHandler {
            lock_option,
            handler: Some(handler),
            server_addr: None,
        })
    }
}

pub fn test_new_from_u8_http_handler() {
    HttpHandler::new_from_u8(2, IoHandler::new()).unwrap();
}

#[test]
#[should_panic]
pub fn fail_new_u8_http_handler() {
    HttpHandler::new_from_u8(4, IoHandler::new()).unwrap();
}
