pub const NANO_AVAX: u64 = 1;
pub const MICRO_AVAX: u64 = 1000 * NANO_AVAX;
pub const MILLI_AVAX: u64 = 1000 * MICRO_AVAX;

/// On the X-Chain, one AVAX is 10^9 units.
/// On the P-Chain, one AVAX is 10^9 units.
/// ref. https://snowtrace.io/unitconverter
pub const AVAX: u64 = 1000 * MILLI_AVAX;

pub const KILO_AVAX: u64 = 1000 * AVAX;
pub const MEGA_AVAX: u64 = 1000 * KILO_AVAX;

/// On the C-Chain, one AVAX is 10^18 units.
/// ref. https://snowtrace.io/unitconverter
pub const AVAX_EVM_CHAIN: u64 = 1000 * MEGA_AVAX;

/// Converts the nano AVAX to AVAX unit for X and P chain.
pub fn convert_navax_for_x_and_p(n: u64) -> u64 {
    n / AVAX
}

/// Converts the nano AVAX to AVAX unit for C-chain and other EVM-based subnets.
pub fn convert_navax_for_evm(n: u64) -> u64 {
    n / AVAX_EVM_CHAIN
}
