use primitive_types::U256;

pub const NANO_AVAX: u64 = 1;
pub const MICRO_AVAX: u64 = 1000 * NANO_AVAX;
pub const MILLI_AVAX: u64 = 1000 * MICRO_AVAX;

/// On the X-Chain, one AVAX is 10^9 units.
/// On the P-Chain, one AVAX is 10^9 units.
/// ref. <https://snowtrace.io/unitconverter>
pub const AVAX: u64 = 1000 * MILLI_AVAX;

pub const KILO_AVAX: u64 = 1000 * AVAX;
pub const MEGA_AVAX: u64 = 1000 * KILO_AVAX;

/// On the C-Chain, one AVAX is 10^18 units.
/// ref. <https://snowtrace.io/unitconverter>
pub const AVAX_EVM_CHAIN: u64 = 1000 * MEGA_AVAX;

/// Converts the nano AVAX to AVAX unit for X and P chain.
pub fn convert_navax_for_x_and_p(n: u64) -> u64 {
    n / AVAX
}

/// Converts the nano AVAX to AVAX unit for C-chain and other EVM-based subnets.
pub fn convert_navax_for_evm(n: u64) -> u64 {
    n / AVAX_EVM_CHAIN
}

/// Converts the nano AVAX to AVAX/i64 unit for C-chain and other EVM-based subnets.
///
/// On the C-Chain, one AVAX is 10^18 units.
/// ref. <https://snowtrace.io/unitconverter>
///
/// If it overflows, it resets to i64::MAX.
pub fn cast_navax_to_avax_i64(navax: U256) -> i64 {
    // ref. "ethers::utils::Units::Ether"
    let avax_unit = U256::from(10).checked_pow(U256::from(18)).unwrap();
    let avaxs = navax.checked_div(avax_unit).unwrap();
    if avaxs >= U256::from(u64::MAX) {
        i64::MAX
    } else {
        let converted = avaxs.as_u64();
        if converted >= i64::MAX as u64 {
            i64::MAX
        } else {
            converted as i64
        }
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- units::test_cast_navax_to_avax_i64 --exact --show-output
#[test]
fn test_cast_navax_to_avax_i64() {
    assert_eq!(cast_navax_to_avax_i64(U256::max_value()), i64::MAX);
    assert_eq!(cast_navax_to_avax_i64(U256::from(i64::MAX)), 9);
    assert_eq!(cast_navax_to_avax_i64(U256::from(100)), 0);
}

/// Converts the AVAX unit to nano-AVAX.
/// On the C-Chain, one AVAX is 10^18 units.
/// ref. <https://snowtrace.io/unitconverter>
/// If it overflows, it resets to U256::MAX.
pub fn cast_avax_to_navax(avax: U256) -> U256 {
    // ref. "ethers::utils::Units::Ether"
    let avax_unit = U256::from(10).checked_pow(U256::from(18)).unwrap();
    if let Some(navaxs) = avax.checked_mul(avax_unit) {
        navaxs
    } else {
        U256::max_value()
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- units::test_cast_avax_to_navax --exact --show-output
#[test]
fn test_cast_avax_to_navax() {
    assert_eq!(cast_avax_to_navax(U256::max_value()), U256::max_value());
    assert_eq!(
        cast_avax_to_navax(U256::from(1)),
        U256::from_dec_str("1000000000000000000").unwrap()
    );
    assert_eq!(
        cast_avax_to_navax(U256::from(10)),
        U256::from_dec_str("10000000000000000000").unwrap()
    );
    assert_eq!(
        cast_avax_to_navax(U256::from(500)),
        U256::from_dec_str("500000000000000000000").unwrap()
    );
}
