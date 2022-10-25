use num_bigint::BigInt;
use serde::{self, Deserialize, Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub fn serialize<S>(x: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("0x{}", big_num_manager::big_int_to_lower_hex(x)))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    big_num_manager::from_hex_to_big_int(&s).map_err(serde::de::Error::custom)
}

pub struct HexBigInt(BigInt);

impl SerializeAs<BigInt> for HexBigInt {
    fn serialize_as<S>(x: &BigInt, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", big_num_manager::big_int_to_lower_hex(x)))
    }
}

impl<'de> DeserializeAs<'de, BigInt> for HexBigInt {
    fn deserialize_as<D>(deserializer: D) -> Result<BigInt, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        big_num_manager::from_hex_to_big_int(&s).map_err(serde::de::Error::custom)
    }
}
