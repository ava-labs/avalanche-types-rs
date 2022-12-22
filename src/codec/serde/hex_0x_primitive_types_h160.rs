use std::str::FromStr;

use serde::{self, Deserialize, Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub fn serialize<S>(x: &primitive_types::H160, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("0x{:x}", *x))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<primitive_types::H160, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let s = s.trim_start_matches("0x");

    primitive_types::H160::from_str(s).map_err(serde::de::Error::custom)
}

pub struct Hex0xH160(primitive_types::H160);

impl SerializeAs<primitive_types::H160> for Hex0xH160 {
    fn serialize_as<S>(x: &primitive_types::H160, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{:x}", *x))
    }
}

impl<'de> DeserializeAs<'de, primitive_types::H160> for Hex0xH160 {
    fn deserialize_as<D>(deserializer: D) -> Result<primitive_types::H160, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.trim_start_matches("0x");

        primitive_types::H160::from_str(s).map_err(serde::de::Error::custom)
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- codec::serde::hex_0x_primitive_types_h160::test_custom_de_serializer --exact --show-output
#[test]
fn test_custom_de_serializer() {
    use serde::Serialize;
    use serde_with::serde_as;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    struct Data {
        #[serde_as(as = "Vec<Hex0xH160>")]
        data: Vec<primitive_types::H160>,
    }

    let d = Data {
        data: vec![
            primitive_types::H160::from_str("0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC").unwrap(),
            primitive_types::H160::from_str("0xeF14C4Ee608e5C79BcE97e3113401a360df809FB").unwrap(),
        ],
    };

    let yaml_encoded = serde_yaml::to_string(&d).unwrap();
    println!("yaml_encoded:\n{}", yaml_encoded);
    let yaml_decoded = serde_yaml::from_str(&yaml_encoded).unwrap();
    assert_eq!(d, yaml_decoded);

    let json_encoded = serde_json::to_string(&d).unwrap();
    println!("json_encoded:\n{}", json_encoded);
    let json_decoded = serde_json::from_str(&json_encoded).unwrap();
    assert_eq!(d, json_decoded);

    let json_decoded_2: Data = serde_json::from_str(
        "

{
\"data\":[\"0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC\", \"0xeF14C4Ee608e5C79BcE97e3113401a360df809FB\"]
}

",
    )
    .unwrap();
    assert_eq!(d, json_decoded_2);
}
