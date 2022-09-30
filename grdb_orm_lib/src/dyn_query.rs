/// The configuration of a dynamic query
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DynamicQuery {
    pub parameter_types: Vec<(String, String, String)>,
    pub extension: String,
    pub func_name: String,
    pub return_types: Vec<String>,
    pub return_types_is_array: bool,
    pub query: String,
    /// Func name of the other query
    pub map_to_different_type: Option<String>,
    /// Set this only to true when using the index optimizer and this query should ignore
    /// a b_tree plan
    pub bypass_b_tree_index_optimizer: bool,
}

impl DynamicQuery {
    pub fn to_toml(&self) -> String {
        toml::ser::to_string(&self).unwrap()
    }
}
