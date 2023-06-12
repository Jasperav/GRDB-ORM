use std::collections::HashSet;
use grdb_orm_lib::room::TypeInterfacesCustomCode;
use grdb_orm_lib::serde::Deserialize;
use grdb_orm_lib::toml::{Deserializer, Value};

read!(Vec<TypeInterfacesCustomCode>);

fn transform(content: &str) -> Vec<TypeInterfacesCustomCode> {
    let deserialize: Vec<TypeInterfacesCustomCode> = grdb_orm_lib::toml::from_str(content).unwrap();
    let uniques = deserialize.iter().map(|d| &d.ty).collect::<HashSet<_>>();

    assert_eq!(uniques.len(), deserialize.len());

    deserialize
}
