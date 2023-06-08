use grdb_orm_lib::room::Room;
use grdb_orm_lib::serde::Deserialize;
use grdb_orm_lib::toml::Value;

read!(Room);

/// Transforms a TOML file to [Room]
fn transform(content: &str) -> Room {
    let value: Value = content.parse().unwrap();

    Room::deserialize(grdb_orm_lib::toml::de::ValueDeserializer::new(
        &value.to_string(),
    ))
    .unwrap()
}
