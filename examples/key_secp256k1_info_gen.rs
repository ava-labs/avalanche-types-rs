use std::env::args;

/// cargo run --example key_secp256k1_info_gen -- 9999 /tmp/key.json
fn main() {
    // ref. https://github.com/env-logger-rs/env_logger/issues/47
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info"),
    );

    let network_id = args().nth(1).expect("no network ID given");
    let network_id = network_id.parse::<u32>().unwrap();

    let file_path = args().nth(2).expect("no file path given");

    let key = avalanche_types::key::secp256k1::private_key::Key::generate()
        .expect("unexpected key generate failure");
    let info = key.to_info(network_id).expect("failed to_info");
    print!("{}", info);

    info.sync(file_path).unwrap();
}
