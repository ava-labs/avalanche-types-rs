use std::{collections::HashMap, thread, time};

use avalanche_types::key;
use aws_manager::{self, kms};

/// cargo run --example key_secp256k1_kms_aws --features="kms_aws"
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
    let mut tags = HashMap::new();
    tags.insert(String::from("Name"), key_name);

    let cmk = ab!(key::secp256k1::kms::aws::Cmk::create(
        kms_manager.clone(),
        tags,
    ))
    .unwrap();

    let cmk_info = cmk.to_info(1).unwrap();
    println!("cmk_info:\n{}", cmk_info);

    let cmk2 = ab!(key::secp256k1::kms::aws::Cmk::from_arn(
        kms_manager,
        &cmk.arn,
    ))
    .unwrap();
    let cmk_info2 = cmk2.to_info(1).unwrap();
    println!("cmk_info2:\n{}", cmk_info2);

    let digest = [0u8; ring::digest::SHA256_OUTPUT_LEN];
    match ab!(cmk.sign_digest(&digest)) {
        Ok(sig) => {
            log::info!(
                "successfully signed with signature output {} bytes",
                sig.as_ref().len()
            );
        }
        Err(e) => {
            log::warn!("failed to sign, error: {:?}", e);
        }
    }

    thread::sleep(time::Duration::from_secs(5));
    ab!(cmk.delete(7)).unwrap();

    // error should be ignored if it's already scheduled for delete
    thread::sleep(time::Duration::from_secs(5));
    ab!(cmk.delete(7)).unwrap();
}
