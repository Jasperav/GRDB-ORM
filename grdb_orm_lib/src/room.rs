#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Room {
    pub imports: Vec<String>,
    pub skip_type_converters: Vec<String>,
    pub convert_with_gson_type_converters: Vec<String>,
    pub unique_indexes: Vec<String>,
}