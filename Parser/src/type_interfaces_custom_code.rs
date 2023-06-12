use std::collections::HashSet;
use grdb_orm_lib::room::TypeInterfacesCustomCode;
use grdb_orm_lib::serde::Deserialize;
use grdb_orm_lib::toml::{Deserializer, Value};

read!(Vec<TypeInterfacesCustomCode>);

fn transform(content: &str) -> Vec<TypeInterfacesCustomCode> {
    let value: Value = content.parse().unwrap();
    let tables = value.as_table().unwrap();
    let mut types = vec![];

    for (_, value) in tables {
        let deserialized = TypeInterfacesCustomCode::deserialize(value).unwrap();

        assert!(!types.iter().any(|t| t.ty == deserialized.ty));

        types.push(deserialized);
    }

    types
}
