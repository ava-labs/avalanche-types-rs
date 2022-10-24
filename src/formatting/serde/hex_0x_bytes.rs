use serde::{Deserialize, Deserializer, Serializer};
use serde_with::{formats, DeserializeAs, SerializeAs};

/// ref. "serde_with::hex::Hex"
pub struct HexBytes<FORMAT: formats::Format = formats::Lowercase>(std::marker::PhantomData<FORMAT>);

impl<T> SerializeAs<T> for HexBytes<formats::Lowercase>
where
    T: AsRef<[u8]>,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = hex::encode(source);
        serializer.serialize_str(&format!("0x{}", s))
    }
}

impl<T> SerializeAs<T> for HexBytes<formats::Uppercase>
where
    T: AsRef<[u8]>,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&hex::encode_upper(source))
    }
}

impl<'de, T, FORMAT> DeserializeAs<'de, T> for HexBytes<FORMAT>
where
    T: TryFrom<Vec<u8>>,
    FORMAT: formats::Format,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        <std::borrow::Cow<'de, str> as Deserialize<'de>>::deserialize(deserializer)
            .and_then(|s| {
                hex::decode(&*s.trim_start_matches("0x")).map_err(serde::de::Error::custom)
            })
            .and_then(|vec: Vec<u8>| {
                let length = vec.len();
                vec.try_into().map_err(|_e: T::Error| {
                    serde::de::Error::custom(format_args!(
                        "Can't convert a Byte Vector of length {} to the output type.",
                        length
                    ))
                })
            })
    }
}

/// RUST_LOG=debug cargo test --package avalanche-types --lib -- formatting::serde::hex::test_custom_de_serializer --exact --show-output
#[test]
fn test_custom_de_serializer() {
    use serde::Serialize;
    use serde_with::serde_as;

    #[serde_as]
    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
    struct Credential {
        #[serde_as(as = "Vec<HexBytes>")]
        signatures: Vec<Vec<u8>>,
    }

    let d = Credential {
        signatures: vec![vec![123]],
    };

    let yaml_encoded = serde_yaml::to_string(&d).unwrap();
    println!("yaml_encoded:\n{}", yaml_encoded);
    let yaml_decoded = serde_yaml::from_str(&yaml_encoded).unwrap();
    assert_eq!(d, yaml_decoded);

    let json_encoded = serde_json::to_string(&d).unwrap();
    println!("json_encoded:\n{}", json_encoded);
    let json_decoded = serde_json::from_str(&json_encoded).unwrap();
    assert_eq!(d, json_decoded);

    let json_decoded_2: Credential = serde_json::from_str(
        "

{
\"signatures\":[\"0x7b\"]
}

",
    )
    .unwrap();
    assert_eq!(d, json_decoded_2);
}
