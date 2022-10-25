use serde::{self, Deserialize, Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub fn serialize<S>(x: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("0x{:x}", *x))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let s = s.trim_start_matches("0x");

    u64::from_str_radix(&s, 16).map_err(serde::de::Error::custom)
}

pub struct HexU64(u64);

impl SerializeAs<u64> for HexU64 {
    fn serialize_as<S>(x: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{:x}", *x))
    }
}

impl<'de> DeserializeAs<'de, u64> for HexU64 {
    fn deserialize_as<D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.trim_start_matches("0x");

        u64::from_str_radix(&s, 16).map_err(serde::de::Error::custom)
    }
}
