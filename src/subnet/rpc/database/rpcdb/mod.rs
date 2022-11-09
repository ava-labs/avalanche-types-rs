pub mod client;
pub mod server;

use std::{collections::HashMap, io};

use crate::subnet::rpc::database::DatabaseError;
use lazy_static::lazy_static;

lazy_static! {
    static ref ERROR_TO_ERROR_CODE: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        m.insert("database closed", DatabaseError::Closed as u32);
        m.insert("not found", DatabaseError::NotFound as u32);
        m
    };
}

pub fn error_to_error_code(msg: &str) -> io::Result<u32> {
    match ERROR_TO_ERROR_CODE.get(msg) {
        None => Ok(0),
        Some(code) => Ok(*code),
    }
}

#[test]
fn database_errors() {
    assert_eq!(
        *ERROR_TO_ERROR_CODE.get("database closed").unwrap(),
        DatabaseError::Closed as u32
    );
    assert_eq!(
        *ERROR_TO_ERROR_CODE.get("not found").unwrap(),
        DatabaseError::NotFound as u32
    );
    assert!(ERROR_TO_ERROR_CODE.get("ohh snap!").is_none());
}
