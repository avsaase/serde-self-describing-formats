use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MyDataType {
    pub value: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maybe_string: Option<String>,
    pub an_enum: AnEnum,
    #[serde(flatten)]
    pub nested_data: AnotherDataType,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AnotherDataType {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AnEnum {
    Variant1,
    Variant2(Variant2Data),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Variant2Data {
    pub numbers: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use crate::{formats::*, test_format_impl};

    use super::*;

    test_format_impl!(
        [
            SerdeJson,
            SerdeMsgpack,
            RmpSerde,
            RmpSerdeNamed,
            MessagePackSerde,
            Ciborium,
            SerdeCbor,
            MiniCborSerde,
            Pot
        ],
        roundtrip
    );

    fn roundtrip<F: SerdeFormat>() {
        let data = MyDataType {
            value: 42,
            maybe_string: None,
            nested_data: AnotherDataType {
                name: "Nested".to_string(),
                values: vec![1.0, 2.5, 3.14],
            },
            an_enum: AnEnum::Variant2(Variant2Data {
                numbers: vec![1.0, 5656.23, 4234.0, 45435.29843],
            }),
        };
        let serialized = F::serialize(&data);
        let deserialized: MyDataType = F::deserialize(serialized);
        assert_eq!(data, deserialized);
    }
}
