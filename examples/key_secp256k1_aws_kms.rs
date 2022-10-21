use std::{thread, time};

use avalanche_types::key;
use aws_manager::{self, kms};

/// cargo run --example key_secp256k1_aws_kms --features="aws_kms"
fn main() {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    macro_rules! ab {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    log::info!("creating AWS KMS resources!");
    let shared_config = ab!(aws_manager::load_config(None)).unwrap();
    let kms_manager = kms::Manager::new(&shared_config);

    let mut key_name = id_manager::time::with_prefix("test");
    key_name.push_str("-cmk");

    let pk = ab!(key::secp256k1::aws_kms::PrivateKey::create(
        kms_manager,
        &key_name
    ))
    .unwrap();

    let digest = [0u8; ring::digest::SHA256_OUTPUT_LEN];
    match ab!(pk.sign_digest(&digest)) {
        Ok(sig) => {
            log::info!(
                "successfully signed with signature output {} bytes",
                sig.to_bytes().len()
            );
        }
        Err(e) => {
            log::warn!("failed to sign, error: {:?}", e);
        }
    }
    ab!(pk.delete()).unwrap();

    thread::sleep(time::Duration::from_secs(5));

    // error should be ignored if it's already scheduled for delete
    ab!(pk.delete()).unwrap();
}
