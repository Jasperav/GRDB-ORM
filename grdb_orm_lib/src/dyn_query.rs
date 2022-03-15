/// The configuration of a dynamic query
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DynamicQuery {
    pub parameter_types: Vec<(String, String, String)>,
    pub extension: String,
    pub func_name: String,
    pub return_types: Vec<String>,
    pub return_types_is_array: bool,
    pub query: String,
}

impl DynamicQuery {
    pub fn to_toml(&self) -> String {
        toml::ser::to_string(&self).unwrap()
    }
}
