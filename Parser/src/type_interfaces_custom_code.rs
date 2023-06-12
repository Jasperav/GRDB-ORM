use grdb_orm_lib::room::TypeInterfacesCustomCode;
use grdb_orm_lib::serde::Deserialize;
use grdb_orm_lib::toml::{Deserializer, Value};

read!(Vec<TypeInterfacesCustomCode>);

fn transform(content: &str) -> Vec<TypeInterfacesCustomCode> {
    grdb_orm_lib::toml::from_str(content).unwrap()
}
