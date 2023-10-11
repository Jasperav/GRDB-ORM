#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Room {
    pub imports: Vec<String>,
    pub disallow_default_dao_methods: bool,
    pub skip_type_converters: Vec<String>,
    pub convert_with_gson_type_converters: Vec<String>,
    pub unique_indexes: Vec<String>,
    pub gson_type_adapters: Vec<Vec<String>>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TypeInterfacesCustomCode {
    pub ty: String,
    pub interfaces: Vec<String>,
    pub custom_code: String,
}
