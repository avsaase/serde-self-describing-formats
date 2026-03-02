#[cfg(test)]
mod test {
    use crate::{
        formats::*,
        roundtrip::{AnEnum, AnotherDataType, MyDataType, Variant2Data},
        test_format_impl,
    };

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
        serialized_size
    );

    fn serialized_size<F: SerdeFormat>() {
        let data = MyDataType {
            value: 42,
            maybe_string: Some("Hello".to_string()),
            nested_data: AnotherDataType {
                name: "Nested".to_string(),
                values: vec![1.0, 2.5, 3.14, 123456.789],
            },
            an_enum: AnEnum::Variant2(Variant2Data {
                numbers: vec![0.1, 0.2, 0.3, 0.4, 0.5],
            }),
        };

        let size = F::serialized_size(&data);
        println!(
            "Serialized size for {}: {} bytes",
            std::any::type_name::<F>(),
            size
        );
    }
}
