use serde::{Deserialize, Serialize};

pub trait SerdeFormat {
    fn serialize<T: Serialize>(value: &T) -> Vec<u8>;
    fn deserialize<T: for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T;

    fn serialized_size<T: Serialize>(value: &T) -> usize {
        Self::serialize(value).len()
    }
}

pub struct SerdeJson;

impl SerdeFormat for SerdeJson {
    fn serialize<T: Serialize>(value: &T) -> Vec<u8> {
        serde_json::to_vec(value).unwrap()
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
        serde_json::from_slice(&bytes).unwrap()
    }
}

pub struct SerdeMsgpack;

impl SerdeFormat for SerdeMsgpack {
    fn serialize<T: Serialize>(value: &T) -> Vec<u8> {
        rmp_serde::to_vec(value).unwrap()
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
        rmp_serde::from_slice(&bytes).unwrap()
    }
}

pub struct RmpSerde;

impl SerdeFormat for RmpSerde {
    fn serialize<T: Serialize>(value: &T) -> Vec<u8> {
        rmp_serde::to_vec(value).unwrap()
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
        rmp_serde::from_slice(&bytes).unwrap()
    }
}

pub struct RmpSerdeNamed;

impl SerdeFormat for RmpSerdeNamed {
    fn serialize<T: Serialize>(value: &T) -> Vec<u8> {
        rmp_serde::to_vec_named(value).unwrap()
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
        rmp_serde::from_slice(&bytes).unwrap()
    }
}

pub struct MessagePackSerde;

impl SerdeFormat for MessagePackSerde {
    fn serialize<T: Serialize>(value: &T) -> Vec<u8> {
        messagepack_serde::to_vec(value).unwrap()
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
        messagepack_serde::from_slice(&bytes).unwrap()
    }
}

pub struct Ciborium;

impl SerdeFormat for Ciborium {
    fn serialize<T: Serialize>(value: &T) -> Vec<u8> {
        let mut buf = Vec::new();
        ciborium::ser::into_writer(value, &mut buf).unwrap();
        buf
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
        ciborium::de::from_reader(&bytes[..]).unwrap()
    }
}

pub struct SerdeCbor;

impl SerdeFormat for SerdeCbor {
    fn serialize<T: Serialize>(value: &T) -> Vec<u8> {
        serde_cbor::to_vec(value).unwrap()
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
        serde_cbor::from_slice(&bytes).unwrap()
    }
}

pub struct MiniCborSerde;

impl SerdeFormat for MiniCborSerde {
    fn serialize<T: Serialize>(value: &T) -> Vec<u8> {
        minicbor_serde::to_vec(value).unwrap()
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
        minicbor_serde::from_slice(&bytes).unwrap()
    }
}

pub struct Pot;

impl SerdeFormat for Pot {
    fn serialize<T: Serialize>(value: &T) -> Vec<u8> {
        pot::to_vec(value).unwrap()
    }

    fn deserialize<T: for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
        pot::from_slice(&bytes).unwrap()
    }
}
