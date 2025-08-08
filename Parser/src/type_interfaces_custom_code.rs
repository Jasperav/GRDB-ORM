use crate::room::TypeInterfacesCustomCode;
use serde::Deserialize;
use toml::Value;

read!(Vec<TypeInterfacesCustomCode>);

fn transform(content: &str) -> Vec<TypeInterfacesCustomCode> {
    let value: Value = content.parse().unwrap();
    let tables = value.as_table().unwrap();
    let mut types: Vec<TypeInterfacesCustomCode> = vec![];

    for (_, value) in tables {
        let deserialized = TypeInterfacesCustomCode::deserialize(toml::de::ValueDeserializer::new(
            &value.to_string(),
        ))
        .unwrap();

        assert!(!types.iter().any(|t| t.ty == deserialized.ty));

        types.push(deserialized);
    }

    types
}
