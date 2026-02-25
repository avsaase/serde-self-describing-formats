use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WithoutNewField {
    value: i32,
    anoter_field: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WithOptionalNewField {
    value: i32,
    new_field: Option<String>,
    anoter_field: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct WithRequiredNewField {
    value: i32,
    new_field: String,
    anoter_field: String,
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
        add_field
    );

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
        remove_field
    );

    fn add_field<F: SerdeFormat>() {
        let initial = WithoutNewField {
            value: 42,
            anoter_field: "test".to_string(),
        };
        let intermediate_some = WithOptionalNewField {
            value: initial.value,
            new_field: Some("new value".to_string()),
            anoter_field: initial.anoter_field.clone(),
        };
        let final_format = WithRequiredNewField {
            value: initial.value,
            new_field: intermediate_some.new_field.clone().unwrap(),
            anoter_field: initial.anoter_field.clone(),
        };

        // Initial -> Intermediate
        let serialized_initial = F::serialize(&initial);
        let deserialized_intermediate: WithOptionalNewField = F::deserialize(serialized_initial);
        assert_eq!(initial.value, deserialized_intermediate.value);
        assert_eq!(initial.anoter_field, deserialized_intermediate.anoter_field);
        assert_eq!(deserialized_intermediate.new_field, None);

        // Intermediate -> Initial
        let serialized_intermediate = F::serialize(&intermediate_some);
        let deserialized_initial: WithoutNewField = F::deserialize(serialized_intermediate);
        assert_eq!(initial.value, deserialized_initial.value);
        assert_eq!(initial.anoter_field, deserialized_initial.anoter_field);

        // Intermediate -> Final
        let serialized_intermediate = F::serialize(&intermediate_some);
        let deserialized_final: WithRequiredNewField = F::deserialize(serialized_intermediate);
        assert_eq!(final_format.value, deserialized_final.value);
        assert_eq!(final_format.anoter_field, deserialized_final.anoter_field);
        assert_eq!(final_format.new_field, deserialized_final.new_field);

        // Final -> Intermediate
        let serialized_final = F::serialize(&final_format);
        let deserialized_intermediate: WithOptionalNewField = F::deserialize(serialized_final);
        assert_eq!(final_format.value, deserialized_intermediate.value);
        assert_eq!(
            final_format.anoter_field,
            deserialized_intermediate.anoter_field
        );
        assert_eq!(
            deserialized_intermediate.new_field,
            Some(final_format.new_field.clone())
        );
    }

    fn remove_field<F: SerdeFormat>() {
        let initial = WithRequiredNewField {
            value: 42,
            new_field: "new value".to_string(),
            anoter_field: "test".to_string(),
        };
        let intermediate_some = WithOptionalNewField {
            value: initial.value,
            new_field: Some(initial.new_field.clone()),
            anoter_field: initial.anoter_field.clone(),
        };
        let intermediate_none = WithOptionalNewField {
            value: initial.value,
            new_field: None,
            anoter_field: initial.anoter_field.clone(),
        };
        let final_format = WithoutNewField {
            value: initial.value,
            anoter_field: initial.anoter_field.clone(),
        };

        // Initial -> Intermediate (Some)
        let serialized_initial = F::serialize(&initial);
        let deserialized_intermediate: WithOptionalNewField = F::deserialize(serialized_initial);
        assert_eq!(initial.value, deserialized_intermediate.value);
        assert_eq!(initial.anoter_field, deserialized_intermediate.anoter_field);
        assert_eq!(
            deserialized_intermediate.new_field,
            Some(initial.new_field.clone())
        );

        // Intermediate (Some) -> Initial
        let serialized_intermediate = F::serialize(&intermediate_some);
        let deserialized_initial: WithRequiredNewField = F::deserialize(serialized_intermediate);
        assert_eq!(initial.value, deserialized_initial.value);
        assert_eq!(initial.anoter_field, deserialized_initial.anoter_field);
        assert_eq!(initial.new_field, deserialized_initial.new_field);

        // Intermediate -> Final
        let serialized_intermediate = F::serialize(&intermediate_none);
        let deserialized_final: WithoutNewField = F::deserialize(serialized_intermediate);
        assert_eq!(final_format.value, deserialized_final.value);
        assert_eq!(final_format.anoter_field, deserialized_final.anoter_field);

        // Final -> Intermediate
        let serialized_final = F::serialize(&final_format);
        let deserialized_intermediate: WithOptionalNewField = F::deserialize(serialized_final);
        assert_eq!(final_format.value, deserialized_intermediate.value);
        assert_eq!(
            final_format.anoter_field,
            deserialized_intermediate.anoter_field
        );
        assert_eq!(deserialized_intermediate.new_field, None);
    }
}
